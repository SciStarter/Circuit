use crate::Error;
use bytes::Bytes;

pub mod airtable;
pub mod eventsql;
pub mod eventsql_custom;
pub mod http;
pub mod neoncrm;

pub use airtable::Airtable;
pub use eventsql::EventsQL;
pub use eventsql_custom::EventsQLWithCustom;
pub use http::HttpGet;
pub use neoncrm::NeonCRM;

pub trait Source: std::fmt::Debug {
    fn load(&self) -> Result<Bytes, Error>;
}

#[derive(Debug)]
pub struct DebugSource<S: std::fmt::Debug>(pub S);

impl<S: Source> Source for DebugSource<S> {
    fn load(&self) -> Result<Bytes, Error> {
        dbg!(self.0.load())
    }
}
