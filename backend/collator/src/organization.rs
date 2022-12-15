use anyhow::Error;
use chrono::{DateTime, FixedOffset, Utc};
use common::{
    model::analytics::{
        OpportunityStatesData, OpportunityTechnology, OpportunityTechnologyData, Organization,
        OrganizationEngagement, OrganizationEngagementData, OrganizationStates,
        OrganizationStatesData, OrganizationTechnology, OrganizationTechnologyData,
        OrganizationTraffic, OrganizationTrafficData, RelativeTimePeriod, Status,
    },
    Database, ToFixedOffset,
};
use google_analyticsdata1_beta::api::{Filter, FilterExpression, StringFilter};
use strum::IntoEnumIterator;
use uuid::Uuid;

use crate::{ga4, CommonState};

pub async fn cache(
    db: &Database,
    org: &common::model::Partner,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    // Fetch any data that are still needed for the complete
    // organization, but no need to fetch redundant information that
    // was already cached by the opportunity caching stage.

    // ga4::cache_report(
    //     db,
    //     begin,
    //     end,
    //     FilterExpression {
    //         filter: Some(Filter {
    //             field_name: Some(String::from("customEvent:partner_uid")),
    //             string_filter: Some(StringFilter {
    //                 case_sensitive: Some(false),
    //                 match_type: Some(String::from("EXACT")),
    //                 value: Some(org.exterior.uid.to_string()),
    //                 //value: Some("65a73b33-6f39-54b2-a2ee-d42f2d2b63df".into()),
    //             }),
    //             ..Default::default()
    //         }),
    //         ..Default::default()
    //     },
    //     org.exterior.uid,
    //     temporary,
    // )
    // .await;

    Ok(())
}

pub async fn collect(
    db: &Database,
    org: &common::model::Partner,
    state: &CommonState,
) -> Result<Organization, Error> {
    let total_opportunities = org.count_total_opportunities(db).await?;
    let current_opportunities = org.count_current_opportunities(db).await?;

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
                opportunity_status: todo!(),
                time_period: todo!(),
                begin: todo!(),
                end: todo!(),
                columns: todo!(),
                totals: todo!(),
                max: todo!(),
                chart: todo!(),
                table: todo!(),
            },
        },
        states: OrganizationStates {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OpportunityStatesData {
                opportunity: todo!(),
                opportunity_status: todo!(),
                time_period: todo!(),
                begin: todo!(),
                end: todo!(),
                max: todo!(),
                states: todo!(),
            },
        },
        technology: OrganizationTechnology {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OrganizationTechnologyData {
                organization: todo!(),
                opportunity_status: todo!(),
                time_period: todo!(),
                begin: todo!(),
                end: todo!(),
                max: todo!(),
                mobile: todo!(),
                tablet: todo!(),
                desktop: todo!(),
            },
        },
        traffic: OrganizationTraffic {
            opportunity_statuses: Status::iter().collect(),
            time_periods: RelativeTimePeriod::iter().collect(),
            data: OrganizationTrafficData {
                organization: todo!(),
                opportunity_status: todo!(),
                time_period: todo!(),
                begin: todo!(),
                end: todo!(),
                columns: todo!(),
                chart: todo!(),
                pie: todo!(),
                max: todo!(),
                table: todo!(),
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
