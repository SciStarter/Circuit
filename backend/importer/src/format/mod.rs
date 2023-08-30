use bytes::Bytes;
use common::model::partner::LoggedError;
use serde_json::Value;

pub mod csv;
pub mod ical;
pub mod json;

pub use self::csv::{CommaSeparated, SemicolonSeparated, TabSeparated};
pub use self::ical::Ical;
pub use json::Json;

pub trait Format: std::fmt::Debug {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError>;
}

#[derive(Debug)]
pub struct DebugFormat<F: std::fmt::Debug>(pub F);

impl<F: Format> Format for DebugFormat<F> {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        let ret = self.0.decode(raw)?;
        println!("{}", serde_json::to_string_pretty(&ret).unwrap());
        Ok(ret)
    }
}
