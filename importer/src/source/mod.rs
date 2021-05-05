use crate::Error;
use bytes::Bytes;

pub mod http;

pub trait Source {
    fn load(&self, source: &dyn AsRef<str>) -> Result<Bytes, Error>;
}
