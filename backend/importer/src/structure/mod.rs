use crate::Error;
use chrono::TimeZone;
use serde_json::Value;
use uuid::Uuid;

pub mod atlanta_science_fest;
pub mod eventsjson;
pub mod eventsql;
pub mod mec;
pub mod neon;
pub mod night_sky_network;

pub use atlanta_science_fest::AtlantaScienceFest;
pub use eventsjson::EventsJson;
pub use eventsql::EventsQL;
pub use mec::ModernEventsCalendar;
pub use night_sky_network::NightSkyNetwork;

pub enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

#[derive(Debug)]
pub struct PartnerAddress {
    pub name: String,
    pub street: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum PartnerFlag {
    Ticketed,
    Cost,
}

#[derive(Debug)]
pub struct PartnerInfo<Tz: TimeZone> {
    pub partner: Uuid,
    pub partner_name: String,
    pub partner_website: Option<String>,
    pub partner_logo_url: Option<String>,
    pub domain: common::model::opportunity::Domain,
    pub descriptor: Vec<common::model::opportunity::Descriptor>,
    pub topics: Vec<common::model::opportunity::Topic>,
    pub flags: Vec<PartnerFlag>,
    pub address: Option<PartnerAddress>,
    pub timezone: Option<Tz>,
}

pub trait Structure: std::fmt::Debug {
    type Data;

    fn interpret(&self, parsed: Value) -> Result<OneOrMany<Self::Data>, Error>;
}
