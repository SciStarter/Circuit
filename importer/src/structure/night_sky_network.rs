use super::{
    Error,
    OneOrMany::{self, Many},
    Structure,
};
use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::Tz;
use common::model::Opportunity;
use once_cell::sync::Lazy;
use serde_json::{from_value, Value};

pub static UID: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("a844e7ee-6417-5bbc-b97c-f85575836442").unwrap());

#[derive(Debug)]
pub struct NightSkyNetwork;

fn get_time_zone(lon: f64, lat: f64) -> Result<Tz, Error> {
    let response: serde_json::Value = ureq::get("http://api.geonames.org/timezoneJSON")
        .query("lng", &lon.to_string())
        .query("lat", &lat.to_string())
        .query("username", "scistarter")
        .call()?
        .into_json()?;

    let name = response["timezoneId"].as_str();

    if let Some(name) = name {
        Ok(name.parse().map_err(|_| Error::Comprehension)?)
    } else {
        return Err(Error::Comprehension);
    }
}

fn normalize_datetime(datetime: &str, zone: Option<Tz>) -> Option<String> {
    if let Ok(dt) = DateTime::parse_from_rfc3339(datetime) {
        Some(dt.to_rfc3339())
    } else if let Ok(dt) = DateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M%z") {
        Some(dt.to_rfc3339())
    } else if let Some(tz) = zone {
        if let Ok(naive) = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M:%S") {
            if let Some(dt) = tz.from_local_datetime(&naive).earliest() {
                Some(dt.to_rfc3339())
            } else {
                Some(format!("{}Z", datetime))
            }
        } else if let Ok(naive) = NaiveDateTime::parse_from_str(datetime, "%Y-%m-%dT%H:%M") {
            if let Some(dt) = tz.from_local_datetime(&naive).earliest() {
                Some(dt.to_rfc3339())
            } else {
                Some(format!("{}:00Z", datetime))
            }
        } else {
            None
        }
    } else {
        Some(format!("{}Z", datetime))
    }
}

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

                    // NSN generates incorrect GeoJSON, with the
                    // coordinates in lat,lon instead of the specified
                    // lon,lat (aka x,y) order
                    let lat = obj["location_point"]["coordinates"][0].as_f64();
                    let lon = obj["location_point"]["coordinates"][1].as_f64();

                    let tz = if let (Some(lon), Some(lat)) = (lon, lat) {
                        obj["location_point"]["coordinates"][0] = lon.into();
                        obj["location_point"]["coordinates"][1] = lat.into();

                        // Additionally, they provide times without
                        // time zones, so we try to get the time zone
                        // based on the coordinates
                        match get_time_zone(lon, lat) {
                            Ok(tz) => Some(tz),
                            Err(_) => None,
                        }
                    } else {
                        None
                    };

                    // They sometimes leave the time zone off of the
                    // partner_created field, contrary to requirements.
                    if let Some(Value::String(datetime)) = obj.get("partner_created") {
                        obj["partner_created"] = match normalize_datetime(&datetime, tz) {
                            Some(s) => Value::String(s),
                            None => Value::Null,
                        }
                    };

                    // They sometimes leave the time zone off of the
                    // partner_updated field, contrary to requirements.
                    if let Some(Value::String(datetime)) = obj.get("partner_updated") {
                        obj["partner_updated"] = match normalize_datetime(&datetime, tz) {
                            Some(s) => Value::String(s),
                            None => Value::Null,
                        }
                    }

                    // They sometimes leave the time zone off of the
                    // start_dates field values, contrary to
                    // requirements.
                    obj["start_dates"] = if let Some(datetimes) = obj["start_dates"].as_array() {
                        Value::Array(
                            datetimes
                                .clone()
                                .into_iter()
                                .map(|v| {
                                    if let Some(s) = v.as_str() {
                                        match normalize_datetime(s, tz) {
                                            Some(n) => Value::String(n),
                                            None => Value::Null,
                                        }
                                    } else {
                                        Value::Null
                                    }
                                })
                                .filter(|v| v.is_string())
                                .collect(),
                        )
                    } else {
                        Value::Null
                    };

                    // They sometimes leave the time zone off of the
                    // end_dates field values, contrary to
                    // requirements.
                    obj["end_dates"] = if let Some(datetimes) = obj["end_dates"].as_array() {
                        Value::Array(
                            datetimes
                                .clone()
                                .into_iter()
                                .map(|v| {
                                    if let Some(s) = v.as_str() {
                                        match normalize_datetime(s, tz) {
                                            Some(n) => Value::String(n),
                                            None => Value::Null,
                                        }
                                    } else {
                                        Value::Null
                                    }
                                })
                                .filter(|v| v.is_string())
                                .collect(),
                        )
                    } else {
                        Value::Null
                    };

                    if obj["start_dates"].is_array() {
                        obj["start_datetimes"] = obj["start_dates"].take();
                    }

                    if obj["end_dates"].is_array() {
                        obj["end_datetimes"] = obj["end_dates"].take();
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
