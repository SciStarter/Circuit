use super::{Error, OneOrMany, PartnerInfo, Structure};
use chrono::{DateTime, TimeZone};
use common::model::{opportunity::EntityType, partner::LoggedError, Opportunity, Partner};
use common::ToFixedOffset;
use htmlentity::entity::ICodedDataTrait;
use serde_json::Value;
use sqlx::{Pool, Postgres};

// https://scitechinstitute.org/wp-json/mecexternal/v1/calendar/1

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
pub struct MecFields {
    #[serde(rename = "0")]
    zero: Vec<String>,
    #[serde(rename = "1")]
    one: Vec<String>,
    #[serde(rename = "2")]
    two: Vec<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Meta {
    mec_cost: String,
    mec_fields: MecFields,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct FeaturedImage {
    large: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Post {
    post_date_gmt: String,
    post_modified_gmt: String,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Event {
    title: String,
    content: String,
    post: Post,
    meta: Meta,
    permalink: String,
    featured_image: FeaturedImage,
    locations: Value,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Moment {
    #[serde(default, rename = "date")]
    _date: String,
    #[serde(default, rename = "hour")]
    _hour: String,
    #[serde(default, rename = "minutes")]
    _minutes: String,
    #[serde(default, rename = "ampm")]
    _ampm: String,
    timestamp: i64,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct StartAndEnd {
    start: Moment,
    end: Moment,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Entry {
    #[serde(default, rename = "ID")]
    _id: u64,
    data: Event,
    date: StartAndEnd,
}

#[derive(Debug)]
pub struct ModernEventsCalendar<Tz: TimeZone>(pub PartnerInfo<Tz>);

#[async_trait::async_trait]
impl<Tz> Structure for ModernEventsCalendar<Tz>
where
    Tz: TimeZone + std::fmt::Debug + Sync + Send,
{
    type Data = Opportunity;

    fn interpret(&self, parsed: Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        if let Some(days) = parsed["content_json"].as_object() {
            let mut opps = Vec::new();

            for (_, day) in days.into_iter() {
                for opp in match day.as_array().ok_or_else(|| {
                    Error::Data(String::from(
                        "content_json does not follow the expected structure",
                    ))
                }) {
                    Ok(array) => array,
                    Err(err) => return OneOrMany::One(Err(err.into())),
                }
                .into_iter()
                {
                    opps.push(
                        if let Ok(entry) = serde_json::from_value::<Entry>(opp.clone()) {
                            let mut opp = Opportunity::default();

                            opp.exterior.uid = uuid::Uuid::new_v5(
                                &self.0.partner,
                                entry.data.permalink.as_bytes(),
                            );

                            opp.exterior.partner = self.0.partner.clone();

                            opp.exterior.partner_name = self.0.partner_name.clone();

                            opp.exterior.partner_website = self.0.partner_website.clone();

                            opp.exterior.partner_logo_url = self.0.partner_logo_url.clone();

                            opp.exterior.partner_created =
                                match entry.data.post.post_date_gmt.split_once(' ') {
                                    Some(pair) => match DateTime::parse_from_rfc3339(&format!(
                                        "{}T{}Z",
                                        pair.0, pair.1
                                    )) {
                                        Ok(dt) => Some(dt),
                                        Err(_) => None,
                                    },
                                    None => None,
                                };

                            opp.exterior.partner_updated =
                                match entry.data.post.post_modified_gmt.split_once(' ') {
                                    Some(pair) => match DateTime::parse_from_rfc3339(&format!(
                                        "{}T{}Z",
                                        pair.0, pair.1
                                    )) {
                                        Ok(dt) => Some(dt),
                                        Err(_) => None,
                                    },
                                    None => None,
                                };

                            opp.exterior.partner_opp_url = Some(entry.data.permalink);

                            opp.exterior.organization_name = self.0.partner_name.clone();

                            opp.exterior.organization_website = self.0.partner_website.clone();

                            opp.interior.contact_email = String::new();

                            opp.interior.contact_phone = String::new();

                            opp.exterior.entity_type = EntityType::Opportunity;

                            opp.exterior.pes_domain = self.0.domain.clone();

                            opp.exterior.tags = entry
                                .data
                                .meta
                                .mec_fields
                                .zero
                                .into_iter()
                                .chain(entry.data.meta.mec_fields.one.into_iter())
                                .chain(entry.data.meta.mec_fields.two.into_iter())
                                .filter(|x| !x.is_empty())
                                .collect();

                            opp.exterior.title =
                                htmlentity::entity::decode(entry.data.title.as_bytes())
                                    .to_string()
                                    .unwrap_or_default();

                            opp.exterior.description =
                                htmlentity::entity::decode(entry.data.content.as_bytes())
                                    .to_string()
                                    .unwrap_or_default();

                            opp.exterior.image_url = entry.data.featured_image.large;

                            if let Some(tz) = &self.0.timezone {
                                // add 7 hours (25200 seconds) to the
                                // timestamp since for reasons unknown it
                                // is offset by that much in the SciTech
                                // Institute data.
                                opp.exterior.start_datetimes = vec![tz
                                    .timestamp_opt(entry.date.start.timestamp + 25200, 0)
                                    .earliest()
                                    .expect("Start datetime should be within DateTime domain")
                                    .to_fixed_offset()];

                                opp.exterior.end_datetimes = vec![tz
                                    .timestamp_opt(entry.date.end.timestamp + 25200, 0)
                                    .earliest()
                                    .expect("End datetime should be within DateTime domain")
                                    .to_fixed_offset()];

                                opp.exterior.has_end = true;
                            }

                            opp.exterior.attraction_hours = None;

                            opp.exterior.cost = match (
                                self.0.flags.contains(&crate::structure::PartnerFlag::Cost),
                                entry.data.meta.mec_cost.as_ref(),
                            ) {
                                (true, _) => common::model::opportunity::Cost::Cost,
                                (false, "Free") => common::model::opportunity::Cost::Free,
                                _ => common::model::opportunity::Cost::Cost,
                            };

                            opp.exterior.opp_venue =
                                vec![common::model::opportunity::VenueType::Indoors];

                            opp.exterior.opp_descriptor = self.0.descriptor.clone();

                            opp.exterior.min_age = 0;

                            opp.exterior.max_age = 999;

                            opp.exterior.ticket_required = self
                                .0
                                .flags
                                .contains(&crate::structure::PartnerFlag::Ticketed);

                            opp.exterior.languages = vec!["en".to_string()];

                            opp.exterior.is_online = false;

                            if let Some(addr) = entry
                                .data
                                .locations
                                .as_object()
                                .map(|x| x.values().next())
                                .flatten()
                            {
                                let name = &addr["name"];
                                let street = &addr["address"];

                                if name.is_string() || street.is_string() {
                                    opp.exterior.location_type =
                                        common::model::opportunity::LocationType::At;
                                    opp.exterior.location_name =
                                        name.as_str().unwrap_or("").to_string();
                                    opp.exterior.address_street =
                                        street.as_str().unwrap_or("").to_string();
                                }
                            }

                            Ok(opp)
                        } else {
                            Err(LoggedError::default().set_raw(opp.to_string()))
                        },
                    )
                }
            }

            OneOrMany::Many(opps)
        } else {
            OneOrMany::One(Err(Error::Data(String::from(
                "content_json missing or incorrect type",
            ))
            .into()))
        }
    }

    async fn load_partner(&self, db: &Pool<Postgres>) -> Result<Partner, Error> {
        self.0.load_partner(db).await
    }
}
