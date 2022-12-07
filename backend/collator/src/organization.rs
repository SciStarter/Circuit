use chrono::{DateTime, FixedOffset};
use common::Database;

pub async fn cache(
    db: &Database,
    org: &common::model::Partner,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    Ok(())
}

pub async fn collect(
    db: &Database,
    org: &common::model::Partner,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<Opportunity, Error> {
    Ok(())
}
