use std::collections::BTreeMap;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Datelike, Days, FixedOffset, LocalResult, TimeZone, Utc};
use common::{
    model::analytics::{
        DetailedEngagementDataChart, DetailedEngagementDataChartWithPoint, EngagementDataBar,
        EngagementDataChart, EngagementType, Opportunity, OpportunityEngagement,
        OpportunityEngagementData, OpportunityEngagementDataBars, OpportunityOverlap,
        OpportunityOverlapChart, OpportunityOverlapData, OpportunityStates, OpportunityStatesData,
        OpportunityTechnology, OpportunityTechnologyData, OpportunityTraffic,
        OpportunityTrafficData, PieChart, PieData, RegionEngagement, RelativeTimePeriod,
        StateEngagement, Status, TrafficChart,
    },
    Database, ToFixedOffset,
};
use futures_util::TryStreamExt;
use google_analyticsdata1_beta::api::{Filter, FilterExpression, StringFilter};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{ga4, CommonState};

pub async fn cache(
    db: &Database,
    opp: &common::model::opportunity::Opportunity,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    if ga4::is_cached(db, begin, end, opp.exterior.uid).await {
        return Ok(());
    }

    ga4::cache_report(
        db,
        begin,
        end,
        FilterExpression {
            // https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/FilterExpression
            filter: Some(Filter {
                //field_name: Some(String::from("pagePath")),
                field_name: Some(String::from("customEvent:entity_uid")),
                string_filter: Some(StringFilter {
                    case_sensitive: Some(false),
                    match_type: Some(String::from("EXACT")),
                    value: Some(opp.exterior.uid.to_string()),
                    //value: Some("c6e5ee2c-56ec-54d3-9b76-83359943ef05".into()),
                    //match_type: Some(String::from("ENDS_WITH")),
                    //value: Some(format!("/{}", &opp.exterior.slug)),
                    //value: Some("/become-a-techgirl-1".into()),
                }),
                // in_list_filter: Some(InListFilter {
                //     values: Some(vec![
                //         String::from("/exchange/b9224b48-dcc3-5153-9c31-7b53ff24a380/"),
                //         String::from("/exchange/b9224b48-dcc3-5153-9c31-7b53ff24a380/summer-science-institute"),
                //     ]),
                //     case_sensitive: Some(false),
                // }),
                ..Default::default()
            }),
            ..Default::default()
        },
        temporary,
    )
    .await;

    Ok(())
}

pub async fn collect(
    db: &Database,
    opp: &common::model::opportunity::Opportunity,
    state: &CommonState,
) -> Result<Opportunity, Error> {
    let mut engagement_data_chart = BTreeMap::new();

    let mut query = sqlx::query!(
        r#"SELECT * FROM c_analytics_cache WHERE "begin" = $1 AND "end" = $2 AND "opportunity" = $3"#,
        state.begin,
        state.end,
        opp.exterior.uid
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
            r#"SELECT COUNT(*) AS "count!: i64" FROM c_log WHERE "action" = 'external' AND "object" = $1 AND "when" >= $2 AND "when" < $3"#,
            opp.exterior.uid,
            begin,
            end
        )
        .fetch_one(db)
        .await?.try_into().unwrap_or(0);
    }

    let opp_clicks = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!: i64" FROM c_log WHERE "action" = 'external' AND "object" = $1 AND "when" >= $2 AND "when" < $3"#,
            opp.exterior.uid,
            state.begin,
            state.end
        )
        .fetch_one(db)
        .await?.try_into().unwrap_or(0);

    let (opp_views, opp_unique) = sqlx::query!(
        r#"SELECT SUM("views") AS "views!: i64", SUM("total_users") AS "unique!: i64" FROM c_analytics_cache WHERE "opportunity" = $1 AND "begin" = $2 AND "end" = $3"#,
        opp.exterior.uid,
        state.begin,
        state.end
    )
        .map(|row| {
            (
                row.views.try_into().unwrap_or(0),
                row.unique.try_into().unwrap_or(0),
            )
        })
        .fetch_one(db)
        .await?;

    let mut states_max = DetailedEngagementDataChart::default();
    let mut states = BTreeMap::new();

    // Note: in the values returned from GA4 and stored in the
    // database, "region" denotes the state and "city" denotes the
    // area within the state. In the engagement chart data, "state"
    // denotes the state and "region" denotes the area within the
    // state. The the word "region" is used with two different
    // meanings: "region" -> "state" and "city" -> "region"

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
WHERE "begin" = $1 AND "end" = $2 AND "opportunity" = $3
GROUP BY "region"
"#,
        state.begin,
        state.end,
        opp.exterior.uid,
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
WHERE "begin" = $1 AND "end" = $2 AND "region" = $3 AND "opportunity" = $4
GROUP BY "city"
"#,
            state.begin,
            state.end,
            &state_name,
            opp.exterior.uid,
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

    let mut tech_rows = sqlx::query!(
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
WHERE "begin" = $1 AND "end" = $2 AND "opportunity" = $3
GROUP BY "device_category"
"#,
        state.begin,
        state.end,
        opp.exterior.uid,
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
    FROM c_log
    WHERE
      "action" = 'external' AND
      "object" = $1 AND
      "when"::date = c_analytics_cache."date"::date
  ) AS "clicks!: i64"
