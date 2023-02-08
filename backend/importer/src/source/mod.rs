use crate::Error;
use bytes::Bytes;

pub mod airtable;
pub mod eventsql;
pub mod eventsql_custom;
pub mod http;
pub mod neoncrm;
pub mod wp_rest;

pub use airtable::Airtable;
use common::model::partner::LoggedError;
pub use eventsql::EventsQL;
pub use eventsql_custom::EventsQLWithCustom;
pub use http::HttpGet;
pub use neoncrm::NeonCRM;
pub use wp_rest::WordPressRest;

pub trait Source: std::fmt::Debug {
    fn load(&self) -> Result<Bytes, LoggedError>;
}

#[derive(Debug)]
pub struct DebugSource<S: std::fmt::Debug>(pub S);

impl<S: Source> Source for DebugSource<S> {
    fn load(&self) -> Result<Bytes, LoggedError> {
        dbg!(self.0.load())
    }
}
