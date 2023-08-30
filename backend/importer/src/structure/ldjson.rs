use std::collections::HashMap;

use chrono::TimeZone;
use common::{
    geo::Match,
    model::{
        partner::{LoggedError, LoggedErrorLevel},
        Opportunity, Partner,
    },
    ToFixedOffset,
};
use htmlentity::entity::ICodedDataTrait;
use serde_json::{json, Value};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::Error;

use super::{OneOrMany, PartnerInfo, Structure};

#[derive(Debug)]
pub struct LdJson<Tz: TimeZone>(pub String, pub PartnerInfo<Tz>);

#[derive(serde::Deserialize, Debug)]
enum SchemaOrgContext {
    #[serde(rename = "http://schema.org", alias = "https://schema.org")]
    SchemaOrg,
}

#[derive(serde::Deserialize, Debug)]
enum SchemaOrgType {
    Event,
}

#[derive(serde::Deserialize, Debug)]
enum SchemaOrgLocationType {
    Place,
}

#[derive(serde::Deserialize, Debug)]
enum SchemaOrgOrganizerType {
    Person,
}

#[derive(serde::Deserialize, Debug)]
enum SchemaOrgEventStatus {
    #[serde(
        rename = "http://schema.org/EventCancelled",
        alias = "https://schema.org/EventCancelled"
    )]
    Cancelled,
    #[serde(
        rename = "http://schema.org/EventMovedOnline",
        alias = "https://schema.org/EventMovedOnline"
    )]
    MovedOnline,
    #[serde(
        rename = "http://schema.org/EventPostponed",
        alias = "https://schema.org/EventPostponed"
    )]
    Postponed,
    #[serde(
        rename = "http://schema.org/EventRescheduled",
        alias = "https://schema.org/EventRescheduled"
    )]
    Rescheduled,
    #[serde(
        rename = "http://schema.org/EventScheduled",
        alias = "https://schema.org/EventScheduled"
    )]
    Scheduled,
}

