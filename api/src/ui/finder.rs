use chrono::{DateTime, FixedOffset};
use common::{
    model::opportunity::{Cost, Descriptor, Topic, VenueType},
    Database,
};
use serde::Deserialize;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::ui::request_person;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("filters", |r| r.get(filters))
        .at("topics", |r| r.get(filters))
        .at("activities", |r| r.get(activities))
        .at("random-categories", |r| r.get(random_categories))
        .at("search", |r| r.get(search))
}

pub async fn filters(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn topics(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn activities(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

#[derive(Deserialize)]
struct RandomCategoriesQuery {
    num: u16,
}

pub async fn random_categories(req: tide::Request<Database>) -> tide::Result {
    let query: RandomCategoriesQuery = req.query()?;

    todo!()
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SearchQueryPhysical {
    InPersonOrOnline,
    InPerson,
    Online,
}

#[derive(Deserialize)]
#[serde(rename_all = "snake_case")]
enum SearchOrdering {
    Closest,
    Soonest,
}

#[derive(Deserialize)]
struct SearchQuery {
    pub _latitude: Option<f64>,
    pub _longitude: Option<f64>,
    pub _proximity: Option<f64>,
    pub _text: Option<String>,
    pub _beginning: Option<DateTime<FixedOffset>>,
    pub _ending: Option<DateTime<FixedOffset>>,
    pub _physical: Option<SearchQueryPhysical>,
    pub _min_age: Option<i16>,
    pub _max_age: Option<i16>,
    pub _topics: Vec<Topic>,
    pub _descriptors: Vec<Descriptor>,
    pub _cost: Option<Cost>,
    pub _venue_type: Option<VenueType>,
    pub _host: Option<String>,
    pub _partner: Option<Uuid>,
    pub _sort: Option<SearchOrdering>,
    pub _page: Option<u32>,
    pub _per_page: Option<u8>,
    pub _saved: Option<bool>,
    pub _participated: Option<bool>,
    pub _reviewing: Option<bool>,
    pub _withdrawn: Option<bool>,
    pub _over: Option<bool>,
}

pub async fn search(mut req: tide::Request<Database>) -> tide::Result {
    let _person = request_person(&mut req).await?;

    let _query: SearchQuery = req.query()?;

    todo!()
}
