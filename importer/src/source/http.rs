use crate::Error;
use bytes::{BufMut, Bytes, BytesMut};

// We won't read more than 10 MiB from the server, even if it wants to
// send us that much. It's likely an error, or outright malicious.
pub const MAX_SIZE: usize = 10 * 1024 * 1024;

pub struct HttpGet;

impl super::Source for HttpGet {
    fn load<S: AsRef<str>>(&self, source: S) -> Result<Bytes, Error> {
        let mut writer = BytesMut::new().limit(MAX_SIZE).writer();
        let mut reader = ureq::get(source.as_ref()).call()?.into_reader();

        std::io::copy(&mut reader, &mut writer)?;

        Ok(writer.into_inner().into_inner().into())
    }
}
