use std::collections::HashSet;

use bytes::{Bytes, BytesMut};
use chrono::{Days, Months};
use common::model::partner::{LoggedError, LoggedErrorLevel};
use once_cell::sync::Lazy;
use regex::Regex;

static LD_JSON_BLOCK: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?is)<\s*script\s+type="application/ld\+json"\s*>(.*?)<\s*/\s*script\s*>"#)
        .expect("LD_JSON_BLOCK regex should be valid")
});

#[derive(Debug)]
pub struct ModernEventsCalendar {
    endpoint: String,
}

impl ModernEventsCalendar {
    pub fn new<S: AsRef<str>>(endpoint: S) -> Self {
        ModernEventsCalendar {
            endpoint: endpoint.as_ref().to_string(),
        }
    }
}

#[derive(serde::Deserialize, Default, Debug)]
struct MECResponse {
    offset: usize,
    count: usize,
    #[serde(default, rename = "current_month_divider")]
    _current_month_divider: String,
    #[serde(default, rename = "end_date")]
    _end_date: String,
    #[serde(default, rename = "has_more_event")]
    _has_more_event: u8,
    html: String,
}

impl super::Source for ModernEventsCalendar {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut result = BytesMut::new();
        let mut date = chrono::Local::now().date_naive();
        let mut offset;
        let mut seen = HashSet::new();
        let mut first = true;

        let last_date = date.checked_add_months(Months::new(1)).unwrap();

        result.extend_from_slice(br#"{"items": ["#);

        while date <= last_date {
            dbg!(&date);

            offset = 0;

            loop {
                let raw = ureq::post(&self.endpoint)
                    .send_form(&[
                        ("action", "mec_list_load_more"),
                        ("mec_start_date", date.to_string().as_str()),
                        ("mec_offset", &dbg!(offset).to_string()),
                    ])?
                    .into_string()?; //.into_json()?;

                let resp: MECResponse = serde_json::from_str(&raw)?;

                for block in LD_JSON_BLOCK.captures_iter(&resp.html) {
                    if let Some(cap) = block.get(1) {
                        let block = cap
                            .as_str()
                            .trim()
                            .replace(r"\ ", r"\\ ") // If we need to do many fixups, use aho-corasick
                            .as_bytes()
                            .to_vec();
                        if !seen.contains(&block) {
                            if first {
                                first = false;
                            } else {
                                result.extend_from_slice(b",");
                            }
                            result.extend_from_slice(&block);
                            seen.insert(block);
                        }
                    } else {
                        return Err(LoggedError::new(
                            LoggedErrorLevel::Debug,
                            "Unable to find ld+json content",
                        ));
                    }
                }

                if resp.offset >= (offset + resp.count) {
                    offset = resp.offset;
                } else {
                    break;
                }
            }

            date = date.checked_add_days(Days::new(1)).unwrap();
        }

        result.extend_from_slice(b"]}");

        Ok(result.freeze())
    }
}
