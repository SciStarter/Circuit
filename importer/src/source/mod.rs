use crate::Error;
use bytes::Bytes;

pub mod eventsql;
pub mod http;

pub trait Source {
    fn load(&self) -> Result<Bytes, Error>;
}
