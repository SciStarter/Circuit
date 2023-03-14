use chrono::TimeZone;
use common::model::{
    partner::{LoggedError, LoggedErrorLevel},
    Opportunity, Partner,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};

use crate::Error;

use super::{OneOrMany, PartnerInfo, Structure};

#[derive(Debug)]
pub struct LdJson<Tz: TimeZone>(pub String, pub PartnerInfo<Tz>);

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SchemaOrgLocation {
    #[serde(rename = "@type")]
    schema_type: String,
    image: String,
    address: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SchemaOrgPerson {
    #[serde(rename = "@type")]
    schema_type: String,
    name: String,
    url: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Offer {
    url: String,
    price: String,
    price_currency: String,
    availability: String,
    valid_from: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SchemaOrgEvent {
    #[serde(rename = "@context")]
    schema_context: String,
    #[serde(rename = "@type")]
    schema_type: String,
    event_status: String,
    start_date: String,
    end_date: String,
    event_attendance_mode: String,
    location: SchemaOrgLocation,
    organizer: SchemaOrgPerson,
    offers: Offer,
    performer: String,
    description: String,
    image: String,
    name: String,
    url: String,
}

impl<Tz> LdJson<Tz>
where
    Tz: TimeZone + std::fmt::Debug + Sync + Send,
{
    fn interpret_one(&self, mut json: Value) -> Result<Opportunity, LoggedError> {
        let data: SchemaOrgEvent = serde_json::from_value(json)?;
        todo!()
    }
}

#[async_trait::async_trait]
impl<Tz> Structure for LdJson<Tz>
where
    Tz: TimeZone + std::fmt::Debug + Sync + Send,
{
    type Data = Opportunity;

    fn interpret(&self, mut parsed: Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        if parsed[&self.0].is_array() {
            OneOrMany::Many(
                parsed[&self.0]
                    .as_array()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|json| self.interpret_one(json))
                    .collect(),
            )
        } else if parsed[&self.0].is_object() {
            OneOrMany::One(self.interpret_one(parsed[&self.0].clone()))
        } else {
            OneOrMany::One(Err(LoggedError::new(
                LoggedErrorLevel::Error,
                format!(
                    "Expected either an object or an array of objects in the `{}` field",
                    self.0
                ),
            )))
        }
    }

    async fn load_partner(&self, db: &Pool<Postgres>) -> Result<Partner, Error> {
        self.1.load_partner(db).await
    }
}