FROM c_analytics_cache
WHERE "opportunity" = $1 AND "date" >= $2 AND "date" < $3
GROUP BY "date"
"#,
        opp.exterior.uid,
        state.begin,
        state.end,
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
WHERE "opportunity" = $1 AND "date" >= $2 AND "date" < $3
GROUP BY "session_channel_group"
ORDER BY "session_channel_group"
"#,
                opp.exterior.uid,
                state.begin,
                state.end,
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
WHERE "begin" = $1 AND "end" = $2 AND "opportunity" = $3
GROUP BY "page_referrer", "session_channel_group"
"#,
        state.begin,
        state.end,
        opp.exterior.uid,
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

    let overlap = sqlx::query!(
        r#"
WITH c_neighbors AS (
  SELECT
    "other",
    COUNT(*) AS "views"
  FROM (
    SELECT
      "created" AS "when",
      "prior" AS "other"
    FROM c_transit
    WHERE
      "postor" = $1::uuid AND
      "created" >= $2 AND
      "created" < $3
    UNION
    SELECT
      "created" AS "when",
      "postor" AS "other"
    FROM c_transit
    WHERE
      "prior" = $1::uuid AND
      "created" >= $2 AND
      "created" < $3
     
  ) AS "inner"
  GROUP BY "other"
)
SELECT
  c_neighbors."views"::real / $4::real AS "overlap!",
  c_opportunity.exterior->>'title' AS "name!",
  c_opportunity.exterior->>'organization_name' AS "host!",
  c_opportunity.exterior->'opp_descriptor' AS "activity_types!",
  CASE
    WHEN (c_opportunity.exterior->'has_end')::bool THEN 'Event'
    ELSE 'On Demand'
  END AS "format!",
  c_opportunity.exterior->'opp_venue' AS "venue_types!",
  (c_opportunity.exterior->'min_age')::smallint AS "min_age!",
  (c_opportunity.exterior->'max_age')::smallint AS "max_age!"
FROM
  c_neighbors INNER JOIN c_opportunity
  ON c_neighbors."other"::text = c_opportunity."exterior"->>'uid'
WHERE c_opportunity.exterior->>'entity_type' = 'opportunity'
ORDER BY "overlap!" DESC;
"#,
        opp.exterior.uid,
        state.begin,
        state.end,
        opp_clicks as f32
    )
    .map(|row| OpportunityOverlapChart {
        name: row.name,
        overlap: row.overlap.into(),
        host: row.host,
        activity_types: serde_json::from_value(row.activity_types).unwrap_or_default(),
        format: row.format,
        venue_types: serde_json::from_value(row.venue_types).unwrap_or_default(),
        min_age: row.min_age,
        max_age: row.max_age,
    })
    .fetch_all(db)
    .await?;

    Ok(Opportunity {
        opportunity: opp.exterior.uid,
        updated: Utc::now().to_fixed_offset(),
        engagement: OpportunityEngagement {
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OpportunityEngagementData {
                opportunity: opp.exterior.uid,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                columns: EngagementType::iter().collect(),
                chart: engagement_data_chart.into_values().collect(),
                bars: OpportunityEngagementDataBars {
                    self_: EngagementDataBar {
                        views: opp_views,
                        unique: opp_unique,
                        clicks: opp_clicks,
                    },
                    mean: state.engagement_mean.clone(),
                    median: state.engagement_median.clone(),
                },
            },
        },
        states: OpportunityStates {
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OpportunityStatesData {
                opportunity: opp.exterior.uid,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                max: states_max,
                states,
            },
        },
        technology: OpportunityTechnology {
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OpportunityTechnologyData {
                opportunity: opp.exterior.uid,
                time_period: state.period,
                begin: state.begin,
                end: state.end,
                max: tech_max,
                mobile: tech_mobile,
                tablet: tech_tablet,
                desktop: tech_desktop,
            },
        },
        traffic: OpportunityTraffic {
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OpportunityTrafficData {
                opportunity: opp.exterior.uid,
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
        overlap: OpportunityOverlap {
            engagement_types: EngagementType::iter().collect(),
            data: OpportunityOverlapData {
                engagement_type: EngagementType::Views,
                table: overlap,
            },
        },
    })
}

/*
OPPORTUNITY DATA EXPLORER
            {
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
                        "chart": [
                            {"date": "2022-07-29", "Views": 15, "Unique": 8, "Clicks to Website": 4},
                            {"date": "2022-07-28", "Views": 8, "Unique": 2, "Clicks to Website": 7},
                            {"date": "2022-07-27", "Views": 13, "Unique": 11, "Clicks to Website": 1},
                        ],
                        "bars": {
                            "self": {"Views": 432, "Unique": 234, "Clicks to Website": 119},
                            "mean": {"Views": 321, "Unique": 78, "Clicks to Website": 210},
                            "median": {"Views": 210, "Unique": 112, "Clicks to Website": 87},
                        },
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

                "overlap": {
                    "engagement_types": ["Views", "Unique", "Clicks to Website"],
                    "data": {
                        "engagement_type": "Views",
                        "table": [
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                            {"name": "Test Opp 1", "overlap": 0.73, "host": "Moocow Projects", "activity_types": ["science_slam", "service"], "format": "Event", "venue_types": ["indoors"], "min_age": 16, "max_age": 999},
                            {"name": "Test Opp 2", "overlap": 0.21, "host": "Demo Org", "activity_types": ["science_slam", "service"], "format": "On Demand", "venue_types": ["indoors"], "min_age": 16, "max_age": 18},
                            {"name": "Test Opp 3", "overlap": 0.04, "host": "Bonzo McBean", "activity_types": ["service"], "format": "On Demand", "venue_types": ["outdoors"], "min_age": 0, "max_age": 999},
                        ]
                    },
                },
            }

*/
