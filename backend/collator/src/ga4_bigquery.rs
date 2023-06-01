use std::collections::BTreeMap;
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Days, FixedOffset, Utc};
use common::{Database, ToFixedOffset};
use google_analyticsdata1_beta::api::{
    DateRange, Dimension, Filter, FilterExpression, Metric, RunReportRequest, RunReportResponse,
    StringFilter,
};
use google_analyticsdata1_beta::{hyper, hyper_rustls};
use google_bigquery2::api::{
    QueryParameter, QueryParameterType, QueryParameterValue, QueryRequest,
};
use google_bigquery2::oauth2;
use once_cell::sync::Lazy;
use uuid::Uuid;

use crate::reportiter::BQReportIterator;

static FIELD_SUBSTITUTIONS: Lazy<BTreeMap<String, String>> = Lazy::new(|| {
    [
        (
            r#"pagePathPlusQueryString"#.into(),
            r#"REGEXP_EXTRACT(page_location, "^https?://[^/]+(.*)")"#.into(),
        ),
        (
            r#"pagePath"#.into(),
            r#"REGEXP_EXTRACT(page_location, "^https?://[^/]+([^?]*)")"#.into(),
        ),
        (
            r#"firstSessionDate"#.into(),
            r#"DATE(TIMESTAMP_MICROS(user_first_touch_timestamp))"#.into(),
        ),
        (r#"customEvent:entity_uid"#.into(), r#"entity_uid"#.into()),
        (r#"customEvent:partner_uid"#.into(), r#"partner_uid"#.into()),
        (r#"city"#.into(), r#"geo_city"#.into()),
        (r#"region"#.into(), r#"geo_region"#.into()),
        (r#"date"#.into(), r#"event_date"#.into()),
        (r#"deviceCategory"#.into(), r#"device_category"#.into()),
        (r#"date"#.into(), r#"event_date"#.into()),
        (r#"sessionDefaultChannelGroup"#.into(), r#"medium"#.into()),
    ]
    .into_iter()
    .collect()
});

pub use crate::ga4::{
    clear_cached_temporary, is_cached, is_overview_cached, is_search_terms_cached,
};

static BQ_SIMULTANEOUS: Lazy<tokio::sync::Semaphore> = Lazy::new(|| tokio::sync::Semaphore::new(8));

pub async fn run_report(
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    dimension_filter: Option<FilterExpression>,
    dimensions: Vec<Dimension>,
    metrics: Vec<Metric>,
) -> Result<BQReportIterator, Error> {
    let permit = BQ_SIMULTANEOUS.acquire().await?;

    let secret = oauth2::read_service_account_key(
        env::var("SNM_ANALYTICS_SECRET")
            .unwrap_or(String::from("/etc/ga4/snm-analytics-secret.json")),
    )
    .await
    .expect("read service account key");

    let auth = oauth2::ServiceAccountAuthenticator::builder(secret)
        .build()
        .await
        .expect("create authenticator");

    let hub = google_bigquery2::Bigquery::new(
        hyper::Client::builder().build(
            hyper_rustls::HttpsConnectorBuilder::new()
                .with_native_roots()
                .https_or_http()
                .enable_http1()
                .enable_http2()
                .build(),
        ),
        auth,
    );

    let begin = begin.date_naive();
    let Some(end) = end.date_naive().checked_sub_days(Days::new(1)) else { return Err(anyhow!("End date out of range")) };

    let mut ident_issuer = std::iter::successors(Some(1u32), |n| n.checked_add(1));

    let mut group_clauses = vec!["event_date".to_string()];
    let mut select_clauses = vec!["event_date".to_string()];
    let mut where_clauses = vec![
        r#"event_date >= @begin"#.to_string(),
        r#"event_date <= @end"#.to_string(),
    ];

    let mut params = vec![
        QueryParameter {
            name: Some("begin".into()),
            parameter_type: Some(QueryParameterType {
                type_: Some("STRING".into()),
                ..Default::default()
            }),
            parameter_value: Some(QueryParameterValue {
                value: Some(
                    "20230101".into(), /*begin.format("%Y%m%d").to_string()*/
                ),
                ..Default::default()
            }),
        },
        QueryParameter {
            name: Some("end".into()),
            parameter_type: Some(QueryParameterType {
                type_: Some("STRING".into()),
                ..Default::default()
            }),
            parameter_value: Some(QueryParameterValue {
                value: Some(end.format("%Y%m%d").to_string()),
                ..Default::default()
            }),
        },
    ];

    for dim in dimensions {
        match dim {
            Dimension {
                name: Some(name), ..
            } if FIELD_SUBSTITUTIONS.contains_key(&name) => {
                let normalized_name = name.replace("customEvent:", "customEvent_");
                group_clauses.push(format!("`{}`", &normalized_name));
                select_clauses.push(format!(
                    "{} AS `{}`",
                    FIELD_SUBSTITUTIONS
                        .get(&name)
                        .expect("we just checked that it's there"),
                    normalized_name
                ));
            }
            _ => panic!("Unsupported dimension: {:?}", dim),
        }
    }

    for met in metrics {
        match met {
            Metric {
                name: Some(name), ..
            } if name == "screenPageViews" => {
                select_clauses.push(r#"COUNT(*) AS screenPageViews"#.to_string());
            }
            Metric {
                name: Some(name), ..
            } if name == "sessions" => {
                select_clauses.push(r#"COUNT(distinct session_id) AS sessions"#.to_string());
            }
            Metric {
                name: Some(name), ..
            } if name == "eventCount" => {
                // We filterd out non-view events, so this is not
                // quite correct, but the correct value would not be
                // particularly useful.
                select_clauses.push(r#"COUNT(*) AS eventCount"#.to_string());
            }
            Metric {
                name: Some(name), ..
            } if name == "totalUsers" => {
                select_clauses.push(r#"COUNT(distinct user_pseudo_id) AS totalUsers"#.to_string());
            }
            Metric {
                name: Some(name), ..
            } if name == "newUsers" => {
                select_clauses.push(r#"COUNTIF(new_user) AS newUsers"#.to_string());
            }
            Metric {
                name: Some(name), ..
            } if name == "userEngagementDuration" => {
                select_clauses
                    .push(r#"SUM(engagement_time_msec) AS userEngagementDuration"#.to_string());
            }
            _ => panic!("Unsupported metric: {:?}", met),
        }
    }

    match dimension_filter {
        Some(FilterExpression {
            filter:
                Some(Filter {
                    field_name: Some(field_name),
                    string_filter:
                        Some(StringFilter {
                            match_type: Some(match_type),
                            value: Some(value),
                            case_sensitive: Some(false),
                        }),
                    ..
                }),
            ..
        }) => {
            let field_expr = FIELD_SUBSTITUTIONS
                .get(&field_name)
                .map(|x| x.to_string())
                .unwrap_or_else(|| field_name);

            match match_type.as_ref() {
                "BEGINS_WITH" => {
                    where_clauses.push(format!(
                        r#"STARTS_WITH(UPPER({}), UPPER("{}"))"#,
                        field_expr, value
                    ));
                }
                "EXACT" => {
                    where_clauses.push(format!(r#"UPPER({}) = UPPER("{}")"#, field_expr, value));
                }
                _ => {
                    panic!("Unsupported match type: {}", match_type);
                }
            }
        }
        Some(_) => {
            panic!("Unsupported filter: {:?}", dimension_filter);
        }
        None => {}
    }

    let (response, results) = hub
        .jobs()
        .query(
            QueryRequest {
                query: Some(format!(
                    r#"
WITH views AS (
  SELECT
    event_date,
    event_timestamp,
    user_pseudo_id,
    user_first_touch_timestamp,
    event_timestamp - user_first_touch_timestamp < 300000000 AS new_user,
    event_timestamp = user_first_touch_timestamp AS first_access,
    (select value.int_value from unnest(event_params) where key = "ga_session_id") AS session_id,
    (select value.int_value from unnest(event_params) where key = "engagement_time_msec") AS engagement_time_msec,
    (select value.string_value from unnest(event_params) where key = "activity_types") AS activity_types,
    (select value.string_value from unnest(event_params) where key = "domain") AS domain,
    (select value.string_value from unnest(event_params) where key = "medium") AS medium,
    (select value.string_value from unnest(event_params) where key = "page_location") AS page_location,
    (select value.string_value from unnest(event_params) where key = "partner_uid") AS partner_uid,
    (select value.string_value from unnest(event_params) where key = "campaign") AS campaign,
    (select value.string_value from unnest(event_params) where key = "page_referrer") AS page_referrer,
    (select value.string_value from unnest(event_params) where key = "source") AS source,
    (select value.string_value from unnest(event_params) where key = "entity_uid") AS entity_uid,
    (select value.string_value from unnest(event_params) where key = "term") AS term,
    device.category AS device_category,
    device.operating_system AS device_operating_system,
    device.language AS device_language,
    geo.country AS geo_country,
    geo.region AS geo_region,
    geo.city AS geo_city
  FROM `analytics_322158266.*`
  WHERE event_name = "page_view"
)
SELECT {} FROM views WHERE {} GROUP BY {}
"#,
                    select_clauses.join(", "),
                    where_clauses.join(" AND "),
                    group_clauses.join(", "),
                )),
                query_parameters: Some(params),
                parameter_mode: Some("NAMED".into()),
                timeout_ms: Some(60000),
                use_legacy_sql: Some(false),
                ..Default::default()
            },
            "snm-analytics-ac-1669127864474",
        )
        .doit()
        .await?;

    BQReportIterator::new(results)
}

pub async fn cache_report(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    filter: FilterExpression,
    temporary: bool,
) {
    for row in match run_report(
        begin,
        end,
        Some(filter),
        vec![
            Dimension {
                name: Some(String::from("customEvent:entity_uid")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("customEvent:partner_uid")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("city")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("date")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("deviceCategory")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("firstSessionDate")),
                ..Default::default()
            },
            // Primarily sourced from UTM params
            Dimension {
                name: Some(String::from("sessionDefaultChannelGroup")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("pagePath")),
                ..Default::default()
            },
            Dimension {
                name: Some(String::from("region")),
                ..Default::default()
            },
        ],
        vec![
            Metric {
                name: Some(String::from("screenPageViews")),
                ..Default::default()
            },
            Metric {
                name: Some(String::from("sessions")),
                ..Default::default()
            },
            Metric {
                name: Some(String::from("eventCount")),
                ..Default::default()
            },
            Metric {
                name: Some(String::from("totalUsers")),
                ..Default::default()
            },
            Metric {
                name: Some(String::from("newUsers")),
                ..Default::default()
            },
            Metric {
                name: Some(String::from("userEngagementDuration")),
                ..Default::default()
            },
        ],
    )
    .await
    {
        Ok(iter) => iter,
        Err(err) => {
            println!("Error attempting to run GA4 report: {:?}", err);
            return;
        }
    } {
        let Ok(date) = row.get_date("date") else { println!("Unable to parse date in GA4 response: {:?}", row); continue; };
        let Ok(partner) = row.get_uuid("customEvent:partner_uid") else { println!("Unable to parse partner uid: {:?}", row); continue };
        let Ok(opportunity) = row.get_uuid("customEvent:entity_uid") else { println!("Unable to parse entity uid: {:?}", row); continue };
        let city = row.get_string("city");
        let device_category = row.get_string("deviceCategory");
        let first_session_date = row
            .get_date("firstSessionDate")
            .unwrap_or_else(|_| Utc::now().to_fixed_offset());
        let session_channel_group = row.get_string("sessionDefaultChannelGroup");
        let page_path = row.get_string("pagePath");
        let page_referrer = row.get_string("pageReferrer");
        let region = row.get_string("region");
        let views = row.get_int("screenPageViews");
        let sessions = row.get_int("sessions");
        let events = row.get_int("eventCount");
        let total_users = row.get_int("totalUsers");
        let new_users = row.get_int("newUsers");
        let engagement_duration = row.get_float("userEngagementDuration");

        if let Err(err) = sqlx::query!(
            r#"
INSERT INTO c_analytics_cache (
    "temporary",
    "begin",
    "end",
    "opportunity",
    "partner",
    "date",
    "city",
    "device_category",
    "first_session_date",
    "page_path",
    "region",
    "views",
    "events",
    "total_users",
    "new_users",
    "engagement_duration",
    "sessions",
    "session_channel_group",
    "page_referrer"
)
VALUES (
  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19
)
"#,
            temporary,
            begin,
            end,
            opportunity,
            partner,
            date,
            city,
            device_category,
            first_session_date,
            page_path,
            region,
            views,
            events,
            total_users,
            new_users,
            engagement_duration,
            sessions,
            session_channel_group,
            page_referrer,
        )
        .execute(db)
        .await
        {
            println!("Error inserting into c_analytics_cache: {:?}", err);
        }
    }
}