#[derive(serde::Deserialize, Debug)]
enum SchemaOrgEventAttendanceMode {
    #[serde(
        rename = "http://schema.org/OnlineEventAttendanceMode",
        alias = "https://schema.org/OnlineEventAttendanceMode"
    )]
    Online,
    #[serde(
        rename = "http://schema.org/OfflineEventAttendanceMode",
        alias = "https://schema.org/OfflineEventAttendanceMode"
    )]
    Offline,
    #[serde(
        rename = "http://schema.org/MixedEventAttendanceMode",
        alias = "https://schema.org/MixedEventAttendanceMode"
    )]
    Mixed,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SchemaOrgLocation {
    #[serde(rename = "@type")]
    schema_type: SchemaOrgLocationType,
    name: String,
    #[serde(default, rename = "image")]
    _image: String,
    address: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SchemaOrgOrganizer {
    #[serde(rename = "@type")]
    _schema_type: SchemaOrgOrganizerType,
    name: String,
    url: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Offer {
    #[serde(default, rename = "url")]
    _url: String,
    price: String,
    #[serde(default, rename = "price_currency")]
    _price_currency: String,
    #[serde(default, rename = "availability")]
    _availability: String,
    #[serde(default, rename = "valid_from")]
    _valid_from: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SchemaOrgEvent {
    #[serde(rename = "@context")]
    _schema_context: SchemaOrgContext,
    #[serde(rename = "@type")]
    _schema_type: SchemaOrgType,
    event_status: SchemaOrgEventStatus,
    start_date: String,
    end_date: String,
    event_attendance_mode: SchemaOrgEventAttendanceMode,
    location: SchemaOrgLocation,
    organizer: SchemaOrgOrganizer,
    offers: Offer,
    #[serde(default, rename = "performer")]
    _performer: String,
    description: String,
    image: String,
    name: String,
    url: String,
}

impl<Tz> LdJson<Tz>
where
    Tz: TimeZone + std::fmt::Debug + Sync + Send,
{
    fn interpret_one(
        &self,
        json: Value,
        cache: &mut HashMap<String, Option<Match>>,
    ) -> Result<Opportunity, LoggedError> {
        let data: SchemaOrgEvent = serde_json::from_value(json)?;

        let mut opp = Opportunity::default();

        opp.exterior.uid = Uuid::new_v5(&self.1.partner, data.url.as_bytes());

        opp.exterior.title = htmlentity::entity::decode(data.name.as_bytes())
            .to_string()
            .unwrap_or_default();

        opp.exterior.description = htmlentity::entity::decode(data.description.as_bytes())
            .to_string()
            .unwrap_or_default();

        opp.exterior.partner_opp_url = if !data.url.is_empty() {
            Some(data.url)
        } else {
            None
        };

        opp.exterior.image_url = data.image;

        opp.exterior.cost = match data.offers.price.as_ref() {
            "Free" | "free" => common::model::opportunity::Cost::Free,
            "$0" | "$0.00" => common::model::opportunity::Cost::Free,
            _ => common::model::opportunity::Cost::Cost,
        };

        opp.exterior.organization_name = htmlentity::entity::decode(data.organizer.name.as_bytes())
            .to_string()
            .unwrap_or_default();

        opp.exterior.organization_website = if !data.organizer.url.is_empty() {
            Some(data.organizer.url)
        } else {
            None
        };

        opp.exterior.min_age = 0;
        opp.exterior.max_age = 999;

        opp.exterior.opp_venue = vec![common::model::opportunity::VenueType::Indoors];

        match data.location.schema_type {
            SchemaOrgLocationType::Place => {
                opp.exterior.location_type = common::model::opportunity::LocationType::At;
                opp.exterior.location_name = data.location.name;

                println!("Looking up: {}", &data.location.address);
                if let Some(loc) = match cache.get(&data.location.address) {
                    Some(val) => val.as_deref(),
                    None => {
                        let query = common::geo::Query::new(data.location.address.clone(), true);
                        cache.insert(
                            data.location.address.clone(),
                            async_std::task::block_on(query.lookup_one()),
                        );
                        cache
                            .get(&data.location.address)
                            .map(|v| v.as_deref())
                            .flatten()
                    }
                } {
                    println!(
                        "Location: {}, {}",
                        loc.geometry.longitude, loc.geometry.latitude
                    );
                    opp.exterior.location_point = Some(
                        json!({"type": "Point", "coordinates": [loc.geometry.longitude, loc.geometry.latitude]}),
                    );
                    opp.exterior.address_street = loc
                        .formatted
                        .split_once(',')
                        .unwrap_or(("", ""))
                        .0
                        .to_owned();
                    opp.exterior.address_city = loc
                        .components
                        .city
                        .clone()
                        .or_else(|| loc.components.town.clone())
                        .unwrap_or_default();
                    opp.exterior.address_state = loc.components.state.clone().unwrap_or_default();
                    opp.exterior.address_country = loc.components.country_code.clone();
                    opp.exterior.address_zip = loc.components.postcode.clone().unwrap_or_default();
                }
            }
        };

        match data.event_attendance_mode {
            SchemaOrgEventAttendanceMode::Online => {
                opp.exterior.is_online = true;
                opp.exterior.location_type = common::model::opportunity::LocationType::Any;
            }
            SchemaOrgEventAttendanceMode::Offline => opp.exterior.is_online = false,
            SchemaOrgEventAttendanceMode::Mixed => {
                opp.exterior.is_online = true;
            }
        }

        if let Some(tz) = &self.1.timezone {
            opp.exterior.start_datetimes = vec![tz
                .from_local_datetime(
                    &data
                        .start_date
                        .parse::<chrono::NaiveDate>()?
                        .and_hms_opt(0, 0, 0)
                        .expect("00:00:00 should always be valid"),
                )
                .earliest()
                .ok_or_else(|| {
                    LoggedError::new(LoggedErrorLevel::Error, "Unable to determine start date")
                })?
                .to_fixed_offset()];
            opp.exterior.end_datetimes = vec![tz
                .from_local_datetime(
                    &data
                        .end_date
                        .parse::<chrono::NaiveDate>()?
                        .and_hms_opt(23, 59, 59)
                        .expect("11:59:59 should always be valid"),
                )
                .earliest()
                .ok_or_else(|| {
                    LoggedError::new(LoggedErrorLevel::Error, "Unable to determine end date")
                })?
                .to_fixed_offset()];
        } else {
            return Err(LoggedError::new(
                LoggedErrorLevel::Debug,
                "Partner time zone is not set, so the start and end datetimes would be ambiguous",
            ));
        }

        opp.exterior.partner = self.1.partner;

        opp.exterior.partner_name = self.1.partner_name.clone();

        opp.exterior.partner_website = self.1.partner_website.clone();

        opp.exterior.partner_logo_url = self.1.partner_logo_url.clone();

        opp.exterior.pes_domain = self.1.domain;

        opp.exterior.opp_descriptor = self.1.descriptor.clone();

        opp.exterior.opp_topics = self.1.topics.clone();

        opp.interior.withdrawn = match data.event_status {
            SchemaOrgEventStatus::Cancelled => true,
            SchemaOrgEventStatus::MovedOnline => false,
            SchemaOrgEventStatus::Postponed => true,
            SchemaOrgEventStatus::Rescheduled => false,
            SchemaOrgEventStatus::Scheduled => false,
        };

        Ok(opp)
    }
}

#[async_trait::async_trait]
impl<Tz> Structure for LdJson<Tz>
where
    Tz: TimeZone + std::fmt::Debug + Sync + Send,
{
    type Data = Opportunity;

    fn interpret(&self, parsed: Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        let mut cache = HashMap::new();

        if parsed[&self.0].is_array() {
            OneOrMany::Many(
                parsed[&self.0]
                    .as_array()
                    .unwrap()
                    .iter()
                    .cloned()
                    .map(|json| self.interpret_one(json, &mut cache))
                    .collect(),
            )
        } else if parsed[&self.0].is_object() {
            OneOrMany::One(self.interpret_one(parsed[&self.0].clone(), &mut cache))
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
