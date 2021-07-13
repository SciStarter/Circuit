use crate::Error;
use serde_json::Value;

pub mod night_sky_network;

pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

pub trait Structure: std::fmt::Debug {
    type Data;

    fn interpret(&self, parsed: Value) -> Result<OneOrMany<Self::Data>, Error>;
}
