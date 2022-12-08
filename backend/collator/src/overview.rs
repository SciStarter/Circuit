use anyhow::Error;
use chrono::{DateTime, FixedOffset};
use common::{model::analytics::Overview, Database};

use crate::CommonState;

pub async fn cache(
    db: &Database,
    temporary: bool,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
) -> Result<(), Error> {
    Ok(())
}

pub async fn collect(db: &Database, state: &CommonState) -> Result<Overview, Error> {
    Ok(Overview::default())
}
