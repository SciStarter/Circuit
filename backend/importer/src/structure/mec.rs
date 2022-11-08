use super::{
    Error,
    OneOrMany::{self, Many},
    PartnerInfo, Structure,
};
use chrono::{DateTime, TimeZone, Utc};
use common::model::{opportunity::EntityType, Opportunity};
use common::ToFixedOffset;
use serde_json::Value;

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
    date: String,
    hour: String,
    minutes: String,
    ampm: String,
    timestamp: i64,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct StartAndEnd {
    start: Moment,
    end: Moment,
}

#[derive(serde::Deserialize, Debug, Default)]
pub struct Entry {
    #[serde(rename = "ID")]
    id: u64,
    data: Event,
    date: StartAndEnd,
}

#[derive(Debug)]
pub struct ModernEventsCalendar<Tz: TimeZone>(pub PartnerInfo<Tz>);

impl<Tz> Structure for ModernEventsCalendar<Tz>
where
    Tz: TimeZone + std::fmt::Debug,
{
    type Data = Opportunity;

    fn interpret(&self, parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Some(days) = parsed["content_json"].as_object() {
            let mut opps = Vec::new();

            for (_, day) in days.into_iter() {
                for opp in day
                    .as_array()
                    .ok_or_else(|| {
                        Error::Structure(String::from(
                            "content_json does not follow the expected structure",
                        ))
                    })?
                    .into_iter()
                {
                    if let Ok(entry) = serde_json::from_value::<Entry>(opp.clone()) {
                        let mut opp = Opportunity::default();

                        opp.exterior.uid =
                            uuid::Uuid::new_v5(&self.0.partner, entry.data.permalink.as_bytes());

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

                        opp.exterior.title = htmlentity::entity::decode(&entry.data.title)
                            .iter()
                            .collect();

                        opp.exterior.description = entry.data.content;

                        opp.exterior.image_url = entry.data.featured_image.large;

                        opp.exterior.start_datetimes = vec![Utc
                            .timestamp(entry.date.start.timestamp, 0)
                            .to_fixed_offset()];

                        opp.exterior.end_datetimes =
                            vec![Utc.timestamp(entry.date.end.timestamp, 0).to_fixed_offset()];

                        opp.exterior.has_end = true;

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

                        opps.push(opp)
                    }
                }
            }

            Ok(OneOrMany::Many(opps))
        } else {
            Err(Error::Structure(String::from(
                "content_json missing or incorrect type",
            )))
        }
    }
}
