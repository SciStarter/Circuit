use std::collections::BTreeMap;
use std::env;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Days, FixedOffset, LocalResult, NaiveDate, Utc};
use common::{Database, ToFixedOffset};
use google_analyticsdata1_beta::api::{
    DateRange, Dimension, FilterExpression, Metric, RunReportRequest,
};
use google_analyticsdata1_beta::{hyper, hyper_rustls, oauth2, AnalyticsData};
use uuid::Uuid;

use crate::reportiter::ReportIterator;

// Need to rate limit GA4 requests based on
// property_quota.tokens_per_hour and tokens_per_day in response data

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

    let req = RunReportRequest {
        date_ranges: Some(vec![DateRange {
            start_date: Some(begin.to_string()),
            end_date: Some(end.to_string()),
            name: Some(String::from("date_range")),
        }]),
        dimension_filter: Some(dimension_filter),
        dimensions: Some(vec![
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
        ..Default::default()
    };

    let (response, data) = hub
        .properties()
        .run_report(req, "properties/322158266")
        .doit()
        .await?;

    if response.status() == 200 {
        Ok(ReportIterator::new(dbg!(data)))
    } else {
        Err(anyhow!(format!(
            "GA4 request received response {}: {:?}",
            response.status(),
            response.body()
        )))
    }
}

pub async fn cache_report(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    filter: FilterExpression,
    about: Uuid,
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
    "about",
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
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
ON CONFLICT ("begin", "end", "about")
DO UPDATE SET
    "temporary" = EXCLUDED."temporary",
    "date" = EXCLUDED."date",
    "city" = EXCLUDED."city",
    "device_category" = EXCLUDED."device_category",
    "first_session_date" = EXCLUDED."first_session_date",
    "page_path" = EXCLUDED."page_path",
    "region" = EXCLUDED."region",
    "views" = EXCLUDED."views",
    "events" = EXCLUDED."events",
    "total_users" = EXCLUDED."total_users",
    "new_users" = EXCLUDED."new_users",
    "engagement_duration" = EXCLUDED."engagement_duration",
    "sessions" = EXCLUDED."sessions",
    "session_channel_group" = EXCLUDED."session_channel_group",
    "page_referrer" = EXCLUDED."page_referrer"
"#,
            temporary,
            begin,
            end,
            about,
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
    about: Uuid,
) -> bool {
    sqlx::query_scalar!(
        r#"SELECT EXISTS(SELECT 1 FROM c_analytics_cache WHERE "begin" = $1 AND "end" = $2 AND "about" = $3) AS "exists!: bool""#,
        begin,
        end,
        about
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
