use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static OPENCAGE_API_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("OPENCAGE_API_KEY").unwrap_or_else(|_| String::new()));

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serializing query {0:?} failed: {1}")]
    Surf(Query, String),
}

#[derive(Serialize, Debug, Clone)]
pub struct Query {
    key: &'static str,
    q: String,
    no_annotations: u8,
}

impl Query {
    pub fn new(query: String, annotations: bool) -> Query {
        Query {
            key: OPENCAGE_API_KEY.as_str(),
            q: query,
            no_annotations: if annotations { 0 } else { 1 },
        }
    }

    pub async fn lookup(&self) -> Result<Response, Error> {
        // Alternative: we may be able to use https://www.geonames.org/export/web-services.html
        let result: Response = surf::get("https://api.opencagedata.com/geocode/v1/json")
            .query(self)
            .map_err(|err| Error::Surf(self.clone(), err.to_string()))?
            .recv_json()
            .await
            .map_err(|err| Error::Surf(self.clone(), err.to_string()))?;

        Ok(result)
    }

    pub async fn lookup_one(&self) -> Option<Match> {
        let matches = match self.lookup().await {
            Ok(r) => r.results,
            Err(_) => return None,
        };

        matches.into_iter().max_by_key(|m| m.confidence)
    }
}

#[readonly::make]
#[derive(Deserialize, Clone, Debug)]
pub struct Status {
    pub message: String,
    pub code: u16,
}

#[readonly::make]
#[derive(Deserialize, Clone, Debug)]
pub struct Point {
    #[serde(rename = "lat")]
    pub latitude: f32,
    #[serde(rename = "lng")]
    pub longitude: f32,
}

#[readonly::make]
#[derive(Deserialize, Clone, Debug)]
pub struct Location {
    pub city: Option<String>,
    pub town: Option<String>,
    pub county: Option<String>,
    pub state: Option<String>,
    pub state_code: String,
    pub postcode: Option<String>,
    pub country: String,
    pub country_code: String,
}

#[readonly::make]
#[derive(Deserialize, Clone, Debug)]
pub struct Match {
    pub confidence: u16,
    pub formatted: String,
    pub geometry: Point,
    pub components: Location,
}

// Partial, we're not interested in everything that comes back from the API
#[readonly::make]
#[derive(Deserialize, Clone, Debug)]
pub struct Response {
    pub status: Status,
    pub results: Vec<Match>,
}
