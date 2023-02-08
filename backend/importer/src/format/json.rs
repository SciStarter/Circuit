use crate::Error;
use bytes::Bytes;
use common::model::partner::LoggedError;
use serde_json::Value;

#[derive(Debug)]
pub struct Json;

impl super::Format for Json {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        Ok(serde_json::from_slice(&raw)?)

        // // This is a workaround for tab characters in JSON strings.
        // // Hopefully temporary, and to be removed when the Night Sky
        // // Network starts providing valid JSON.
        // let filtered: String = std::str::from_utf8(&raw)?
        //     .chars()
        //     .map(|c| if c == '\t' { ' ' } else { c })
        //     .collect();
        // Ok(serde_json::from_str(&filtered)?)
    }
}
