//! Uses WP-JSON to query event information from The Events Calendar
//! Pro

use bytes::Bytes;
use common::model::partner::LoggedError;
use serde_json::{json, Value};

#[derive(Debug)]
pub struct WordPressRest {
    endpoint: String,
}

#[derive(serde::Deserialize, Default, Debug)]
struct WPJsonResponse {
    events: Vec<Value>,
    #[serde(default, rename = "rest_url")]
    _rest_url: String,
    next_rest_url: Option<String>,
    #[serde(default, rename = "previous_rest_url")]
    _previous_rest_url: Option<String>,
    #[serde(default, rename = "total")]
    _total: u32,
    #[serde(default, rename = "total_pages")]
    _total_pages: u32,
}

impl WordPressRest {
    pub fn new<S: AsRef<str>>(endpoint: S) -> Self {
        WordPressRest {
            endpoint: endpoint.as_ref().to_string(),
        }
    }
}

impl super::Source for WordPressRest {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut records = Vec::new();

        let mut resp = WPJsonResponse {
            next_rest_url: Some(self.endpoint.clone()),
            ..Default::default()
        };

        while let Some(url) = resp.next_rest_url.take() {
            resp = ureq::get(dbg!(&url)).call()?.into_json()?;
            records.append(&mut resp.events);
        }

        Ok(serde_json::to_string(&json!({ "events": records }))?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Source;

    #[test]
    fn fetch_michigan_science_center() {
        WordPressRest::new(
            "https://www.mi-sci.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50",
        )
        .load()
        .unwrap();
    }

    #[test]
    fn fetch_museum_of_discovery_and_science() {
        WordPressRest::new(
            "https://mods.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50",
        )
        .load()
        .unwrap();
    }
}
