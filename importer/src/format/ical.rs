use crate::Error;
use bytes::Bytes;
use ical::IcalParser;
use serde_json::json;
use serde_json::Value;
use std::io::BufReader;

#[derive(Debug)]
pub struct Ical;

fn _property(p: &ical::property::Property) -> (String, Value) {
    let mut vals = serde_json::Map::new();

    if let Some(ps) = &p.params {
        vals.insert(
            "params".to_string(),
            Value::Object(
                ps.iter()
                    .map(|p| {
                        (
                            p.0.clone(),
                            Value::Array(p.1.iter().map(|s| Value::String(s.clone())).collect()),
                        )
                    })
                    .collect(),
            ),
        );
    } else {
        vals.insert("params".to_string(), Value::Null);
    }

    if let Some(v) = &p.value {
        vals.insert("value".to_string(), Value::String(v.clone()));
    } else {
        vals.insert("value".to_string(), Value::Null);
    }

    (p.name.clone(), Value::Object(vals))
}

impl super::Format for Ical {
    fn decode(&self, raw: Bytes) -> Result<Value, Error> {
        let mut entries = Vec::new();

        for calendar in IcalParser::new(BufReader::new(raw.as_ref())) {
            let calendar = calendar?;
            let mut entry = serde_json::Map::new();

            entry.insert(
                "properties".to_string(),
                Value::Object(calendar.properties.iter().map(_property).collect()),
            );

            entry.insert(
                "events".to_string(),
                Value::Array(
                    calendar
                        .events
                        .iter()
                        .map(|event| {
                            json!({
                                "properties": Value::Object(event.properties.iter().map(_property).collect()),
                                "alarms": Value::Array(event.alarms.iter().map(|p| Value::Object(p.properties.iter().map(_property).collect())).collect())
                            })
                        })
                        .collect(),
                ),
            );

            entry.insert(
                "alarms".to_string(),
                Value::Array(
                    calendar
                        .alarms
                        .iter()
                        .map(|alarm| {
                            Value::Object(alarm.properties.iter().map(_property).collect())
                        })
                        .collect(),
                ),
            );

            entry.insert(
                "todos".to_string(),
                Value::Array(
                    calendar
                        .todos
                        .iter()
                        .map(|todo| {
                            json!({
                                "properties": Value::Object(todo.properties.iter().map(_property).collect()),
                                "alarms": Value::Array(todo.alarms.iter().map(|p| Value::Object(p.properties.iter().map(_property).collect())).collect())
                            })
                        })
                        .collect(),
                ),
            );

            entry.insert(
                "journals".to_string(),
                Value::Array(
                    calendar
                        .journals
                        .iter()
                        .map(|journal| {
                            Value::Object(journal.properties.iter().map(_property).collect())
                        })
                        .collect(),
                ),
            );

            entry.insert(
                "free_busys".to_string(),
                Value::Array(
                    calendar
                        .free_busys
                        .iter()
                        .map(|free_busy| {
                            Value::Object(free_busy.properties.iter().map(_property).collect())
                        })
                        .collect(),
                ),
            );

            entry.insert(
                "timezones".to_string(),
                Value::Array(
                    calendar
                        .timezones
                        .iter()
                        .map(|timezone| {
                            json!({
                                "properties": Value::Object(timezone.properties.iter().map(_property).collect()),
                                "transitions": Value::Array(timezone.transitions.iter().map(|p| Value::Object(p.properties.iter().map(_property).collect())).collect())
                            })
                        })
                        .collect(),
                ),
            );

            entries.push(Value::Object(entry));
        }

        Ok(Value::Array(entries))
    }
}
