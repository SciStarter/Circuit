use std::marker::PhantomData;

use super::{
    Error,
    OneOrMany::{self, Many},
    PartnerInfo, Structure,
};
use chrono::{DateTime, TimeZone, Utc};
use common::model::{opportunity::EntityType, partner::LoggedError, Opportunity, Partner};
use common::ToFixedOffset;
use htmlentity::entity::ICodedDataTrait;
use serde::Deserialize;
use serde_json::Value;
use sqlx::{Pool, Postgres};

#[derive(Debug)]
pub struct EventsJson<Tz: TimeZone>(pub PartnerInfo<Tz>);

#[derive(Debug)]
pub enum StringOr<Value> {
    String(String),
    Value(Value),
}

impl<T> ToString for StringOr<T>
where
    T: ToString,
{
    fn to_string(&self) -> String {
        match self {
            StringOr::String(s) => s.clone(),
            StringOr::Value(v) => v.to_string(),
        }
    }
}

impl<'de, T> serde::de::Deserialize<'de> for StringOr<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VisitStringOr<V>(PhantomData<fn() -> V>);

        impl<'de, V: serde::de::Deserialize<'de>> serde::de::Visitor<'de> for VisitStringOr<V> {
            type Value = StringOr<V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "string or {}", std::any::type_name::<V>())
            }

            fn visit_str<E>(self, value: &str) -> Result<StringOr<V>, E>
            where
                E: serde::de::Error,
            {
                Ok(StringOr::String(value.to_string()))
            }

            fn visit_map<M>(self, map: M) -> Result<StringOr<V>, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                Ok(StringOr::Value(Deserialize::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?))
            }
        }

        deserializer.deserialize_any(VisitStringOr(PhantomData))
    }
}

#[derive(Debug, Default)]
pub enum Optional<Value> {
    #[default]
    Empty,
    Value(Value),
}

impl<'de, T> serde::de::Deserialize<'de> for Optional<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VisitOptional<V>(PhantomData<fn() -> V>);

        impl<'de, V: serde::de::Deserialize<'de>> serde::de::Visitor<'de> for VisitOptional<V> {
            type Value = Optional<V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(
                    formatter,
                    "false, null, empty string, or {}",
                    std::any::type_name::<V>()
                )
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(Optional::Empty)
            }

            fn visit_bool<E>(self, v: bool) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                if !v {
                    Ok(Optional::Empty)
                } else {
                    Err(E::custom("non-false bool"))
                }
            }

            fn visit_str<E>(self, value: &str) -> Result<Optional<V>, E>
            where
                E: serde::de::Error,
            {
                if value.is_empty() {
                    Ok(Optional::Empty)
                } else {
                    Err(E::custom("non-empty string"))
                }
            }

            fn visit_map<M>(self, map: M) -> Result<Optional<V>, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                Ok(Optional::Value(Deserialize::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?))
            }
        }

        deserializer.deserialize_any(VisitOptional(PhantomData))
    }
}

#[derive(Debug, Default)]
pub enum ListOption<Value> {
    #[default]
    Empty,
    Value(Value),
}

impl<'de, T> serde::de::Deserialize<'de> for ListOption<T>
where
    T: serde::de::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct VisitListOption<V>(PhantomData<fn() -> V>);

        impl<'de, V: serde::de::Deserialize<'de>> serde::de::Visitor<'de> for VisitListOption<V> {
            type Value = ListOption<V>;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "null, list, or {}", std::any::type_name::<V>())
            }

            fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(ListOption::Empty)
            }

            fn visit_map<M>(self, map: M) -> Result<ListOption<V>, M::Error>
            where
                M: serde::de::MapAccess<'de>,
            {
                Ok(ListOption::Value(Deserialize::deserialize(
                    serde::de::value::MapAccessDeserializer::new(map),
                )?))
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<ListOption<V>, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                if let Some(el) = seq.next_element::<V>()? {
                    Ok(ListOption::Value(el))
                } else {
                    Ok(ListOption::Empty)
                }
            }
        }

        deserializer.deserialize_any(VisitListOption(PhantomData))
    }
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct Image {
    url: String,
}

#[derive(serde::Deserialize)]
struct StateAlternates {
    stateprovince: Option<String>,
    state: Option<String>,
}

