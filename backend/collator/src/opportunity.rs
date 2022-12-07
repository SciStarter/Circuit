use std::collections::BTreeMap;

use anyhow::Error;
use chrono::{DateTime, Datelike, Days, FixedOffset, LocalResult, TimeZone, Utc};
use common::{
    model::analytics::{
        AbsoluteTimePeriod, EngagementDataBar, EngagementDataChart, EngagementType, Opportunity,
        OpportunityEngagement, OpportunityEngagementData, OpportunityEngagementDataBars,
        RelativeTimePeriod, Status,
    },
    Database, ToFixedOffset,
};
use google_analyticsdata1_beta::api::{Filter, FilterExpression, StringFilter};
use strum::IntoEnumIterator;

use crate::ga4;

pub async fn cache(
    db: &Database,
    opp: &common::model::Opportunity,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    if ga4::is_cached(db, begin, end, opp.exterior.uid).await {
        return;
    }

    ga4::cache_report(
        db,
        begin,
        end,
        FilterExpression {
            // https://developers.google.com/analytics/devguides/reporting/data/v1/rest/v1beta/FilterExpression
            filter: Some(Filter {
                field_name: Some(String::from("pagePath")),
                string_filter: Some(StringFilter {
                    case_sensitive: Some(false),
                    match_type: Some(String::from("ENDS_WITH")),
                    value: Some(format!("/{}", &opp.exterior.slug)),
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
        opp.exterior.uid,
        temporary,
    )
    .await?
}

pub async fn collect(
    db: &Database,
    opp: &common::model::opportunity::Opportunity,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<Opportunity, Error> {
    let AbsoluteTimePeriod { begin, end } = period.absolute();

    let org = common::model::partner::Partner::load_by_uid(db, &opp.exterior.partner).await?;
    let total_opportunities = org.count_total_opportunities(db).await?;
    let current_opportunities = org.count_current_opportunities(db).await?;

    let mut engagement_data_chart = BTreeMap::new();

    let mut query = sqlx::query!(
        r#"SELECT * FROM c_analytics_cache WHERE "begin" = $1 AND "end" = $2 AND "about" = $3"#,
        begin,
        end,
        opp.exterior.uid
    )
    .fetch(db);

    while let Some(entry) = query.try_next().await {
        let date = entry.date;
        let views = {
            if entry.views > 0 {
                entry.views
            } else {
                // best guess fallback, view is the primary event on opp page
                entry.events
            }
        };

        let row: &mut EngagementDataChart = engagement_data_chart.entry(date).or_default();
        row.date = date;
        row.views += views;
        row.unique += entry.total_users;
        row.new += entry.new_users;
        row.returning = row.unique - row.new;

        dbg!(entry);
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

        let Some(end) = begin.checked_add_days(Days::new(1)) else {println!("Error calculating end of day: {}", row.date); continue; };

        row.clicks = sqlx::query_scalar!(
            r#"SELECT COUNT(*) AS "count!: i64" FROM c_log WHERE "action" = 'external' AND "object" = $1 AND "when" >= $2 AND "when" < $3"#,
            opp.exterior.uid,
            begin,
            end
        )
        .fetch_one(db)
        .await
        .unwrap_or(0)
        .try_into()
        .unwrap_or(0);
    }

    // !!!
    Ok(Opportunity {
        opportunity: opp.exterior.uid,
        updated: Utc::now().to_fixed_offset(),
        total_opportunities,
        current_opportunities,
        engagement: OpportunityEngagement {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OpportunityEngagementData {
                opportunity: opp.exterior.uid,
                opportunity_status: if opp.current() {
                    Status::Closed
                } else {
                    Status::Live
                },
                time_period: period,
                begin,
                end,
                columns: EngagementType::iter().collect(),
                chart: engagement_data_chart.into_values().collect(),
                bars: OpportunityEngagementDataBars {
                    self_: EngagementDataBar {
                        views: todo!(),
                        unique: todo!(),
                        clicks: todo!(),
                    },
                    mean: EngagementDataBar {
                        views: todo!(),
                        unique: todo!(),
                        clicks: todo!(),
                    },
                    median: EngagementDataBar {
                        views: todo!(),
                        unique: todo!(),
                        clicks: todo!(),
                    },
                },
            },
        },
        states: todo!(),
        technology: todo!(),
        traffic: todo!(),
        overlap: todo!(),
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
