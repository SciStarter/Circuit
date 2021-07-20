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

fn interpret_one(partner: &PartnerInfo, mut entry: Value) -> Option<Opportunity> {
    let mut opp = Opportunity::default();

    opp.exterior.partner = partner.partner.clone();

    opp.exterior.partner_name = partner.partner_name.clone();

    opp.exterior.partner_website = partner.partner_website.clone();

    opp.exterior.partner_logo_url = partner.partner_logo_url.clone();

    if let Some(node) = entry.get("node") {
        opp.exterior.uid = uuid::Uuid::new_v5(
            &partner.partner,
            match node.get("id").and_then(|id| id.as_str()) {
                Some(id) => id,
                None => {
                    println!("Error: Invalid entry (missing id): {:?}", entry);
                    return None;
                }
            }
            .as_bytes(),
        );

        opp.exterior.partner_updated = match node
            .get("modifiedGmt")
            .and_then(|s| s.as_str())
            .map(|s| DateTime::parse_from_rfc3339(&format!("{}Z", s)))
            .transpose()
        {
            Ok(dt) => dt,
            Err(err) => {
                println!(
                    "Warning: Parsing modifiedGmt failed, substituting None: {:?}",
                    err
                );
                None
            }
        };

        opp.exterior.partner_opp_url = match node.get("link").and_then(|s| s.as_str()) {
            Some(url) => url.to_string(),
            None => {
                println!("Error: Entry missing 'link' field.");
                return None;
            }
        };

        if let Some(organizer) = node.get("organizer") {
            opp.exterior.organization_name = match organizer.get("name").and_then(|s| s.as_str()) {
                Some(s) => s.to_string(),
                None => "".to_string(),
            };

            opp.exterior.organization_website = organizer
                .get("url")
                .and_then(|s| s.as_str())
                .map(|s| s.to_string());
        }
    } else {
        println!("Error: Entry missing the 'node' field.");
        return None;
    }

    if let Some(custom) = entry.get_mut("scienceNearMeData") {
        opp.exterior.opp_venue = match custom
            .get_mut("indoorsOutdoors")
            .and_then(|val| val.as_array_mut())
        {
            Some(vec) => vec
                .drain(..)
                .map(|v| serde_json::from_value(v))
                .flatten()
                .collect(),
            None => {
                vec![]
            }
        };

        // !!! next up is opp_descriptor if it gets added to the custom fields
    }

    Some(opp)
}

impl Structure for EventsQLResult {
    type Data = Opportunity;

    fn interpret(&self, mut parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Some(entries) = parsed
            .get_mut("data")
            .ok_or_else(|| Error::Structure("Missing 'data' key in GrtaphQL result".to_string()))?
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
