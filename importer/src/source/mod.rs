use crate::Error;
use bytes::Bytes;

pub mod eventsql;
pub mod eventsql_custom;
pub mod http;

pub trait Source: std::fmt::Debug {
    fn load(&self) -> Result<Bytes, Error>;
}
