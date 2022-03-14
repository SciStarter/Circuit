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
pub struct EventsJson<Tz: TimeZone>(pub PartnerInfo<Tz>);

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct Image {
    url: String,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct Data {
    global_id: String,
    date_utc: String,
    modified_utc: String,
    url: String,
    utc_start_date: String,
    utc_end_date: String,
    tags: Vec<String>,
    slug: String,
    image: Option<Image>,
    cost: Option<String>,
    description: String,
    title: String,
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

    opp.exterior.uid = uuid::Uuid::new_v5(&partner.partner, data.global_id.as_bytes());

    opp.exterior.partner = partner.partner.clone();

    opp.exterior.partner_name = partner.partner_name.clone();

    opp.exterior.partner_website = partner.partner_website.clone();

    opp.exterior.partner_logo_url = partner.partner_logo_url.clone();

    opp.exterior.partner_created = match data.date_utc.split_once(' ') {
        Some(pair) => match DateTime::parse_from_rfc3339(&format!("{}T{}Z", pair.0, pair.1)) {
            Ok(dt) => Some(dt),
            Err(_) => None,
        },
        None => None,
    };

    opp.exterior.partner_updated = match data.modified_utc.split_once(' ') {
        Some(pair) => match DateTime::parse_from_rfc3339(&format!("{}T{}Z", pair.0, pair.1)) {
            Ok(dt) => Some(dt),
            Err(_) => None,
        },
        None => None,
    };

    opp.exterior.partner_opp_url = Some(data.url);

    opp.exterior.organization_name = "Terry Lee Wells Nevada Discovery Museum".to_string();

    opp.exterior.organization_website = Some("https://nvdm.org/".to_string());

    opp.interior.contact_email = String::new();

    opp.interior.contact_phone = String::new();

    opp.exterior.entity_type = EntityType::Opportunity;

    opp.exterior.pes_domain = partner.domain.clone();

    opp.exterior.tags = data.tags.into_iter().collect();

    opp.exterior.title = htmlentity::entity::decode(&data.title).iter().collect();

    opp.exterior.description = data.description;

    if let Some(img) = data.image {
        opp.exterior.image_url = img.url;
    }

    opp.exterior.image_credit = String::new();

    opp.exterior.start_datetimes = match data.utc_start_date.split_once(' ') {
        Some(pair) => match format!("{}T{}Z", pair.0, pair.1).parse::<DateTime<Utc>>() {
            Ok(dt) => vec![dt.to_fixed_offset()],
            Err(_) => vec![],
        },
        None => vec![],
    };

    opp.exterior.end_datetimes = match data.utc_end_date.split_once(' ') {
        Some(pair) => match format!("{}T{}Z", pair.0, pair.1).parse::<DateTime<Utc>>() {
            Ok(dt) => vec![dt.to_fixed_offset()],
            Err(_) => vec![],
        },
        None => vec![],
    };

    opp.exterior.has_end = false;

    opp.exterior.attraction_hours = None;

    opp.exterior.cost = if partner.flags.contains(&crate::structure::PartnerFlag::Cost) {
        common::model::opportunity::Cost::Cost
    } else if data.cost.unwrap_or_else(String::new).is_empty() {
        common::model::opportunity::Cost::Free
    } else {
        common::model::opportunity::Cost::Cost
    };

    if let Some(addr) = &partner.address {
        opp.exterior.location_type = common::model::opportunity::LocationType::At;
        opp.exterior.location_name = addr.name.clone();
        opp.exterior.address_street = addr.street.clone();
        opp.exterior.address_city = addr.city.clone();
        opp.exterior.address_state = addr.state.clone();
        opp.exterior.address_zip = addr.zip.clone();
        opp.exterior.address_country = addr.country.clone();
    }

    opp.exterior.opp_venue = vec![common::model::opportunity::VenueType::Indoors];

    opp.exterior.opp_descriptor = partner.descriptor.clone();

    opp.exterior.min_age = 0;

    opp.exterior.max_age = 999;

    opp.exterior.ticket_required = partner
        .flags
        .contains(&crate::structure::PartnerFlag::Ticketed);

    opp.exterior.organization_type = Default::default();

    opp.exterior.languages = vec!["en".to_string()];

    opp.exterior.is_online = false;

    Some(opp)
}

impl<Tz> Structure for EventsJson<Tz>
where
    Tz: TimeZone + std::fmt::Debug,
{
    type Data = Opportunity;

    fn interpret(&self, mut parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Some(entries) = parsed
            .get_mut("events")
            .ok_or_else(|| Error::Structure("Missing 'events' key in GraphQL result".to_string()))?
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
