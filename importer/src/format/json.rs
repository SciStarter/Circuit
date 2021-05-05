use crate::Error;
use bytes::Bytes;
use serde_json::Value;

pub struct Json;

impl super::Format for Json {
    fn decode(&self, raw: Bytes) -> Result<Value, Error> {
        Ok(serde_json::from_slice(&raw)?)
    }
}
