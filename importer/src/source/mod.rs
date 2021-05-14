use crate::Error;
use bytes::Bytes;

pub mod eventsql;
pub mod http;

pub trait Source {
    fn load<S: AsRef<str>>(&self, source: S) -> Result<Bytes, Error>;
}
