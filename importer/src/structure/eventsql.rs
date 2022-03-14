use inflector::Inflector;

use super::{
    Error,
    OneOrMany::{self, Many},
    PartnerInfo, Structure,
};
use chrono::{DateTime, TimeZone, Utc};
use common::model::{opportunity::EntityType, Opportunity};
use common::ToFixedOffset;
use serde_json::Value;

#[derive(Debug)]
pub struct EventsQL<Tz: TimeZone>(pub PartnerInfo<Tz>);

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataOrganizer {
    name: String,
    url: Option<String>,
    email: Option<String>,
    telephone: Option<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataLocationAddress {
    street_address: Option<String>,
    address_locality: Option<String>,
    address_region: Option<String>,
    postal_code: Option<String>,
    address_country: Option<String>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataLocation {
    url: Option<String>,
    name: Option<String>,
    address: Option<DataLocationAddress>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataLinked {
    organizer: Option<DataOrganizer>,
    description: String,
    location: Option<DataLocation>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataTagNode {
    name: String,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataTag {
    node: DataTagNode,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataTags {
    edges: Vec<DataTag>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct DataCustom {
    indoors_outdoors: Option<Vec<common::model::opportunity::VenueType>>,
    descriptor: Vec<common::model::opportunity::Descriptor>,
    min_age: Option<i16>,
    max_age: Option<i16>,
    topic: Vec<common::model::opportunity::Topic>,
    ticket_required: Option<String>,
    short_description: Option<String>,
    language: Option<Vec<String>>,
    online: Option<bool>,
    social_media_hashtags: Option<String>,
    twitter: Option<String>,
    facebook: Option<String>,
    instagram: Option<String>,
    organization_type: Option<common::model::opportunity::OrganizationType>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
struct DataFeaturedImageValue {
    source_url: String,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
struct DataFeaturedImage {
    node: DataFeaturedImageValue,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default, rename_all = "camelCase")]
struct DataNode {
    id: String,
    guid: String,
    date_gmt: String,
    duration: Option<u32>,
    end_date: Option<String>,
    modified_gmt: String,
    link: String,
    linked_data: DataLinked,
    excerpt: Option<String>,
    tags: DataTags,
    title: Option<String>,
    slug: String,
    featured_image: Option<DataFeaturedImage>,
    start_dates: Option<Vec<String>>,
    cost: Option<String>,
    #[serde(rename = "scienceNearMeData")]
    custom: Option<DataCustom>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct Data {
    node: Option<DataNode>,
}

fn interpret_one<Tz: TimeZone>(partner: &PartnerInfo<Tz>, entry: Value) -> Option<Opportunity> {
    let dump = serde_json::to_string_pretty(&entry)
        .unwrap_or_else(|_| "[failed to serialize]".to_string());

    let data: Data = match serde_json::from_value(entry) {
        Ok(d) => d,
        Err(err) => {
            println!("{}", dump);
            println!("Warning: Entry could not be parsed: {:?}", err);
            return None;
        }
    };

    let mut opp = Opportunity::default();

    if let Some(node) = data.node {
        opp.exterior.uid = uuid::Uuid::new_v5(&partner.partner, node.guid.as_bytes());

        opp.exterior.partner = partner.partner.clone();

        opp.exterior.partner_name = partner.partner_name.clone();

        opp.exterior.partner_website = partner.partner_website.clone();

        opp.exterior.partner_logo_url = partner.partner_logo_url.clone();

        opp.exterior.partner_created =
            match DateTime::parse_from_rfc3339(&format!("{}Z", &node.date_gmt)) {
                Ok(dt) => Some(dt),
                Err(_) => None,
            };

        opp.exterior.partner_updated =
            match DateTime::parse_from_rfc3339(&format!("{}Z", &node.modified_gmt)) {
                Ok(dt) => Some(dt),
                Err(_) => None,
            };

        opp.exterior.partner_opp_url = Some(node.link);

        if let Some(org) = node.linked_data.organizer {
            opp.exterior.organization_name = org.name;

            opp.exterior.organization_website = org.url;

            opp.interior.contact_email = htmlentity::entity::decode(&org.email.unwrap_or_default())
                .into_iter()
                .collect();

            opp.interior.contact_phone = org.telephone.unwrap_or_default();
        }

        opp.exterior.entity_type = EntityType::Opportunity;

        opp.exterior.pes_domain = partner.domain.clone();

        opp.exterior.tags = node.tags.edges.into_iter().map(|x| x.node.name).collect();

        opp.exterior.title = node.title.unwrap_or_else(|| node.slug.to_title_case());

        opp.exterior.short_desc = node.excerpt.unwrap_or_default();
        opp.exterior.description = node.linked_data.description;

        opp.exterior.image_url = if let Some(img) = node.featured_image {
            img.node.source_url
        } else {
            String::new()
        };

        opp.exterior.image_credit = String::new();

        if let Some(start_dates) = node.start_dates {
            opp.exterior.start_datetimes = start_dates
                .into_iter()
                .flat_map(|s| {
                    if let Some(tz) = &partner.timezone {
                        match tz.datetime_from_str(&s, "%F %T") {
                            Ok(dt) => Some(dt.to_fixed_offset()),
                            Err(_) => None,
                        }
                    } else if let Some(pair) = s.split_once(' ') {
                        if let Ok(dt) = format!("{}T{}Z", pair.0, pair.1).parse::<DateTime<Utc>>() {
                            Some(dt.to_fixed_offset())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();
        } else {
            opp.exterior.start_datetimes =
                match DateTime::parse_from_rfc3339(&format!("{}Z", &node.date_gmt)) {
                    Ok(dt) => vec![dt],
                    Err(_) => vec![],
                };

            if let Some(end_date) = &node.end_date {
                if let Some(tz) = &partner.timezone {
                    opp.exterior.end_datetimes = match tz.datetime_from_str(&end_date, "%F %T") {
                        Ok(dt) => vec![dt.to_fixed_offset()],
                        Err(_) => vec![],
                    }
                } else if let Some(end_date) = &node.end_date {
                    if let Some(pair) = end_date.split_once(' ') {
                        opp.exterior.end_datetimes = match format!("{}T{}Z", pair.0, pair.1).parse()
                        {
                            Ok(dt) => vec![dt],
                            Err(_) => vec![],
                        };
                    }
                }
            }
        }

        if opp.exterior.end_datetimes.len() == 1 {
            if opp.exterior.start_datetimes.is_empty() {
                // Guess that the event started 8 hours before it ends
                opp.exterior
                    .start_datetimes
                    .push(opp.exterior.end_datetimes[0] - chrono::Duration::hours(8));
            }
            opp.exterior.has_end = true;
        } else if !opp.exterior.end_datetimes.is_empty() {
            opp.exterior.has_end = true;
        } else {
            opp.exterior.has_end = false;
        }

        opp.exterior.attraction_hours = None;

        opp.exterior.cost = if node.cost.is_some() {
            common::model::opportunity::Cost::Cost
        } else {
            common::model::opportunity::Cost::Free
        };

        if let Some(loc) = node.linked_data.location {
            if let Some(addr) = loc.address {
                opp.exterior.location_type = common::model::opportunity::LocationType::At;
                opp.exterior.location_name = loc.name.unwrap_or_default();
                opp.exterior.address_street = addr.street_address.unwrap_or_default();
                opp.exterior.address_city = addr.address_locality.unwrap_or_default();
                opp.exterior.address_state = addr.address_region.unwrap_or_default();
                opp.exterior.address_zip = addr.postal_code.unwrap_or_default();
                opp.exterior.address_country = addr.address_country.unwrap_or_default();
            } else if let Some(addr) = &partner.address {
                opp.exterior.location_type = common::model::opportunity::LocationType::At;
                opp.exterior.location_name = addr.name.clone();
                opp.exterior.address_street = addr.street.clone();
                opp.exterior.address_city = addr.city.clone();
                opp.exterior.address_state = addr.state.clone();
                opp.exterior.address_zip = addr.zip.clone();
                opp.exterior.address_country = addr.country.clone();
            } else {
                opp.exterior.location_type = common::model::opportunity::LocationType::Any;
            }
        } else if let Some(addr) = &partner.address {
            opp.exterior.location_type = common::model::opportunity::LocationType::At;
            opp.exterior.location_name = addr.name.clone();
            opp.exterior.address_street = addr.street.clone();
            opp.exterior.address_city = addr.city.clone();
            opp.exterior.address_state = addr.state.clone();
            opp.exterior.address_zip = addr.zip.clone();
            opp.exterior.address_country = addr.country.clone();
        } else {
            opp.exterior.location_type = common::model::opportunity::LocationType::Any;
        }

        if let Some(custom) = node.custom {
            opp.exterior.opp_venue = custom.indoors_outdoors.unwrap_or_default();

            opp.exterior.opp_descriptor = custom.descriptor.into_iter().collect();

            opp.exterior.min_age = custom.min_age.unwrap_or(0);

            opp.exterior.max_age = custom.max_age.unwrap_or(999);

            opp.exterior.opp_topics = custom.topic;

            opp.exterior.ticket_required = custom
                .ticket_required
                .unwrap_or_else(|| "no".to_string())
                .as_str()
                == "yes";

            opp.exterior.short_desc = custom.short_description.unwrap_or(opp.exterior.short_desc);

            opp.exterior.organization_type = custom.organization_type.unwrap_or_default();

            opp.exterior.languages = custom
                .language
                .unwrap_or_default()
                .into_iter()
                .flat_map(|s| {
                    if let Some(pair) = s.split_once(':') {
                        Some(pair.0.to_string())
                    } else {
                        None
                    }
                })
                .collect();

            opp.exterior.is_online = custom.online.unwrap_or(false);

            let separators: &[_] = &[' ', ',', '#'];

            opp.exterior.opp_hashtags = custom
                .social_media_hashtags
                .unwrap_or_else(String::new)
                .split(separators)
                .flat_map(|s| if s.is_empty() { None } else { Some(s) })
                .map(|s| format!("#{}", s))
                .collect();

            opp.exterior.opp_social_handles = [
                ("twitter".to_string(), custom.twitter.unwrap_or_default()),
                ("facebook".to_string(), custom.facebook.unwrap_or_default()),
                (
                    "instagram".to_string(),
                    custom.instagram.unwrap_or_default(),
                ),
            ]
            .into_iter()
            .collect();
        }

        opp.exterior
            .opp_descriptor
            .append(&mut partner.descriptor.clone());
        opp.exterior.opp_topics.append(&mut partner.topics.clone());
    } else {
        println!("Warning: Opportunity data is not present in the record");
        return None;
    }

    Some(opp)
}

impl<Tz> Structure for EventsQL<Tz>
where
    Tz: TimeZone + std::fmt::Debug,
{
    type Data = Opportunity;

    fn interpret(&self, mut parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Some(entries) = parsed
            .get_mut("data")
            .ok_or_else(|| Error::Structure("Missing 'data' key in GraphQL result".to_string()))?
            .get_mut("events")
            .ok_or_else(|| Error::Structure("Missing 'events' key in GraphQL result".to_string()))?
            .get_mut("edges")
            .ok_or_else(|| Error::Structure("Missing 'edges' key in GraphQL result".to_string()))?
            .as_array_mut()
        {
            Ok(Many(
                entries
                    .drain(..)
                    .flat_map(|x| interpret_one(&self.0, x))
                    .collect(),
            ))
        } else {
            Err(Error::Structure(
                "Edges list was not a list in GraphQL result".to_string(),
            ))
        }
    }
}
