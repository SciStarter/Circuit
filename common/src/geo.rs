use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

static OPENCAGE_API_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("OPENCAGE_API_KEY").unwrap_or_else(|err| String::new()));

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serializing query failed")]
    Surf(String),
}

#[derive(Serialize)]
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
            .map_err(|err| Error::Surf(err.to_string()))?
            .recv_json()
            .await
            .map_err(|err| Error::Surf(err.to_string()))?;

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
#[derive(Deserialize)]
pub struct Status {
    pub message: String,
    pub code: u16,
}

#[readonly::make]
#[derive(Deserialize)]
pub struct Point {
    #[serde(rename = "lat")]
    pub latitude: f32,
    #[serde(rename = "lng")]
    pub longitude: f32,
}

#[readonly::make]
#[derive(Deserialize)]
pub struct Match {
    pub confidence: u16,
    pub formatted: String,
    pub geometry: Point,
}

// Partial, we're not interested in everything that comes back from the API
#[readonly::make]
#[derive(Deserialize)]
pub struct Response {
    pub status: Status,
    pub results: Vec<Match>,
}
