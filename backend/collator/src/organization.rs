use anyhow::Error;
use chrono::{DateTime, FixedOffset};
use common::{model::analytics::Organization, Database};

use crate::CommonState;

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
    state: &CommonState,
) -> Result<Organization, Error> {
    Ok(Organization::default())
}
