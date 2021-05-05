use crate::Error;
use bytes::Bytes;
use std::io::Read;

// We won't read more than 10 MiB from the server, even if it wants to
// send us that much. It's likely an error, or outright malicious.
pub const MAX_SIZE: u64 = 10 * 1024 * 1024;

pub struct HttpGet;

impl super::Source for HttpGet {
    fn load(&self, source: &dyn AsRef<str>) -> Result<Bytes, Error> {
        let mut data = Vec::new();

        ureq::get(source.as_ref())
            .call()?
            .into_reader()
            .take(MAX_SIZE)
            .read_to_end(&mut data)?;

        Ok(data.into())
    }
}
