use std::collections::BTreeMap;
use std::env;
use std::time::Duration;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Days, FixedOffset, LocalResult, NaiveDate, Utc};
use common::{Database, ToFixedOffset};
use google_analyticsdata1_beta::api::{
    DateRange, Dimension, FilterExpression, Metric, RunReportRequest, RunReportResponse,
};
use google_analyticsdata1_beta::{hyper, hyper_rustls, oauth2, AnalyticsData};
use uuid::Uuid;

use crate::reportiter::ReportIterator;

const HOURLY_SECONDS_PER_TOKEN: f32 = (60.0 * 60.0) / 5000.0;
const DAILY_SECONDS_PER_TOKEN: f32 = (60.0 * 60.0 * 24.0) / 25000.0;

fn get_date(
    record: &BTreeMap<String, Option<String>>,
    key: &str,
) -> Result<DateTime<FixedOffset>, Error> {
    let Some(Some(date)) = record.get(key) else { return Err(anyhow!("Field is missing or empty: {}", key)); };
    let Ok(date) = NaiveDate::parse_from_str(&*date, "%Y%m%d") else { return Err(anyhow!("Unparsable date: {}", date)); };
    let Some(date) = date.and_hms_opt(12, 0, 0) else { return Err(anyhow!("Out of bounds date: {}", date)); };
    let LocalResult::Single(date) = date.and_local_timezone(Utc) else { return Err(anyhow!("Failed setting timezone for date")); };
    Ok(date.to_fixed_offset())
}

fn get_uuid(record: &BTreeMap<String, Option<String>>, key: &str) -> Result<Uuid, Error> {
    let Some(Some(uuid)) = record.get(key) else { return Err(anyhow!("Field is missing or empty: {}", key)); };
    Ok(Uuid::parse_str(&uuid)?)
}

fn get_int(record: &BTreeMap<String, Option<String>>, key: &str) -> i64 {
    let Some(Some(val)) = record.get(key) else { return 0 };
    let Ok(val) = val.parse() else { return 0 };
    val
}

fn get_float(record: &BTreeMap<String, Option<String>>, key: &str) -> f64 {
    let Some(Some(val)) = record.get(key) else { return 0.0 };
    let Ok(val) = val.parse() else { return 0.0 };
    val
}

fn get_string(record: &BTreeMap<String, Option<String>>, key: &str) -> String {
    let Some(Some(val)) = record.get(key) else { return String::new(); };
    val.clone()
}

pub async fn run_report(
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    dimension_filter: FilterExpression,
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
        dimension_filter: Some(dimension_filter),
        dimensions: Some(vec![
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
        ]),
        metrics: Some(vec![
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
        ]),
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

        has_more = dbg!(match &cumulative {
            Some(RunReportResponse {
                rows: Some(rows),
                row_count: Some(row_count),
                ..
            }) if rows.len() < (*row_count).try_into().unwrap_or(0) => true,
            _ => false,
        });

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
    for row in match run_report(begin, end, filter).await {
        Ok(iter) => iter,
        Err(err) => {
            println!("Error attempting to run GA4 report: {:?}", err);
            return;
        }
    } {
        let Ok(date) = get_date(&row, "date") else { println!("Unable to parse date in GA4 response: {:?}", row); continue; };
        let Ok(partner) = get_uuid(&row, "customEvent:partner_uid") else { println!("Unable to parse partner uid: {:?}", row); continue };
        let Ok(opportunity) = get_uuid(&row, "customEvent:entity_uid") else { println!("Unable to parse entity uid: {:?}", row); continue };
        let city = get_string(&row, "city");
        let device_category = get_string(&row, "deviceCategory");
        let Ok(first_session_date) = get_date(&row, "firstSessionDate") else { println!("Unable to parse first session date in GA4 response: {:?}", row); continue; };
        let session_channel_group = get_string(&row, "sessionDefaultChannelGroup");
        let page_path = get_string(&row, "pagePath");
        let page_referrer = get_string(&row, "pageReferrer");
        let region = get_string(&row, "region");
        let views = get_int(&row, "screenPageViews");
        let sessions = get_int(&row, "sessions");
        let events = get_int(&row, "eventCount");
        let total_users = get_int(&row, "totalUsers");
        let new_users = get_int(&row, "newUsers");
        let engagement_duration = get_float(&row, "userEngagementDuration");

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
    "page_referrer",

    "current_on_date"
)
VALUES (
  $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
  (
    SELECT c_opportunity_is_current_as_of(c_opportunity."interior", c_opportunity."exterior", $6)
    FROM c_opportunity
    WHERE (c_opportunity."exterior"->>'uid')::uuid = $4
    LIMIT 1
  )
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

pub async fn clear_cached_temporary(db: &Database) -> Result<(), Error> {
    sqlx::query!(r#"DELETE FROM c_analytics_cache WHERE "temporary" = true"#)
        .execute(db)
        .await?;
    Ok(())
}
