use std::collections::BTreeMap;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Datelike, Days, FixedOffset, LocalResult, TimeZone, Utc};
use common::{
    model::analytics::{
        DetailedEngagementDataChart, DetailedEngagementDataChartWithPoint, EngagementDataChart,
        EngagementType, OpportunityChart, Organization, OrganizationEngagement,
        OrganizationEngagementData, OrganizationStates, OrganizationStatesData,
        OrganizationTechnology, OrganizationTechnologyData, OrganizationTraffic,
        OrganizationTrafficData, PieChart, PieData, RegionEngagement, RelativeTimePeriod,
        StateEngagement, Status, TrafficChart,
    },
    Database, ToFixedOffset,
};
use futures_util::TryStreamExt;
use strum::IntoEnumIterator;

use crate::CommonState;

pub async fn cache(
    _db: &Database,
    _org: &common::model::Partner,
    _temporary: bool,
    _begin: DateTime<FixedOffset>,
    _end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    // Here's where we fetch any data that are still needed for the complete
    // organization, but no need to fetch redundant information that
    // was already cached by the opportunity caching stage.

    Ok(())
}

pub async fn collect(
    db: &Database,
    org: &common::model::Partner,
    state: &CommonState,
) -> Result<Organization, Error> {
    let total_opportunities = org.count_total_opportunities(db).await?;
    let current_opportunities = org.count_current_opportunities(db).await?;

    let mut engagement_totals = EngagementDataChart::default();
    let mut engagement_max = EngagementDataChart::default();
    let mut engagement_data_chart = BTreeMap::new();
    let mut engagement_data_table = BTreeMap::new();

    let mut query = sqlx::query!(
        r#"
SELECT *
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND "partner" = $3 AND c_opportunity_by_uid_is_status("opportunity", $4)"#,
        state.begin,
        state.end,
        org.exterior.uid,
        state.status.discriminate()
    )
    .fetch(db);

    while let Ok(Some(entry)) = query.try_next().await {
        let date = entry.date.to_fixed_offset();

        let row: &mut EngagementDataChart = engagement_data_chart.entry(date).or_default();
        row.date = date;
        row.views += entry.views.try_into().unwrap_or(0);
        row.unique += entry.total_users.try_into().unwrap_or(0);
        row.new += entry.new_users.try_into().unwrap_or(0);
        row.returning = row.unique - row.new;

        if let Some(opp_info) = sqlx::query!(
            r#"
SELECT
  exterior->>'title' AS "name!",
  exterior->>'slug' AS "slug!"
FROM c_opportunity
WHERE (exterior->>'uid')::uuid = $1 LIMIT 1
"#,
            entry.opportunity
        )
        .fetch_optional(db)
        .await?
        {
            let row: &mut OpportunityChart =
                engagement_data_table.entry(entry.opportunity).or_default();
            row.name = opp_info.name;
            row.slug = opp_info.slug;
            row.values.views += entry.views.try_into().unwrap_or(0);
            row.values.unique += entry.total_users.try_into().unwrap_or(0);
            row.values.new += entry.new_users.try_into().unwrap_or(0);
            row.values.returning = row.values.unique - row.values.new;
        }
    }

    for row in engagement_data_chart.values_mut() {
        let LocalResult::Single(begin) = row.date.timezone().with_ymd_and_hms(
            row.date.year(),
            row.date.month(),
            row.date.day(),
            0,
            0,
            0,
        ) else { println!("Error calculating beginning of day: {}", row.date); continue; };

        let Some(end) = begin.checked_add_days(Days::new(1)) else { println!("Error calculating end of day: {}", row.date); continue; };

        row.clicks = sqlx::query_scalar!(
            r#"
SELECT
  COUNT(*) AS "count!: i64"
FROM c_log INNER JOIN c_opportunity ON c_log."object" = (c_opportunity.exterior->>'uid')::uuid
WHERE
  "action" = 'external' AND
  (c_opportunity.exterior->>'partner')::uuid = $1 AND
  "when" >= $2 AND
  "when" < $3
"#,
            org.exterior.uid,
            begin,
            end
        )
        .fetch_one(db)
        .await?
        .try_into()
        .unwrap_or(0);

        engagement_totals.views += row.views;
        engagement_totals.unique += row.unique;
        engagement_totals.new += row.new;
        engagement_totals.returning += row.returning;
        engagement_totals.clicks += row.clicks;
    }

    for (uid, row) in engagement_data_table.iter_mut() {
        row.values.clicks = sqlx::query_scalar!(
            r#"
SELECT
  COUNT(*) AS "count!: i64"
FROM c_log
WHERE
  "action" = 'external' AND
  "object" = $1 AND
  "when" >= $2 AND
  "when" < $3
"#,
            uid,
            state.begin,
            state.end
        )
        .fetch_one(db)
        .await?
        .try_into()
        .unwrap_or(0);

        engagement_max.views = engagement_max.views.max(row.values.views);
        engagement_max.unique = engagement_max.unique.max(row.values.unique);
        engagement_max.new = engagement_max.new.max(row.values.new);
        engagement_max.returning = engagement_max.returning.max(row.values.returning);
        engagement_max.clicks = engagement_max.clicks.max(row.values.clicks);
    }

    let mut states_max = DetailedEngagementDataChart::default();
    let mut states = BTreeMap::new();

    let mut states_rows = sqlx::query!(
        r#"
SELECT
  "region" AS "state!: String",
  SUM("total_users") AS "unique_users!: i64",
  SUM("new_users") AS "new_users!: i64",
  SUM("total_users") - SUM("new_users") AS "returning_users!: i64",
  SUM("views") AS "total_pageviews!: i64",
  SUM("sessions") AS "unique_pageviews!: i64",
  AVG("engagement_duration") AS "average_time!: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND "partner" = $3 AND c_opportunity_by_uid_is_status("opportunity", $4)
GROUP BY "region"
"#,
        state.begin,
        state.end,
        org.exterior.uid,
        state.status.discriminate()
    )
    .fetch(db);

    while let Some(state_row) = states_rows.try_next().await? {
        let state_name = state_row.state;

        let state_row = DetailedEngagementDataChart {
            date: None,
            unique_users: state_row.unique_users.try_into().unwrap_or(0),
            new_users: state_row.new_users.try_into().unwrap_or(0),
            returning_users: state_row.returning_users.try_into().unwrap_or(0),
            total_pageviews: state_row.total_pageviews.try_into().unwrap_or(0),
            unique_pageviews: state_row.unique_pageviews.try_into().unwrap_or(0),
            average_time: state_row.average_time.try_into().unwrap_or(0.0),
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
  SUM("total_users") AS "unique_users!: i64",
  SUM("new_users") AS "new_users!: i64",
  SUM("total_users") - SUM("new_users") AS "returning_users!: i64",
  SUM("views") AS "total_pageviews!: i64",
  SUM("sessions") AS "unique_pageviews!: i64",
  AVG("engagement_duration") AS "average_time!: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND "region" = $3 AND "partner" = $4 AND c_opportunity_by_uid_is_status("opportunity", $5)
GROUP BY "city"
"#,
            state.begin,
            state.end,
            &state_name,
            org.exterior.uid,
            state.status.discriminate()
        )
        .fetch(db);

        while let Some(region_row) = regions_rows.try_next().await? {
            let region_name = region_row.region;

            let region_row = DetailedEngagementDataChart {
                date: None,
                unique_users: region_row.unique_users.try_into().unwrap_or(0),
                new_users: region_row.new_users.try_into().unwrap_or(0),
                returning_users: region_row.returning_users.try_into().unwrap_or(0),
                total_pageviews: region_row.total_pageviews.try_into().unwrap_or(0),
                unique_pageviews: region_row.unique_pageviews.try_into().unwrap_or(0),
                average_time: region_row.average_time.try_into().unwrap_or(0.0),
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
  SUM("total_users") AS "unique_users!: i64",
  SUM("new_users") AS "new_users!: i64",
  SUM("total_users") - SUM("new_users") AS "returning_users!: i64",
  SUM("views") AS "total_pageviews!: i64",
  SUM("sessions") AS "unique_pageviews!: i64",
  AVG("engagement_duration") AS "average_time!: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND "partner" = $3 AND c_opportunity_by_uid_is_status("opportunity", $4)
GROUP BY "device_category"
"#,
        state.begin,
        state.end,
        org.exterior.uid,
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
            unique_users: row.unique_users.try_into().unwrap_or(0),
            new_users: row.new_users.try_into().unwrap_or(0),
            returning_users: row.returning_users.try_into().unwrap_or(0),
            total_pageviews: row.total_pageviews.try_into().unwrap_or(0),
            unique_pageviews: row.unique_pageviews.try_into().unwrap_or(0),
            average_time: row.average_time.try_into().unwrap_or(0.0),
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
  SUM("views") AS "views!: i64",
  SUM("sessions") AS "unique!: i64",
  SUM("new_users") AS "new!: i64",
  SUM("sessions") - SUM("new_users") AS "returning!: i64",
  (
    SELECT COUNT(*)
    FROM c_log INNER JOIN c_opportunity ON c_log."object" = (c_opportunity.exterior->>'uid')::uuid
    WHERE
      "action" = 'external' AND
      (c_opportunity.exterior->>'partner')::uuid = $1 AND
      "when"::date = c_analytics_cache."date"::date
  ) AS "clicks!: i64"
FROM c_analytics_cache
WHERE "partner" = $1 AND "date" >= $2 AND "date" < $3 AND c_opportunity_by_uid_is_status("opportunity", $4)
GROUP BY "date"
"#,
        org.exterior.uid,
        state.begin,
        state.end,
        state.status.discriminate()
    )
    .map(|row| EngagementDataChart {
        date: row.date,
        views: row.views.try_into().unwrap_or(0),
        unique: row.unique.try_into().unwrap_or(0),
        new: row.new.try_into().unwrap_or(0),
        returning: row.returning.try_into().unwrap_or(0),
        clicks: row.clicks.try_into().unwrap_or(0),
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
SELECT "session_channel_group" AS "group!", SUM("views") AS "count!: i64"
FROM c_analytics_cache
WHERE "partner" = $1 AND "date" >= $2 AND "date" < $3 AND c_opportunity_by_uid_is_status("opportunity", $4)
GROUP BY "session_channel_group"
ORDER BY "session_channel_group"
"#,
                org.exterior.uid,
                state.begin,
                state.end,
                state.status.discriminate()
            )
            .map(|row| row.count.try_into().unwrap_or(0))
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
  SUM("total_users") AS "unique_users!: i64",
  SUM("new_users") AS "new_users!: i64",
  SUM("total_users") - SUM("new_users") AS "returning_users!: i64",
  SUM("views") AS "total_pageviews!: i64",
  SUM("sessions") AS "unique_pageviews!: i64",
  AVG("engagement_duration") AS "average_time!: f64"
FROM c_analytics_cache
WHERE "begin" = $1 AND "end" = $2 AND "partner" = $3 AND c_opportunity_by_uid_is_status("opportunity", $4)
GROUP BY "page_referrer", "session_channel_group"
"#,
        state.begin,
        state.end,
        org.exterior.uid,
        state.status.discriminate()
    )
    .map(|row| {
        let chart = TrafficChart {
            name: row.page_referrer,
            type_: row.type_,
            values: DetailedEngagementDataChart {
                date: None,
                unique_users: row.unique_users.try_into().unwrap_or(0),
                new_users: row.new_users.try_into().unwrap_or(0),
                returning_users: row.returning_users.try_into().unwrap_or(0),
                total_pageviews: row.total_pageviews.try_into().unwrap_or(0),
                unique_pageviews: row.unique_pageviews.try_into().unwrap_or(0),
                average_time: row.average_time.try_into().unwrap_or(0.0),
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

    Ok(Organization {
        organization: org.exterior.uid,
        name: org.exterior.name.clone(),
        updated: Utc::now().to_fixed_offset(),
        total_opportunities,
        current_opportunities,
        engagement: OrganizationEngagement {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OrganizationEngagementData {
                organization: org.exterior.uid,
                opportunity_status: state.status,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                columns: EngagementType::iter().collect(),
                totals: engagement_totals,
                max: engagement_max,
                chart: engagement_data_chart.into_values().collect(),
                table: engagement_data_table.into_values().collect(),
            },
        },
        states: OrganizationStates {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OrganizationStatesData {
                organization: org.exterior.uid,
                opportunity_status: state.status,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                max: states_max,
                states,
            },
        },
        technology: OrganizationTechnology {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OrganizationTechnologyData {
                organization: org.exterior.uid,
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
        traffic: OrganizationTraffic {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OrganizationTrafficData {
                organization: org.exterior.uid,
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
    })
}

/*
ORGANIZATION DATA OVERVIEW
              {
                "Demo Org": {
                    "uid": 'c36bd22f-f530-4469-8c9e-b919951e3486',
                    "updated": "2022-07-28T14:33:27.12343242-07:00",
                    "total_opportunities": 23,
                    "current_opportunities": 18,
                    "engagement": {
                        "opportunity_statuses": ["Live and Closed", "Live", "Closed"],
                        "time_periods": ["This Month", "Last Month", "This Quarter", "Last Quarter", "This Semiannum", "Last Semiannum", "This Year", "Last Year", "All Time"],
                        "data": {
                            "opportunity_status": "Live and Closed",
                            "time_period": "This Month",
                            "begin": "2022-07-27",
                            "end": "2022-07-29",
                            "columns": ["Views" , "Unique", "Clicks to Website"],
                            "totals": {"Views": 36, "Unique": 21, "Clicks to Website": 12},
                            "max": {"Views": 432, "Unique": 234, "Clicks to Website": 210},
                            "chart": [
                                {"date": "2022-07-29", "Views": 15, "Unique": 8, "Clicks to Website": 4},
                                {"date": "2022-07-28", "Views": 8, "Unique": 2, "Clicks to Website": 7},
                                {"date": "2022-07-27", "Views": 13, "Unique": 11, "Clicks to Website": 1},
                            ],
                            "table": [
                                {"name": "Test Opp 1", "slug": "test-opp-1", "Views": 432, "Unique": 234, "Clicks to Website": 119},
                                {"name": "Test Opp 2", "slug": "test-opp-2", "Views": 321, "Unique": 78, "Clicks to Website": 210},
                                {"name": "Test Opp 3", "slug": "test-opp-3", "Views": 210, "Unique": 112, "Clicks to Website": 87},
                                {"name": "Test Opp 4", "slug": "test-opp-4", "Views": 122, "Unique": 34, "Clicks to Website": 12},
                                {"name": "Test Opp 5", "slug": "test-opp-5", "Views": 97, "Unique": 12, "Clicks to Website": 4},
                                {"name": "Test Opp 6", "slug": "test-opp-6", "Views": 15, "Unique": 2, "Clicks to Website": 1},
                            ],
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
                },
            }
*/
