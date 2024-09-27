use bytes::Bytes;
use common::model::partner::LoggedError;

pub struct Embedded {
    content: &'static [u8],
}

impl std::fmt::Debug for Embedded {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Embedded")
    }
}

impl Embedded {
    pub fn new(content: &'static [u8]) -> Self {
        Embedded { content }
    }
}

impl super::Source for Embedded {
    fn load(&self) -> Result<Bytes, LoggedError> {
        Ok(Bytes::from_static(self.content))
    }
}
