use super::{
    Error,
    OneOrMany::{self, Many},
    PartnerInfo, Structure,
};
use chrono::{DateTime, FixedOffset};
use common::model::Opportunity;
use serde_json::{from_value, Value};

#[derive(Debug)]
pub struct EventsQLResult(PartnerInfo);

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct EventsQLDataOrganizer {
    name: String,
    url: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct EventsQLDataNode {
    id: String,
    modified_gmt: String,
    link: String,
    organizer: EventsQLDataOrganizer,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
struct EventsQLDataCustom {
    indoors_outdoors: Vec<common::model::opportunity::VenueType>,
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct EventsQLData {
    node: Option<EventsQLDataNode>,
    #[serde(rename = "scienceNearMeData")]
    custom: Option<EventsQLDataCustom>,
}

fn interpret_one(partner: &PartnerInfo, entry: Value) -> Option<Opportunity> {
    let data: EventsQLData = match serde_json::from_value(entry) {
        Ok(d) => d,
        Err(err) => {
            println!("Warning: Entry could not be parsed: {:?}", err);
            return None;
        }
    };

    let mut opp = Opportunity::default();

    if let Some(node) = data.node {
        opp.exterior.partner = partner.partner.clone();

        opp.exterior.partner_name = partner.partner_name.clone();

        opp.exterior.partner_website = partner.partner_website.clone();

        opp.exterior.partner_logo_url = partner.partner_logo_url.clone();

        opp.exterior.uid = uuid::Uuid::new_v5(&partner.partner, node.id.as_bytes());

        opp.exterior.partner_updated =
            match DateTime::parse_from_rfc3339(&format!("{}Z", &node.modified_gmt)) {
                Ok(dt) => Some(dt),
                Err(err) => {
                    println!(
                        "Warning: Parsing modifiedGmt failed, substituting None: {:?}",
                        err
                    );
                    None
                }
            };

        opp.exterior.partner_opp_url = node.link;

        opp.exterior.organization_name = node.organizer.name;

        opp.exterior.organization_website = node.organizer.url;

        // !!! TODO
    } else {
        println!("Warning: Opportunity data is not present in the record");
        return None;
    }

    if let Some(custom) = data.custom {
        opp.exterior.opp_venue = custom.indoors_outdoors;

        // !!! TODO next up is opp_descriptor if it gets added to the custom fields
    }

    Some(opp)
}

impl Structure for EventsQLResult {
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