fn deserialize_state_alternates<'d, D: serde::Deserializer<'d>>(d: D) -> Result<String, D::Error> {
    let StateAlternates {
        stateprovince,
        state,
    } = StateAlternates::deserialize(d)?;
    stateprovince
        .or(state)
        .or_else(|| Some(String::new()))
        .ok_or_else(|| serde::de::Error::custom("`stateprovince` or `state` field is required"))
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct Venue {
    #[serde(alias = "venue")]
    name: String,
    address: String,
    city: String,
    #[serde(deserialize_with = "deserialize_state_alternates", flatten)]
    state: String,
    country: String,
    zip: String,
}

#[derive(serde::Deserialize, Debug)]
struct Tag {
    name: String,
}

impl ToString for Tag {
    fn to_string(&self) -> String {
        self.name.clone()
    }
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct Data {
    global_id: String,
    date_utc: String,
    modified_utc: String,
    url: String,
    utc_start_date: String,
    utc_end_date: String,
    tags: Vec<StringOr<Tag>>,
    slug: String,
    image: Optional<Image>,
    cost: Option<String>,
    description: String,
    title: String,
    venue: ListOption<Venue>,
}

fn interpret_one<Tz: TimeZone>(
    partner: &PartnerInfo<Tz>,
    entry: Value,
) -> Result<Opportunity, LoggedError> {
    //let dump = serde_json::to_string_pretty(&entry)?;

    let data: Data = serde_json::from_value(entry)?;

    let mut opp = Opportunity::default();

    opp.exterior.uid = uuid::Uuid::new_v5(&partner.partner, data.global_id.as_bytes());

    opp.exterior.partner = partner.partner.clone();

    opp.exterior.partner_name = partner.partner_name.clone();

    opp.exterior.partner_website = partner.partner_website.clone();

    opp.exterior.partner_logo_url = partner.partner_logo_url.clone();

    opp.exterior.partner_created = match data.date_utc.split_once(' ') {
        Some(pair) => match DateTime::parse_from_rfc3339(&format!("{}T{}Z", pair.0, pair.1)) {
            Ok(dt) => Some(dt),
            Err(_) => None,
        },
        None => None,
    };

    opp.exterior.partner_updated = match data.modified_utc.split_once(' ') {
        Some(pair) => match DateTime::parse_from_rfc3339(&format!("{}T{}Z", pair.0, pair.1)) {
            Ok(dt) => Some(dt),
            Err(_) => None,
        },
        None => None,
    };

    opp.exterior.partner_opp_url = Some(data.url);

    opp.exterior.organization_name = partner.partner_name.clone();

    opp.exterior.organization_website = partner.partner_website.clone();

    opp.interior.contact_email = String::new();

    opp.interior.contact_phone = String::new();

    opp.exterior.entity_type = EntityType::Opportunity;

    opp.exterior.pes_domain = partner.domain.clone();

    opp.exterior.tags = data.tags.into_iter().map(|x| x.to_string()).collect();

    opp.exterior.title = htmlentity::entity::decode(data.title.as_bytes())
        .to_string()
        .unwrap_or_default();

    opp.exterior.description = common::html_to_md(&data.description);

    if let Optional::Value(img) = data.image {
        opp.exterior.image_url = img.url;
    }

    opp.exterior.image_credit = String::new();

    opp.exterior.start_datetimes = match data.utc_start_date.split_once(' ') {
        Some(pair) => match format!("{}T{}Z", pair.0, pair.1).parse::<DateTime<Utc>>() {
            Ok(dt) => vec![dt.to_fixed_offset()],
            Err(_) => vec![],
        },
        None => vec![],
    };

    opp.exterior.end_datetimes = match data.utc_end_date.split_once(' ') {
        Some(pair) => match format!("{}T{}Z", pair.0, pair.1).parse::<DateTime<Utc>>() {
            Ok(dt) => vec![dt.to_fixed_offset()],
            Err(_) => vec![],
        },
        None => vec![],
    };

    opp.exterior.has_end = !opp.exterior.end_datetimes.is_empty();

    opp.exterior.attraction_hours = None;

    opp.exterior.cost = if partner.flags.contains(&crate::structure::PartnerFlag::Cost) {
        common::model::opportunity::Cost::Cost
    } else {
        match data.cost.unwrap_or_else(String::new).as_str() {
            "" | "free" | "Free" => common::model::opportunity::Cost::Free,
            _ => common::model::opportunity::Cost::Cost,
        }
    };

    if let ListOption::Value(venue) = data.venue {
        opp.exterior.location_type = common::model::opportunity::LocationType::At;
        opp.exterior.location_name = venue.name.clone();
        opp.exterior.address_street = venue.address.clone();
        opp.exterior.address_city = venue.city.clone();
        opp.exterior.address_state = venue.state.clone();
        opp.exterior.address_zip = venue.zip.clone();
        opp.exterior.address_country = venue.country.clone();
    } else if let Some(addr) = &partner.address {
        opp.exterior.location_type = common::model::opportunity::LocationType::At;
        opp.exterior.location_name = addr.name.clone();
        opp.exterior.address_street = addr.street.clone();
        opp.exterior.address_city = addr.city.clone();
        opp.exterior.address_state = addr.state.clone();
        opp.exterior.address_zip = addr.zip.clone();
        opp.exterior.address_country = addr.country.clone();
    }

    opp.exterior.opp_venue = vec![common::model::opportunity::VenueType::Indoors];

    opp.exterior.opp_descriptor = partner.descriptor.clone();

    opp.exterior.min_age = 0;

    opp.exterior.max_age = 999;

    opp.exterior.ticket_required = partner
        .flags
        .contains(&crate::structure::PartnerFlag::Ticketed);

    opp.exterior.organization_type = Default::default();

    opp.exterior.languages = vec!["en".to_string()];

    opp.exterior.is_online = false;

    Ok(opp)
}

#[async_trait::async_trait]
impl<Tz> Structure for EventsJson<Tz>
where
    Tz: TimeZone + std::fmt::Debug + Sync + Send,
{
    type Data = Opportunity;

    fn interpret(&self, mut parsed: Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        if let Some(entries) = parsed["events"].as_array_mut() {
            Many(
                entries
                    .drain(..)
                    .map(|x| interpret_one(&self.0, x))
                    .collect(),
            )
        } else {
            OneOrMany::One(Err(Error::Data(
                "Edges list was missing or not a list in GraphQL result".to_string(),
            )
            .into()))
        }
    }

    async fn load_partner(&self, db: &Pool<Postgres>) -> Result<Partner, Error> {
        self.0.load_partner(db).await
    }
}
