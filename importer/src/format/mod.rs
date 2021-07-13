use crate::Error;
use bytes::Bytes;
use serde_json::Value;

pub mod csv;
pub mod ical;
pub mod json;

pub trait Format: std::fmt::Debug {
    fn decode(&self, raw: Bytes) -> Result<Value, Error>;
}
