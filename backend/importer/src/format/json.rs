use bytes::Bytes;
use common::model::partner::LoggedError;
use serde_json::Value;

#[derive(Debug)]
pub struct Json;

impl super::Format for Json {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        //let _ = std::fs::write("json.raw", &raw);
        let cleaned = String::from_utf8_lossy(&raw).replace(|c: char| c.is_control(), " ");
        //let _ = std::fs::write("json.cleaned", &cleaned);
        Ok(serde_json::from_str(cleaned.as_str())?)
    }
}
