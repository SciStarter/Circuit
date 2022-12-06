use std::collections::BTreeMap;
use std::env;

use anyhow::{anyhow, Error};
use chrono::{DateTime, Days, FixedOffset, LocalResult, NaiveDate, Utc};
use common::ToFixedOffset;
use google_analyticsdata1_beta::api::{
    DateRange, Dimension, FilterExpression, Metric, RunReportRequest,
};
use google_analyticsdata1_beta::{hyper, hyper_rustls, oauth2, AnalyticsData};

use crate::reportiter::ReportIterator;

// Need to rate limit GA4 requests based on
// property_quota.tokens_per_hour and tokens_per_day in response data

pub fn get_date(
    record: &BTreeMap<String, Option<String>>,
    key: &str,
) -> Result<DateTime<FixedOffset>, anyhow::Error> {
    let Some(Some(date)) = record.get(key) else { return Err(anyhow!("Field is missing or empty: {}", key)); };
    let Ok(date) = NaiveDate::parse_from_str(&*date, "%Y%m%d") else { return Err(anyhow!("Unparsable date: {}", date)); };
    let Some(date) = date.and_hms_opt(12, 0, 0) else { return Err(anyhow!("Out of bounds date: {}", date)); };
    let LocalResult::Single(date) = date.and_local_timezone(Utc) else { return Err(anyhow!("Failed setting timezone for date")); };
    Ok(date.to_fixed_offset())
}

pub fn get_int(record: &BTreeMap<String, Option<String>>, key: &str) -> u64 {
    let Some(Some(val)) = record.get(key) else { return 0 };
    let Ok(val) = val.parse() else { return 0 };
    val
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
        Ok(ReportIterator::new(data))
    } else {
        Err(anyhow!(format!(
            "GA4 request received response {}: {:?}",
            response.status(),
            response.body()
        )))
    }
}
