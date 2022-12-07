use chrono::{DateTime, FixedOffset};
use common::Database;

pub async fn cache(
    db: &Database,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    Ok(())
}

pub async fn collect(
    db: &Database,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<Opportunity, Error> {
    Ok(())
}
