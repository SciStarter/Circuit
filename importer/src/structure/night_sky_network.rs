use super::{
    Error,
    OneOrMany::{self, Many},
    Structure,
};
use chrono::{DateTime, FixedOffset};
use common::model::Opportunity;
use once_cell::sync::Lazy;
use serde_json::{from_value, Value};

pub static UID: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("a844e7ee-6417-5bbc-b97c-f85575836442").unwrap());

#[derive(Debug)]
pub struct NightSkyNetwork;

impl Structure for NightSkyNetwork {
    type Data = Opportunity;

    fn interpret(&self, mut parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Some(events) = parsed.get_mut("events") {
            if let Value::Array(objects) = events.take() {
                let mut opps = Vec::new();

                for mut obj in objects {
                    // The uid field may either be absent or filled
                    // with a UUID string. They included it but left
                    // it empty instead.
                    if let Some(Value::String(uid)) = obj.get("uid") {
                        if uid.is_empty() {
                            let map = obj.as_object_mut().unwrap();
                            map.remove("uid");
                        }
                    }

                    // They sometimes leave the time zone off of the
                    // partner_created field, contrary to requirements.
                    if let Some(Value::String(datetime)) = obj.get("partner_created") {
                        if let Err(_) = DateTime::parse_from_rfc3339(datetime) {
                            let utc = format!("{}Z", datetime);
                            obj["partner_created"] = Value::String(utc);
                        }
                    }

                    // They sometimes leave the time zone off of the
                    // partner_updated field, contrary to requirements.
                    if let Some(Value::String(datetime)) = obj.get("partner_updated") {
                        if let Err(_) = DateTime::parse_from_rfc3339(datetime) {
                            let utc = format!("{}Z", datetime);
                            obj["partner_updated"] = Value::String(utc);
                        }
                    }

                    let mut input: Opportunity = match from_value(obj) {
                        Ok(opp) => opp,
                        Err(err) => {
                            println!("Error importing record: {:?}", err);
                            continue;
                        }
                    };

                    input.exterior.partner = UID.clone();
                    input.validate()?;

                    opps.push(input);
                }

                Ok(Many(opps))
            } else {
                Err(Error::Structure(
                    "Expected Night Sky Network data's events field to contain an array of objects"
                        .to_string(),
                ))
            }
        } else {
            Err(Error::Structure(
                "Expected Night Sky Network data to contain an events field".to_string(),
            ))
        }
    }
}
