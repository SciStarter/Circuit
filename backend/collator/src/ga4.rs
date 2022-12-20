use std::env;
use std::time::Duration;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Days, FixedOffset};
use common::Database;
use google_analyticsdata1_beta::api::{
    DateRange, Dimension, FilterExpression, Metric, RunReportRequest, RunReportResponse,
};
use google_analyticsdata1_beta::{hyper, hyper_rustls, oauth2, AnalyticsData};
use uuid::Uuid;

use crate::reportiter::ReportIterator;

const HOURLY_SECONDS_PER_TOKEN: f32 = (60.0 * 60.0) / 5000.0;
const DAILY_SECONDS_PER_TOKEN: f32 = (60.0 * 60.0 * 24.0) / 25000.0;

pub async fn run_report(
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    dimension_filter: Option<FilterExpression>,
    dimensions: Vec<Dimension>,
    metrics: Vec<Metric>,
) -> Result<ReportIterator, Error> {
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

    let hub = AnalyticsData::new(
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

    let limit = 100000;
    let mut offset = 0;

    let mut req = RunReportRequest {
        date_ranges: Some(vec![DateRange {
            start_date: Some(begin.to_string()),
            end_date: Some(end.to_string()),
            name: Some(String::from("date_range")),
        }]),
        dimension_filter,
        dimensions: if !dimensions.is_empty() {
            Some(dimensions)
        } else {
            None
        },
        metrics: if !metrics.is_empty() {
            Some(metrics)
        } else {
            None
        },
        return_property_quota: Some(true),
        limit: Some(limit.to_string()),
        offset: Some(offset.to_string()),
        ..Default::default()
    };

    let mut cumulative: Option<RunReportResponse> = None;
    let mut has_more = true;

    while has_more {
        let (response, mut data) = hub
            .properties()
            .run_report(req.clone(), "properties/322158266")
            .doit()
            .await?;

        if response.status() == 200 {
            if let Some((day_consumed, hour_consumed)) = data.property_quota.as_ref().map(|x| {
                (
                    x.tokens_per_day
                        .as_ref()
                        .map(|d| d.consumed)
                        .flatten()
                        .unwrap_or(0),
                    x.tokens_per_hour
                        .as_ref()
                        .map(|h| h.consumed)
                        .flatten()
                        .unwrap_or(0),
                )
            }) {
                let delay = ((day_consumed as f32) * DAILY_SECONDS_PER_TOKEN)
                    .max((hour_consumed as f32) * HOURLY_SECONDS_PER_TOKEN);

                // Throttle requests to fit the limits imposed by
                // Google. Note that this simple approach will not work if
                // the requests are being run in parallel.
                tokio::time::sleep(Duration::from_secs_f32(delay)).await;
            }
        } else {
            return Err(anyhow!(format!(
                "GA4 request received response {}: {:?}",
                response.status(),
                response.body()
            )));
        }

        if let Some(cum) = &mut cumulative {
            if let Some(cum_rows) = &mut cum.rows {
                if let Some(data_rows) = &mut data.rows {
                    cum_rows.append(data_rows);
                }
            } else {
                cum.rows = data.rows.take();
            }
        } else {
            cumulative = Some(data);
        }

        has_more = match &cumulative {
            Some(RunReportResponse {
                rows: Some(rows),
                row_count: Some(row_count),
                ..
            }) if rows.len() < (*row_count).try_into().unwrap_or(0) => true,
            _ => false,
        };

        if has_more {
            offset += limit;
            req.offset = Some(offset.to_string());
        }
    }

    if let Some(data) = cumulative {
        Ok(ReportIterator::new(dbg!(data)))
    } else {
        Err(anyhow!("No response received from GA4"))
    }
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
        let Ok(first_session_date) = row.get_date("firstSessionDate") else { println!("Unable to parse first session date in GA4 response: {:?}", row); continue; };
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

pub async fn is_cached(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    opp: Uuid,
) -> bool {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM c_analytics_cache WHERE "begin" = $1 AND "end" = $2 AND "opportunity" = $3) AS "exists!: bool""#,
        begin,
        end,
        opp
    )
    .fetch_one(db)
    .await.unwrap_or(false)
}

pub async fn is_overview_cached(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> bool {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM c_analytics_overview_cache WHERE "begin" = $1 AND "end" = $2) AS "exists!: bool""#,
        begin,
        end,
    )
    .fetch_one(db)
    .await.unwrap_or(false)
}

pub async fn is_search_terms_cached(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> bool {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM c_analytics_search_term_cache WHERE "begin" = $1 AND "end" = $2) AS "exists!: bool""#,
        begin,
        end,
    )
    .fetch_one(db)
    .await.unwrap_or(false)
}

pub async fn clear_cached_temporary(db: &Database) -> Result<(), Error> {
    sqlx::query!(r#"DELETE FROM c_analytics_cache WHERE "temporary" = true"#)
        .execute(db)
        .await?;
    sqlx::query!(r#"DELETE FROM c_analytics_overview_cache WHERE "temporary" = true"#)
        .execute(db)
        .await?;
    sqlx::query!(r#"DELETE FROM c_analytics_search_term_cache WHERE "temporary" = true"#)
        .execute(db)
        .await?;
    Ok(())
}
