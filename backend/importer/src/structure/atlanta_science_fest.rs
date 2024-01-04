use chrono::DateTime;
use common::model::{
    opportunity::{Descriptor, Domain, EntityType, LocationType, OpportunityAll, VenueType},
    partner::LoggedError,
    Partner,
};
use serde_json::Value;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::{Error, OneOrMany, Structure};

const ATLANTA_SCIENCE_FEST: Uuid = Uuid::from_bytes([
    243, 251, 230, 218, 202, 12, 90, 4, 168, 40, 179, 165, 215, 94, 183, 212,
]);

#[derive(Debug)]
pub struct AtlantaScienceFest<const YEAR: u32>;

impl AtlantaScienceFest<2022> {
    fn import_one(event: &Value) -> Result<OpportunityAll, Error> {
        let mut opp = OpportunityAll::default();
        opp.exterior.opp.uid = Uuid::new_v5(
            &ATLANTA_SCIENCE_FEST,
            event["id"]
                .as_str()
                .ok_or_else(|| Error::Data("Event missing id".into()))?
                .as_bytes(),
        );
        opp.exterior.opp.partner_name = "Atlanta Science Festival".to_string();
        opp.exterior.opp.partner_website = Some("https://atlantasciencefestival.org/".to_string());
        opp.exterior.opp.partner_created = Some(
            DateTime::parse_from_rfc3339(event["createdTime"].as_str().ok_or_else(|| {
                Error::Data("Event record is missing created time field".to_string())
            })?)
            .map_err(|_| Error::Misc("Unable to parse created time field".to_string()))?,
        );
        opp.exterior.opp.partner_opp_url = Some(
            event["fields"]["URL"]
                .as_str()
                .ok_or_else(|| Error::Data("Event record is missing URL".to_string()))?
                .to_string(),
        );
        opp.exterior.opp.organization_name = event["fields"]["Presenting Partner(s)"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        opp.exterior.opp.entity_type = EntityType::Opportunity;
        opp.exterior.opp_venue =
            vec![event["fields"]["Venue type"]
                .as_str()
                .map_or(VenueType::Indoors, |v| {
                    if v == "Outdoors" {
                        VenueType::Outdoors
                    } else {
                        VenueType::Indoors
                    }
                })];
        opp.exterior.opp_descriptor = vec![Descriptor::Festival];
        opp.exterior.opp.pes_domain = Domain::LiveScience;
        opp.exterior.opp.title = event["fields"]["Event Name"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        opp.exterior.opp.description = event["fields"]["Teaser"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        opp.exterior.opp.image_url = event["fields"]["Photo"][0]["url"]
            .as_str()
            .unwrap_or_default()
            .to_string();
        opp.exterior.start_datetimes = DateTime::parse_from_rfc3339(
            event["fields"]["Start time"].as_str().unwrap_or_default(),
        )
        .ok()
        .into_iter()
        .collect();
        opp.exterior.end_datetimes =
            DateTime::parse_from_rfc3339(event["fields"]["End time"].as_str().unwrap_or_default())
                .ok()
                .into_iter()
                .collect();
        opp.exterior.has_end = true;
        opp.exterior.opp.is_online = false;
        opp.exterior.opp.location_type = LocationType::At;
        opp.exterior.opp.location_point = serde_json::json!(
            {
                "type": "Point",
                "coordinates": [
                    event["fields"]["Long"][0].as_f64().ok_or_else(|| Error::Data("Error parsing longitude".to_string()))?,
                    event["fields"]["Lat"][0].as_f64().ok_or_else(|| Error::Data("Error parsing latitude".to_string()))?
                ]
            }
        ).try_into()?;
        opp.exterior.opp.partner = ATLANTA_SCIENCE_FEST.clone();
        opp.interior.accepted = Some(true);
        opp.interior.withdrawn = false;

        Ok(opp)
    }
}

#[async_trait::async_trait]
impl Structure for AtlantaScienceFest<2022> {
    type Data = OpportunityAll;

    fn interpret(&self, parsed: serde_json::Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        let mut opps = Vec::new();

        for event in match parsed["Events"]
            .as_array()
            .ok_or_else(|| Error::Data("No \"Events\" key in the JSON data".to_string()))
        {
            Ok(array) => array,
            Err(err) => {
                return OneOrMany::One(Err(LoggedError::from(err).set_raw(parsed.to_string())))
            }
        } {
            match AtlantaScienceFest::<2022>::import_one(event) {
                Ok(opp) => opps.push(Ok(opp)),
                Err(err) => opps.push(Err(LoggedError::from(err).set_raw(event.to_string()))),
            };
        }

        OneOrMany::Many(opps)
    }

    async fn load_partner(&self, db: &Pool<Postgres>) -> Result<Partner, Error> {
        Ok(Partner::load_by_uid(db, &ATLANTA_SCIENCE_FEST).await?)
    }
}
