use crate::Error;
use bytes::{BufMut, Bytes, BytesMut};
use common::model::partner::LoggedError;

// We won't read more than 64 MiB from the server, even if it wants to
// send us that much. It's likely an error, or outright malicious.
pub const MAX_SIZE: usize = 64 * 1024 * 1024;

#[derive(Debug)]
pub struct HttpGet {
    endpoint: String,
}

impl HttpGet {
    pub fn new<S: AsRef<str>>(endpoint: S) -> Self {
        HttpGet {
            endpoint: endpoint.as_ref().to_string(),
        }
    }
}

impl super::Source for HttpGet {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut writer = BytesMut::new().limit(MAX_SIZE).writer();
        let mut reader = ureq::get(&self.endpoint).call()?.into_reader();

        std::io::copy(&mut reader, &mut writer)?;

        Ok(writer.into_inner().into_inner().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Source;

    #[test]
    fn fetch_night_sky_network() {
        HttpGet::new("https://nightsky.jpl.nasa.gov/js/data/events_json_api.cfm")
            .load()
            .unwrap();
    }
}
