use crate::Error;
use serde_json::Value;
use uuid::Uuid;

pub mod eventsql;
pub mod neon;
pub mod night_sky_network;

pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Debug)]
pub struct PartnerInfo {
    pub partner: Uuid,
    pub partner_name: String,
    pub partner_website: Option<String>,
    pub partner_logo_url: Option<String>,
}

pub trait Structure: std::fmt::Debug {
    type Data;

    fn interpret(&self, parsed: Value) -> Result<OneOrMany<Self::Data>, Error>;
}
