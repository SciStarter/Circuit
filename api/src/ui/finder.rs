use chrono::{DateTime, FixedOffset};
use common::{
    model::opportunity::{Cost, Descriptor, Topic, VenueType},
    Database,
};
use serde::{Deserialize, Serialize};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::ui::{error, okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("filters", |r| r.get(filters))
        .at("topics", |r| r.get(filters))
        .at("activities", |r| r.get(activities))
        .at("random-categories", |r| r.get(random_categories))
        .at("search", |r| r.get(search))
        .at("geo", |r| r.post(geo))
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

#[derive(Serialize, Deserialize, Clone)]
struct GeoPlace {
    near: String,
    lon: f32,
    lat: f32,
    radius: f32,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
enum GeoLookup {
    Coords,
    Near,
}

#[derive(Deserialize)]
struct GeoQuery {
    lookup: GeoLookup,
    place: GeoPlace,
}

#[derive(Serialize)]
struct GeoResult {
    places: Vec<GeoPlace>,
}

#[derive(Serialize)]
struct OpenCageQuery {
    key: String,
    q: String,
    no_annotations: u8,
}

#[derive(Deserialize)]
struct OpenCageStatus {
    message: String,
    code: u16,
}

#[derive(Deserialize)]
struct OpenCagePoint {
    lat: f32,
    #[serde(rename = "lng")]
    lon: f32,
}

#[derive(Deserialize)]
struct OpenCageMatch {
    confidence: u16,
    formatted: String,
    geometry: OpenCagePoint,
}

// Partial, we're not interested in everything that comes back from the API
#[derive(Deserialize)]
struct OpenCageResult {
    status: OpenCageStatus,
    results: Vec<OpenCageMatch>,
}

pub async fn geo(mut req: tide::Request<Database>) -> tide::Result {
    let search: GeoQuery = req.body_json().await?;
    let radius = search.place.radius;

    let query = OpenCageQuery {
        key: std::env::var("OPENCAGE_API_KEY")?,
        q: match search.lookup {
            GeoLookup::Coords => search.place.near,
            GeoLookup::Near => format!("{} {}", search.place.lat, search.place.lon),
        },
        no_annotations: 1,
    };

    let mut result: OpenCageResult = surf::get("https://api.opencagedata.com/geocode/v1/json")
        .query(&query)?
        .recv_json()
        .await?;

    if result.status.code != 200 {
        return error(
            result.status.code,
            "Geographic lookup failed",
            &[&result.status.message],
        );
    }

    result
        .results
        .sort_unstable_by_key(|m| -(m.confidence as i32));

    let places = GeoResult {
        places: result
            .results
            .iter()
            .map(|m| GeoPlace {
                near: m.formatted.to_string(),
                lon: m.geometry.lon,
                lat: m.geometry.lat,
                radius,
            })
            .collect(),
    };

    okay("", &places)
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
