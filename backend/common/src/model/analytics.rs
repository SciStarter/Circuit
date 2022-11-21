use sqlx::types::chrono::{DateTime, FixedOffset};
use uuid::Uuid;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct OpportunityAnalyticsFrame {
    uid: Uuid,
    begin: DateTime<FixedOffset>,
    end: Option<DateTime<FixedOffset>>,
    views: i64,
    unique: i64,
    clicks: i64,
}
