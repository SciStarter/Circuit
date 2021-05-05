use crate::Error;
use serde_json::Value;

pub trait Structure {
    type Data;

    fn interpret(&self, parsed: Value) -> Result<Self::Data, Error>;
}
