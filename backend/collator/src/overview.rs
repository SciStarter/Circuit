use std::collections::BTreeMap;

use anyhow::{anyhow, Error};
use chrono::{DateTime, FixedOffset, Utc};
use common::{
    model::analytics::{
        DetailedEngagementDataChart, DetailedEngagementDataChartWithPoint, EngagementDataChart,
        EngagementType, Overview, OverviewCrossover, OverviewCrossoverData,
        OverviewCrossoverDataChart, OverviewCrossoverDataChartSegment, OverviewDemographics,
        OverviewEngagement, OverviewEngagementData, OverviewEngagementDataSearch,
        OverviewEngagementDataStats, OverviewStates, OverviewStatesData, OverviewTechnology,
        OverviewTechnologyData, OverviewTraffic, OverviewTrafficData, PieChart, PieData,
        RegionEngagement, RelativeTimePeriod, StateEngagement, Status, TrafficChart,
    },
    Database, ToFixedOffset,
};
use futures_util::TryStreamExt;
use google_analyticsdata1_beta::api::{Dimension, Filter, FilterExpression, Metric, StringFilter};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{ga4_bigquery as ga4, CommonState};

pub async fn cache(
    db: &Database,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    // Here's where we fetch any data that are still needed for the
    // complete organization, but no need to fetch redundant
    // information that was already cached by the opportunity or
    // organization caching stages.

    let mut unique_visitors = 0;
    let mut opportunity_unique = 0;

    if !ga4::is_overview_cached(db, begin, end).await {
        for row in ga4::run_report(
            begin,
            end,
            None,
            vec![],
            vec![Metric {
                name: Some("sessions".into()),
                ..Default::default()
            }],
        )
        .await?
        {
            let sessions = row.get_int("sessions");
            unique_visitors += sessions;
            if let Ok(_) = row.get_uuid("customEvent:entity_uid") {
                opportunity_unique += sessions;
            }
        }

        sqlx::query!(
            r#"
INSERT INTO c_analytics_overview_cache (
  "temporary",
  "begin",
  "end",
  "unique_visitors",
  "opportunity_unique",

  "shares",
  "calendar_adds",
  "likes",
  "saves",
  "didits",
  "opportunity_views",
  "opportunity_exits",
  "accounts"
)
VALUES (
  $1, $2, $3, $4, $5,

  (SELECT COALESCE(COUNT(*), 0) FROM c_log WHERE "action" LIKE 'shared:%' AND "when" >= $2 AND "when" < $3),
  (SELECT COALESCE(COUNT(*), 0) FROM c_log WHERE "action" LIKE 'calendar:%' AND "when" >= $2 AND "when" < $3),
  (SELECT COALESCE(COUNT(*), 0) FROM c_opportunity_like WHERE "when" >= $2 AND "when" < $3),
  (SELECT COALESCE(COUNT(*), 0) FROM c_involvement WHERE ("exterior"->'mode')::integer = 20 AND "updated" >= $2 AND "updated" < $3),
  (SELECT COALESCE(COUNT(*), 0) FROM c_involvement WHERE ("exterior"->'mode')::integer >= 30 AND "updated" >= $2 AND "updated" < $3),
  (SELECT COALESCE(SUM("views")::bigint, 0) FROM c_analytics_cache WHERE "begin" = $2 AND "end" = $3),
  (SELECT COALESCE(COUNT(*), 0) FROM c_log WHERE "action" = 'external' AND "when" >= $2 AND "when" < $3),
  (SELECT COALESCE(COUNT(*), 0) FROM c_person WHERE "created" >= $2 AND "created" < $3)
)
    "#,
            temporary,
            begin,
            end,
            unique_visitors,
            opportunity_unique,
        )
        .execute(db)
        .await?;
    }

    if !ga4::is_search_terms_cached(db, begin, end).await {
        let mut search_counts = BTreeMap::new();

        for row in ga4::run_report(
            begin,
            end,
            Some(FilterExpression {
                filter: Some(Filter {
                    field_name: Some("pagePathPlusQueryString".into()),
                    string_filter: Some(StringFilter {
                        match_type: Some("BEGINS_WITH".into()),
                        value: Some("/find".into()),
                        case_sensitive: Some(false),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            vec![Dimension {
                name: Some("pagePathPlusQueryString".into()),
                ..Default::default()
            }],
            vec![Metric {
                name: Some("screenPageViews".into()),
                ..Default::default()
            }],
        )
        .await?
        {
            if let Some((_, qs)) = row.get_string("pagePathPlusQueryString").split_once('?') {
                let query: BTreeMap<String, String> = serde_qs::from_str(qs).unwrap_or_default();

                if let Some(text) = query.get("text") {
                    let text = text.trim().to_lowercase();

                    if text.is_empty() {
                        continue;
                    }

                    let views = row.get_int("screenPageViews");

                    search_counts
                        .entry(text)
                        .and_modify(|v| *v += views)
                        .or_insert(views);
                }
            }
        }

        for (term, times) in search_counts {
            sqlx::query!(
                r#"
INSERT INTO c_analytics_search_term_cache (
  "temporary",
  "begin",
  "end",
  "term",
  "times"
)
VALUES (
  $1, $2, $3, $4, $5
)
"#,
                temporary,
                begin,
                end,
                term,
                times
            )
            .execute(db)
            .await?;
        }
    }

    Ok(())
}

pub async fn collect(db: &Database, state: &CommonState) -> Result<Overview, Error> {
    let engagement = OverviewEngagementData {
        begin: state.begin,
        end: state.end,
        search_max: sqlx::query_scalar!(
            r#"
SELECT COALESCE(MAX("times"), 0) AS "times!"
FROM c_analytics_search_term_cache
WHERE "begin" = $1 AND "end" = $2
"#,
            state.begin,
            state.end,
        )
        .fetch_one(db)
        .await?
        .try_into()
        .unwrap_or(0),
        stats: sqlx::query!(
            r#"
SELECT
  "unique_visitors" AS "unique_visitors!",
  "shares" AS "shares!",
  "calendar_adds" AS "calendar_adds!",
  "likes" AS "likes!",
  "saves" AS "saves!",
  "didits" AS "didits!",
  "opportunity_views" AS "opportunity_views!",
  "opportunity_unique" AS "opportunity_unique!",
  "opportunity_exits" AS "opportunity_exits!",
  "accounts" AS "accounts!"
FROM c_analytics_overview_cache
WHERE
  "begin" = $1 AND
  "end" = $2
LIMIT 1
"#,
            state.begin,
            state.end,
        )
        .map(|row| OverviewEngagementDataStats {
            unique_visitors: row.unique_visitors.try_into().unwrap_or(0),
            accounts: row.accounts.try_into().unwrap_or(0),
            opportunity_views: row.opportunity_views.try_into().unwrap_or(0),
            opportunity_unique: row.opportunity_unique.try_into().unwrap_or(0),
            opportunity_exits: row.opportunity_exits.try_into().unwrap_or(0),
            didits: row.didits.try_into().unwrap_or(0),
            saves: row.saves.try_into().unwrap_or(0),
            likes: row.likes.try_into().unwrap_or(0),
            shares: row.shares.try_into().unwrap_or(0),
            calendar_adds: row.calendar_adds.try_into().unwrap_or(0),
        })
        .fetch_one(db)
        .await?,
        searches: sqlx::query!(
            r#"
SELECT
  "term" AS "term!",
  "times" AS "times!"
FROM c_analytics_search_term_cache
WHERE
  "begin" = $1 AND
  "end" = $2
ORDER BY "times" DESC
LIMIT 30
"#,
            state.begin,
            state.end
        )
        .map(|row| OverviewEngagementDataSearch {
            phrase: row.term,
            searches: row.times.try_into().unwrap_or(0),
        })
        .fetch_all(db)
        .await?
        .into_iter()
        .collect(),
    };

    let demographics: OverviewDemographics = serde_json::from_value(
        sqlx::query_scalar!(
            r#"
SELECT "data" AS "data!" FROM c_demographics WHERE "about" = $1
"#,
            Uuid::nil()
        )
        .fetch_optional(db)
        .await?
        .expect("No demographics data uploaded"),
    )?;

    let mut states_max = DetailedEngagementDataChart::default();
    let mut states = BTreeMap::new();

    let mut states_rows = sqlx::query!(
        r#"
SELECT
  "region" AS "state!: String",
  SUM("total_users")::bigint AS "unique_users: i64",
  SUM("new_users")::bigint AS "new_users: i64",
  COALESCE(SUM("total_users")::bigint, 0) - COALESCE(SUM("new_users")::bigint, 0) AS "returning_users: i64",
  SUM("views")::bigint AS "total_pageviews: i64",
  SUM("sessions")::bigint AS "unique_pageviews: i64",
  AVG("engagement_duration") AS "average_time: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND c_opportunity_by_uid_is_status("opportunity", $3)
GROUP BY "region"
"#,
        state.begin,
        state.end,
        state.status.discriminate()
    )
    .fetch(db);

    while let Some(state_row) = states_rows.try_next().await? {
        let state_name = state_row.state;

        let state_row = DetailedEngagementDataChart {
            date: None,
            unique_users: state_row.unique_users.unwrap_or(0).try_into().unwrap_or(0),
            new_users: state_row.new_users.unwrap_or(0).try_into().unwrap_or(0),
            returning_users: state_row
                .returning_users
                .unwrap_or(0)
                .try_into()
                .unwrap_or(0),
            total_pageviews: state_row
                .total_pageviews
                .unwrap_or(0)
                .try_into()
                .unwrap_or(0),
            unique_pageviews: state_row
                .unique_pageviews
                .unwrap_or(0)
                .try_into()
                .unwrap_or(0),
            average_time: state_row
                .average_time
                .unwrap_or(0.0)
                .try_into()
                .unwrap_or(0.0),
        };

        if state_row.unique_users > states_max.unique_users {
            states_max.unique_users = state_row.unique_users;
        }

        if state_row.new_users > states_max.new_users {
            states_max.new_users = state_row.new_users;
        }

        if state_row.returning_users > states_max.returning_users {
            states_max.returning_users = state_row.returning_users;
        }

        if state_row.total_pageviews > states_max.total_pageviews {
            states_max.total_pageviews = state_row.total_pageviews;
        }

        if state_row.unique_pageviews > states_max.unique_pageviews {
            states_max.unique_pageviews = state_row.unique_pageviews;
        }

        if state_row.average_time > states_max.average_time {
            states_max.average_time = state_row.average_time;
        }

        let mut regions_max = DetailedEngagementDataChart::default();
        let mut regions = BTreeMap::new();

        let mut regions_rows = sqlx::query!(
            r#"
SELECT
  "city" AS "region!: String",
  SUM("total_users")::bigint AS "unique_users: i64",
  SUM("new_users")::bigint AS "new_users: i64",
  COALESCE(SUM("total_users")::bigint, 0) - COALESCE(SUM("new_users")::bigint, 0) AS "returning_users: i64",
  SUM("views")::bigint AS "total_pageviews: i64",
  SUM("sessions")::bigint AS "unique_pageviews: i64",
  AVG("engagement_duration") AS "average_time: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND "region" = $3 AND c_opportunity_by_uid_is_status("opportunity", $4)
GROUP BY "city"
"#,
            state.begin,
            state.end,
            &state_name,
            state.status.discriminate()
        )
        .fetch(db);

        while let Some(region_row) = regions_rows.try_next().await? {
            let region_name = region_row.region;

            let region_row = DetailedEngagementDataChart {
                date: None,
                unique_users: region_row.unique_users.unwrap_or(0).try_into().unwrap_or(0),
                new_users: region_row.new_users.unwrap_or(0).try_into().unwrap_or(0),
                returning_users: region_row
                    .returning_users
                    .unwrap_or(0)
                    .try_into()
                    .unwrap_or(0),
                total_pageviews: region_row
                    .total_pageviews
                    .unwrap_or(0)
                    .try_into()
                    .unwrap_or(0),
                unique_pageviews: region_row
                    .unique_pageviews
                    .unwrap_or(0)
                    .try_into()
                    .unwrap_or(0),
                average_time: region_row
                    .average_time
                    .unwrap_or(0.0)
                    .try_into()
                    .unwrap_or(0.0),
            };

            if region_row.unique_users > regions_max.unique_users {
                regions_max.unique_users = region_row.unique_users;
            }

            if region_row.new_users > regions_max.new_users {
                regions_max.new_users = region_row.new_users;
            }

            if region_row.returning_users > regions_max.returning_users {
                regions_max.returning_users = region_row.returning_users;
            }

            if region_row.total_pageviews > regions_max.total_pageviews {
                regions_max.total_pageviews = region_row.total_pageviews;
            }

            if region_row.unique_pageviews > regions_max.unique_pageviews {
                regions_max.unique_pageviews = region_row.unique_pageviews;
            }

            if region_row.average_time > regions_max.average_time {
                regions_max.average_time = region_row.average_time;
            }

            let point =
                common::geo::Query::new(format!("{}, {}", &region_name, &state_name), false)
                    .lookup_one()
                    .await
                    .map(|m| (m.geometry.longitude as f64, m.geometry.latitude as f64));

            regions.insert(
                region_name,
                DetailedEngagementDataChartWithPoint {
                    values: DetailedEngagementDataChart {
                        date: None,
                        unique_users: region_row.unique_users,
                        new_users: region_row.new_users,
                        returning_users: region_row.returning_users,
                        total_pageviews: region_row.total_pageviews,
                        unique_pageviews: region_row.unique_pageviews,
                        average_time: region_row.average_time,
                    },
                    point,
                },
            );
        }

        states.insert(
            state_name,
            StateEngagement {
                values: DetailedEngagementDataChart {
                    date: None,
                    unique_users: state_row.unique_users,
                    new_users: state_row.new_users,
                    returning_users: state_row.returning_users,
                    total_pageviews: state_row.total_pageviews,
                    unique_pageviews: state_row.unique_pageviews,
                    average_time: state_row.average_time,
                },
                regional: RegionEngagement {
                    max: regions_max,
                    regions,
                },
            },
        );
    }

    let mut tech_desktop = DetailedEngagementDataChart::default();
    let mut tech_tablet = DetailedEngagementDataChart::default();
    let mut tech_mobile = DetailedEngagementDataChart::default();

    let tech_rows = sqlx::query!(
        r#"
SELECT
  "device_category" AS "device_category!: String",
  SUM("total_users")::bigint AS "unique_users: i64",
  SUM("new_users")::bigint AS "new_users: i64",
  COALESCE(SUM("total_users")::bigint, 0) - COALESCE(SUM("new_users")::bigint, 0) AS "returning_users: i64",
  SUM("views")::bigint AS "total_pageviews: i64",
  SUM("sessions")::bigint AS "unique_pageviews: i64",
  AVG("engagement_duration") AS "average_time: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND c_opportunity_by_uid_is_status("opportunity", $3)
GROUP BY "device_category"
"#,
        state.begin,
        state.end,
        state.status.discriminate()
    )
    .fetch_all(db)
    .await?;

    for row in tech_rows {
        let chart = match row.device_category.as_ref() {
            "Desktop" => &mut tech_desktop,
            "desktop" => &mut tech_desktop,
            "Tablet" => &mut tech_tablet,
            "tablet" => &mut tech_tablet,
            "Mobile" => &mut tech_mobile,
            "mobile" => &mut tech_mobile,
            _ => {
                return Err(anyhow!(
                    "Unrecognized tech category: {}",
                    row.device_category
                ))
            }
        };

        *chart = DetailedEngagementDataChart {
            date: None,
            unique_users: row.unique_users.unwrap_or(0).try_into().unwrap_or(0),
            new_users: row.new_users.unwrap_or(0).try_into().unwrap_or(0),
            returning_users: row.returning_users.unwrap_or(0).try_into().unwrap_or(0),
            total_pageviews: row.total_pageviews.unwrap_or(0).try_into().unwrap_or(0),
            unique_pageviews: row.unique_pageviews.unwrap_or(0).try_into().unwrap_or(0),
            average_time: row.average_time.unwrap_or(0.0).try_into().unwrap_or(0.0),
        };
    }

    let tech_max = DetailedEngagementDataChart {
        date: None,
        unique_users: tech_desktop
            .unique_users
            .max(tech_tablet.unique_users)
            .max(tech_mobile.unique_users),
        new_users: tech_desktop
            .new_users
            .max(tech_tablet.new_users)
            .max(tech_mobile.new_users),
        returning_users: tech_desktop
            .returning_users
            .max(tech_tablet.returning_users)
            .max(tech_mobile.returning_users),
        total_pageviews: tech_desktop
            .total_pageviews
            .max(tech_tablet.total_pageviews)
            .max(tech_mobile.total_pageviews),
        unique_pageviews: tech_desktop
            .unique_pageviews
            .max(tech_tablet.unique_pageviews)
            .max(tech_mobile.unique_pageviews),
        average_time: tech_desktop
            .average_time
            .max(tech_tablet.average_time)
            .max(tech_mobile.average_time),
    };

    let traffic_chart = sqlx::query!(
        r#"
SELECT
  "date" AS "date!: DateTime<FixedOffset>",
  SUM("views")::bigint AS "views: i64",
  SUM("sessions")::bigint AS "unique: i64",
  SUM("new_users")::bigint AS "new: i64",
  COALESCE(SUM("sessions")::bigint, 0) - COALESCE(SUM("new_users")::bigint, 0) AS "returning: i64",
  (
    SELECT COUNT(*)
    FROM c_log INNER JOIN c_opportunity ON c_log."object" = (c_opportunity.exterior->>'uid')::uuid
    WHERE
      "action" = 'external' AND
      "when"::date = c_analytics_cache."date"::date
  ) AS "clicks: i64"
FROM c_analytics_cache
WHERE "date" >= $1 AND "date" < $2 AND c_opportunity_by_uid_is_status("opportunity", $3)
GROUP BY "date"
"#,
        state.begin,
        state.end,
        state.status.discriminate()
    )
    .map(|row| EngagementDataChart {
        date: row.date,
        views: row.views.unwrap_or(0).try_into().unwrap_or(0),
        unique: row.unique.unwrap_or(0).try_into().unwrap_or(0),
        new: row.new.unwrap_or(0).try_into().unwrap_or(0),
        returning: row.returning.unwrap_or(0).try_into().unwrap_or(0),
        clicks: row.clicks.unwrap_or(0).try_into().unwrap_or(0),
    })
    .fetch_all(db)
    .await?;

    let traffic_pie = PieChart {
        labels: vec![
            "Affiliates".into(),
            "Direct".into(),
            "Display".into(),
            "Email".into(),
            "Organic Search".into(),
            "Organic Social".into(),
            "Paid Search".into(),
            "Paid Social".into(),
            "Referral".into(),
            "Video".into(),
        ],
        datasets: vec![PieData {
            label: "Referrals by Type".into(),
            hover_offset: 4,
            background_color: vec![
                "#e7e93c".into(),
                "#387ab5".into(),
                "#cd4c24".into(),
                "#5abdda".into(),
                "#5a8cda".into(),
                "#625ada".into(),
                "#5da136".into(),
                "#365ba1".into(),
                "#5bbd08".into(),
                "#a15e36".into(),
            ],
            data: sqlx::query!(
                r#"
SELECT "session_channel_group" AS "group!", SUM("views")::bigint AS "count: i64"
FROM c_analytics_cache
WHERE "date" >= $1 AND "date" < $2 AND c_opportunity_by_uid_is_status("opportunity", $3)
GROUP BY "session_channel_group"
ORDER BY "session_channel_group"
"#,
                state.begin,
                state.end,
                state.status.discriminate()
            )
            .map(|row| row.count.unwrap_or(0).try_into().unwrap_or(0))
            .fetch_all(db)
            .await?,
        }],
    };

    let mut traffic_max = DetailedEngagementDataChart::default();

    let traffic_table = sqlx::query!(
        r#"
SELECT
  "page_referrer" AS "page_referrer!",
  "session_channel_group" AS "type_!",
  SUM("total_users")::bigint AS "unique_users: i64",
  SUM("new_users")::bigint AS "new_users: i64",
  COALESCE(SUM("total_users")::bigint, 0) - COALESCE(SUM("new_users")::bigint, 0) AS "returning_users: i64",
  SUM("views")::bigint AS "total_pageviews: i64",
  SUM("sessions")::bigint AS "unique_pageviews: i64",
  AVG("engagement_duration") AS "average_time: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND c_opportunity_by_uid_is_status("opportunity", $3)
GROUP BY "page_referrer", "session_channel_group"
"#,
        state.begin,
        state.end,
        state.status.discriminate()
    )
    .map(|row| {
        let chart = TrafficChart {
            name: row.page_referrer,
            type_: row.type_,
            values: DetailedEngagementDataChart {
                date: None,
                unique_users: row.unique_users.unwrap_or(0).try_into().unwrap_or(0),
                new_users: row.new_users.unwrap_or(0).try_into().unwrap_or(0),
                returning_users: row.returning_users.unwrap_or(0).try_into().unwrap_or(0),
                total_pageviews: row.total_pageviews.unwrap_or(0).try_into().unwrap_or(0),
                unique_pageviews: row.unique_pageviews.unwrap_or(0).try_into().unwrap_or(0),
                average_time: row.average_time.unwrap_or(0.0).try_into().unwrap_or(0.0),
            },
        };

        traffic_max.unique_users = traffic_max.unique_users.max(chart.values.unique_users);

        traffic_max.new_users = traffic_max.new_users.max(chart.values.new_users);

        traffic_max.returning_users = traffic_max
            .returning_users
            .max(chart.values.returning_users);

        traffic_max.total_pageviews = traffic_max
            .total_pageviews
            .max(chart.values.total_pageviews);

        traffic_max.unique_pageviews = traffic_max
            .unique_pageviews
            .max(chart.values.unique_pageviews);

        traffic_max.average_time = traffic_max.average_time.max(chart.values.average_time);

        chart
    })
    .fetch_all(db)
    .await?;

    let mut crossover_data: BTreeMap<(String, String), EngagementDataChart> = BTreeMap::new();
    let mut crossover_involve: BTreeMap<String, u64> = BTreeMap::new();

    let mut query = sqlx::query!(
        r#"
SELECT
  c_opportunity_by_uid_domain("prior") AS "prior_domain!",
  c_opportunity_by_uid_domain("postor") AS "postor_domain!"
FROM c_transit
WHERE
  "created" >= $1 AND
  "created" < $2 AND
  c_opportunity_by_uid_domain("prior") != c_opportunity_by_uid_domain("postor") AND
  c_opportunity_by_uid_domain("prior") != '' AND
  c_opportunity_by_uid_domain("postor") != '' AND
  c_opportunity_by_uid_is_status("prior", $3) AND
  c_opportunity_by_uid_is_status("postor", $3)
"#,
        state.begin,
        state.end,
        state.status.discriminate(),
    )
    .fetch(db);

    while let Ok(Some(row)) = query.try_next().await {
        let entry = crossover_data
            .entry(if row.prior_domain > row.postor_domain {
                (row.postor_domain.clone(), row.prior_domain.clone())
            } else {
                (row.prior_domain.clone(), row.postor_domain.clone())
            })
            .or_default();

        entry.views += 1;

        crossover_involve
            .entry(row.prior_domain)
            .and_modify(|t| *t += 1)
            .or_insert(1);

        crossover_involve
            .entry(row.postor_domain)
            .and_modify(|t| *t += 1)
            .or_insert(1);
    }

    let crossover_total = crossover_involve
        .iter()
        .fold(1.0, |total, val| total + (*val.1 as f64));

    Ok(Overview {
        updated: Utc::now().to_fixed_offset(),
        engagement: OverviewEngagement { data: engagement },
        demographics,
        states: OverviewStates {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OverviewStatesData {
                opportunity_status: state.status,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                max: states_max,
                states,
            },
        },
        technology: OverviewTechnology {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OverviewTechnologyData {
                opportunity_status: state.status,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                max: tech_max,
                mobile: tech_mobile,
                tablet: tech_tablet,
                desktop: tech_desktop,
            },
        },
        traffic: OverviewTraffic {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OverviewTrafficData {
                opportunity_status: state.status,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                columns: EngagementType::iter().collect(),
                chart: traffic_chart,
                pie: traffic_pie,
                max: traffic_max,
                table: traffic_table,
            },
        },
        crossover: OverviewCrossover {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            engagement_types: vec![EngagementType::Views],
            data: OverviewCrossoverData {
                opportunity_status: state.status,
                time_period: state.period,
                engagement_type: EngagementType::Views,
                chart: OverviewCrossoverDataChart {
                    // Note: Ordering of the domain names in
                    // crossover_data.get() matters. They must always
                    // be in aphabetical order.
                    citizen_science: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve.get("citizen_science").unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: None,
                        live_science: crossover_data
                            .get(&("citizen_science".into(), "live_science".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&("citizen_science".into(), "museum_or_science_center".into()))
                            .cloned(),
                        maker: crossover_data
                            .get(&("citizen_science".into(), "maker".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("citizen_science".into(), "policy".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&(
                                "citizen_science".into(),
                                "out_of_school_time_program".into(),
                            ))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("citizen_science".into(), "formal_education".into()))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&("citizen_science".into(), "science_communications".into()))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("citizen_science".into(), "unspecified".into()))
                            .cloned(),
                    },
                    live_science: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve.get("live_science").unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "live_science".into()))
                            .cloned(),
                        live_science: None,
                        museum_or_science_center: crossover_data
                            .get(&("live_science".into(), "museum_or_science_center".into()))
                            .cloned(),
                        maker: crossover_data
                            .get(&("live_science".into(), "maker".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("live_science".into(), "policy".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&("live_science".into(), "out_of_school_time_program".into()))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("formal_education".into(), "live_science".into()))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&("live_science".into(), "science_communications".into()))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("live_science".into(), "unspecified".into()))
                            .cloned(),
                    },
                    museum_or_science_center: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve
                                .get("museum_or_science_center")
                                .unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "museum_or_science_center".into()))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("live_science".into(), "museum_or_science_center".into()))
                            .cloned(),
                        museum_or_science_center: None,
                        maker: crossover_data
                            .get(&("maker".into(), "museum_or_science_center".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("museum_or_science_center".into(), "policy".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&(
                                "museum_or_science_center".into(),
                                "out_of_school_time_program".into(),
                            ))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("formal_education".into(), "museum_or_science_center".into()))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&(
                                "museum_or_science_center".into(),
                                "science_communications".into(),
                            ))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("museum_or_science_center".into(), "unspecified".into()))
                            .cloned(),
                    },
                    maker: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve.get("maker").unwrap_or(&0) as f64) / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "maker".into()))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("live_science".into(), "maker".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&("maker".into(), "museum_or_science_center".into()))
                            .cloned(),
                        maker: None,
                        policy: crossover_data
                            .get(&("maker".into(), "policy".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&("maker".into(), "out_of_school_time_program".into()))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("formal_education".into(), "maker".into()))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&("maker".into(), "science_communications".into()))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("maker".into(), "unspecified".into()))
                            .cloned(),
                    },
                    policy: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve.get("policy").unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "policy".into()))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("live_science".into(), "policy".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&("museum_or_science_center".into(), "policy".into()))
                            .cloned(),
                        maker: crossover_data
                            .get(&("maker".into(), "policy".into()))
                            .cloned(),
                        policy: None,
                        out_of_school_time_program: crossover_data
                            .get(&("out_of_school_time_program".into(), "policy".into()))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("formal_education".into(), "policy".into()))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&("policy".into(), "science_communications".into()))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("policy".into(), "unspecified".into()))
                            .cloned(),
                    },
                    out_of_school_time_program: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve
                                .get("out_of_school_time_program")
                                .unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&(
                                "citizen_science".into(),
                                "out_of_school_time_program".into(),
                            ))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("live_science".into(), "out_of_school_time_program".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&(
                                "museum_or_science_center".into(),
                                "out_of_school_time_program".into(),
                            ))
                            .cloned(),
                        maker: crossover_data
                            .get(&("maker".into(), "out_of_school_time_program".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("out_of_school_time_program".into(), "policy".into()))
                            .cloned(),
                        out_of_school_time_program: None,
                        formal_education: crossover_data
                            .get(&(
                                "formal_education".into(),
                                "out_of_school_time_program".into(),
                            ))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&(
                                "out_of_school_time_program".into(),
                                "science_communications".into(),
                            ))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("out_of_school_time_program".into(), "unspecified".into()))
                            .cloned(),
                    },
                    formal_education: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve.get("formal_education").unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "formal_education".into()))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("formal_education".into(), "live_science".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&("formal_education".into(), "museum_or_science_center".into()))
                            .cloned(),
                        maker: crossover_data
                            .get(&("formal_education".into(), "maker".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("formal_education".into(), "policy".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&(
                                "formal_education".into(),
                                "out_of_school_time_program".into(),
                            ))
                            .cloned(),
                        formal_education: None,
                        science_communications: crossover_data
                            .get(&("formal_education".into(), "science_communications".into()))
                            .cloned(),
                        unspecified: crossover_data
                            .get(&("formal_education".into(), "unspecified".into()))
                            .cloned(),
                    },
                    science_communications: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve
                                .get("science_communications")
                                .unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "science_communications".into()))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("live_science".into(), "science_communications".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&(
                                "museum_or_science_center".into(),
                                "science_communications".into(),
                            ))
                            .cloned(),
                        maker: crossover_data
                            .get(&("maker".into(), "science_communications".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("policy".into(), "science_communications".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&(
                                "out_of_school_time_program".into(),
                                "science_communications".into(),
                            ))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("formal_education".into(), "science_communications".into()))
                            .cloned(),
                        science_communications: None,
                        unspecified: crossover_data
                            .get(&("science_communications".into(), "unspecified".into()))
                            .cloned(),
                    },
                    unspecified: OverviewCrossoverDataChartSegment {
                        proportion: if crossover_total > 0.0 {
                            (*crossover_involve.get("unspecified").unwrap_or(&0) as f64)
                                / crossover_total
                        } else {
                            0.0
                        },
                        citizen_science: crossover_data
                            .get(&("citizen_science".into(), "unspecified".into()))
                            .cloned(),
                        live_science: crossover_data
                            .get(&("live_science".into(), "unspecified".into()))
                            .cloned(),
                        museum_or_science_center: crossover_data
                            .get(&("museum_or_science_center".into(), "unspecified".into()))
                            .cloned(),
                        maker: crossover_data
                            .get(&("maker".into(), "unspecified".into()))
                            .cloned(),
                        policy: crossover_data
                            .get(&("policy".into(), "unspecified".into()))
                            .cloned(),
                        out_of_school_time_program: crossover_data
                            .get(&("out_of_school_time_program".into(), "unspecified".into()))
                            .cloned(),
                        formal_education: crossover_data
                            .get(&("formal_education".into(), "unspecified".into()))
                            .cloned(),
                        science_communications: crossover_data
                            .get(&("science_communications".into(), "unspecified".into()))
                            .cloned(),
                        unspecified: None,
                    },
                },
            },
        },
    })
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_region_lookup() {
        let found = common::geo::Query::new(format!("{}, {}", "Birmingham", "Alabama"), false)
            .lookup_one()
            .await;

        assert!(!found.is_none());

        let coords = found.map(|m| (m.geometry.longitude as f64, m.geometry.latitude as f64));

        assert_eq!(coords, Some((-86.80242919921875, 33.52068328857422)));
    }
}

/*
SNM DATA OVERVIEW

{
                "updated": "2022-07-28T14:33:27.12343242-07:00",
                "engagement": {
                    "data": {
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "search_max": 41363,
                        "stats": {
                            "unique_visitors": 5732,
                            "accounts": 1112,
                            "opportunity_views": 4214,
                            "opportunity_unique": 3214,
                            "opportunity_exits": 2341,
                            "didits": 123,
                            "saves": 632,
                            "likes": 423,
                            "shares": 343,
                            "calendar_adds": 211,
                        },
                        "searches": [
                            {"phrase": "Science Festival", "searches": 41363},
                            {"phrase": "mumblety-peg", "searches": 8123},
                            {"phrase": "kids and families", "searches": 712}
                        ],
                    },
                },

                "demographics": {
                    "sex": {
                        "male": {"index": 92, "proportion": 0.4507, "national": 0.4915},
                        "female": {"index": 108, "proportion": 0.5493, "national": 0.5085},
                    },
                    "age": {
                        "18-20": {"index": 96, "proportion": 0.0539, "national": 0.0562, "male": {"index": 108, "proportion": 0.0273, "national": 0.0253}, "female": {"index": 86, "proportion": 0.0266, "national": 0.0309}},
                        "21-24": {"index": 86, "proportion": 0.0679, "national": 0.0788, "male": {"index": 87, "proportion": 0.0307, "national": 0.0355}, "female": {"index": 86, "proportion": 0.0372, "national": 0.0433}},
                        "25-29": {"index": 135, "proportion": 0.1358, "national": 0.1005, "male": {"index": 105, "proportion": 0.0474, "national": 0.0453}, "female": {"index": 160, "proportion": 0.0884, "national": 0.0552}},
                        "30-34": {"index": 112, "proportion": 0.1149, "national": 0.1025, "male": {"index": 116, "proportion": 0.0535, "national": 0.0462}, "female":{"index": 109, "proportion": 0.0615, "national": 0.0563}},
                        "35-39": {"index": 112, "proportion": 0.1305, "national": 0.117, "male": {"index": 95, "proportion": 0.0501, "national": 0.0527}, "female": {"index": 125, "proportion": 0.0804, "national": 0.0642}},
                        "40-44": {"index": 101, "proportion": 0.1187, "national": 0.118, "male": {"index": 87, "proportion": 0.0463, "national": 0.0532}, "female": {"index": 112, "proportion": 0.0725, "national": 0.0648}},
                        "45-49": {"index": 94, "proportion": 0.1017, "national": 0.1076, "male": {"index": 104, "proportion": 0.0505, "national": 0.0485}, "female": {"index": 87, "proportion": 0.0512, "national": 0.0591}},
                        "50-54": {"index": 100, "proportion": 0.1093, "national": 0.1092, "male": {"index": 118, "proportion": 0.058, "national": 0.0492}, "female": {"index": 85, "proportion": 0.0512, "national": 0.06}},
                        "55-59": {"index": 89, "proportion": 0.058, "national": 0.0654, "male": {"index": 111, "proportion": 0.0326, "national": 0.0295}, "female": {"index": 71, "proportion": 0.0254, "national": 0.0359}},
                        "60-64": {"index": 76, "proportion": 0.0505, "national": 0.0668, "male": {"index": 92, "proportion": 0.0277, "national": 0.0301}, "female": {"index": 62, "proportion": 0.0228, "national": 0.0367}},
                        "65+": {"index": 75, "proportion": 0.0588, "national": 0.0781, "male": {"index": 75, "proportion": 0.0266, "national": 0.0352}, "female": {"index": 75, "proportion": 0.0322, "national": 0.0429}},
                    },
                    "education": {
                        "No College": {"index": 85, "proportion": 0.3642, "national": 0.4306},
                        "College": {"index": 103, "proportion": 0.4279, "national": 0.4158},
                        "Grad. Sch.": {"index": 135, "proportion": 0.2079, "national": 0.1536},
                    },
                    "income": {
                        "$0-50k": {"index": 84, "proportion": 0.3365, "national": 0.4028},
                        "$50-100k": {"index": 116, "proportion": 0.3657, "national": 0.3139},
                        "$100-150k": {"index": 114, "proportion": 0.1741, "national": 0.1532},
                        "$150k+": {"index": 95, "proportion": 0.1237, "national": 0.1301},
                    },
                    "children": {
                        "No Children under 17": {"index": 103, "proportion": 0.5247, "national": 0.5071},
                        "Some Children under 17": {"index": 96, "proportion": 0.4753, "national": 0.4929},
                    },
                    "ethnicity": {
                        "Cauc.": {"index": 98, "proportion": 0.7362, "national": 0.7506},
                        "Afr. Am.": {"index": 103, "proportion": 0.0961, "national": 0.0936},
                        "Asian": {"index": 140, "proportion": 0.0614, "national": 0.0437},
                        "Hisp": {"index": 99, "proportion": 0.0968, "national": 0.0978},
                        "Other": {"index": 66, "proportion": 0.0094, "national": 0.0143},
                    },
                },

                "states": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "max": {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        "states": {
                            'Texas': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "regional": {
                                'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                "regions": {
                                    'Agua Dulce': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-97.910833, 27.7825]},
                                    'Bear Creek': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-97.932778, 30.181944]},
                                    'Blackwell': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-100.319722, 32.085556]},
                                    'Buffalo Springs': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-101.709167, 33.532222]},
                                },
                            }},
                            'California': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "regional": {
                                'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                "regions": {
                                    'Arcata': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-124.090556, 40.868056]},
                                    'Buellton': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-120.193889, 34.614167]},
                                    'Cotati': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-122.709167, 38.327778]},
                                    'Eastvale': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-117.564167, 33.963611]},
                                },
                            }},
                            'Oregon': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "regional": {
                                'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                                "regions": {
                                    'Keizer': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-123.021944, 45.000556]},
                                    'Monmouth': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332, "point": [-123.23, 44.849167]},
                                    'Winston': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132, "point": [-123.4175, 43.121667]},
                                    'Nyssa': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432, "point": [-116.996944, 43.879167]},
                                },
                            }},
                        },
                    },
                },

                "technology": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        'max': {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        'mobile': {"Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                        'tablet': {"Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                        'desktop': {"Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                    },
                },

                "traffic": {
                    "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                    "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                    "data": {
                        "opportunity_status": "Live and Closed",
                        "time_period": "This Month",
                        "begin": "2022-07-27",
                        "end": "2022-07-29",
                        "columns": ["Unique", "New", "Returning"],
                        "max": {"Unique Users": 112, "New Users": 334, "Returning Users": 332, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                        "chart": [
                            {"date": "2022-07-29", "Unique": 15, "New": 8, "Returning": 4},
                            {"date": "2022-07-28", "Unique": 8, "New": 2, "Returning": 7},
                            {"date": "2022-07-27", "Unique": 13, "New": 11, "Returning": 1},
                        ],
                        "pie": {
                            "labels": ["Direct", "Payed Search", "Display", "Affiliates", "Other"],
                            "datasets": [{
                                "label": "Referrers by Type",
                                "hoverOffset": 4,
                                "backgroundColor": ["#387ab5", "#5da136", "#cd4c24", "#e7e93c", "#5abdda"],
                                "data": [202, 15, 11, 0, 0],
                            }],
                        },
                        "table": [
                            {"name": "Test Ref 1", "type": "Direct", "Unique Users": 57, "New Users": 234, "Returning Users": 232, "Total Pageviews": 123, "Unique Pageviews": 222, "Avg. Time": 332},
                            {"name": "Test Ref 2", "type": "Direct", "Unique Users": 112, "New Users": 134, "Returning Users": 332, "Total Pageviews": 223, "Unique Pageviews": 322, "Avg. Time": 132},
                            {"name": "Test Ref 3", "type": "Direct", "Unique Users": 33, "New Users": 334, "Returning Users": 132, "Total Pageviews": 323, "Unique Pageviews": 422, "Avg. Time": 432},
                            {"name": "Test Ref 4", "type": "Paid Search", "Unique Users": 3, "New Users": 34, "Returning Users": 32, "Total Pageviews": 23, "Unique Pageviews": 22, "Avg. Time": 32},
                            {"name": "Test Ref 5", "type": "Paid Search", "Unique Users": 12, "New Users": 14, "Returning Users": 32, "Total Pageviews": 23, "Unique Pageviews": 32, "Avg. Time": 12},
                            {"name": "Test Ref 6", "type": "Display", "Unique Users": 11, "New Users": 13, "Returning Users": 33, "Total Pageviews": 22, "Unique Pageviews": 32, "Avg. Time": 13},
                        ],
                    },
                },

                "crossover": {
                    "engagement_types": ["Views", "Unique", "Clicks to Website"],
                    "data": {
                        "engagement_type": "Views",
                        "chart": {
                            "citizen_science": {
                                "proportion": 0.23,
                                "live_science": {"Views": 0.6, "Unique": 0.5, "Clicks to Website": 0.333},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.05, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.166, "Unique": 0.04, "Clicks to Website": 0.333},
                                "policy": {"Views": 0.166, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.4, "Clicks to Website": 0.333},
                                "formal_education": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications": {"Views": 0.166, "Unique": 0.01, "Clicks to Website": 0.0},
                                "unspecified": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                            "live_science": {
                                "proportion": 0.05,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "museum_or_science_center": {
                                "proportion": 0.16,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "maker": {
                                "proportion": 0.21,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "policy": {
                                "proportion": 0.08,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "out_of_school_time_program": {
                                "proportion": 0.22,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "science_communications": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "formal_education": {
                                "proportion": 0.01,
                                "citizen_science": {"Views": .1, "Unique": 0.0, "Clicks to Website": 0.0},
                                "live_science": {"Views": 0.1, "Unique": 0.0, "Clicks to Website": 0.0},
                                "museum_or_science_center": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "policy": {"Views": 0.1, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "unspecified": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                            "science_communications": {
                                "proportion": 0.04,
                                "citizen_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "live_science": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "museum_or_science_center": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "maker": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "policy": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "out_of_school_time_program": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "formal_education": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166},
                                "unspecified": {"Views": 0.166, "Unique": 0.166, "Clicks to Website": 0.166}
                            },
                            "unspecified": {
                                "proportion": 0.0,
                                "citizen_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "live_science": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "museum_or_science_center": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "maker": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "policy": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "out_of_school_time_program": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "formal_education": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0},
                                "science_communications,": {"Views": 0.0, "Unique": 0.0, "Clicks to Website": 0.0}
                            },
                        },
                    },
                },
            }

*/
