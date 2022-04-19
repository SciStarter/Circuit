use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use crate::Database;

static OPENCAGE_API_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("OPENCAGE_API_KEY").unwrap_or_else(|_| String::new()));

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("serializing query {0:?} failed: {1}")]
    Surf(Query, String),
    #[error("serializing query {0:?} failed: {1}")]
    SurfGeom(GeomQuery, String),
    #[error("Zipcode lookup error")]
    ZipDB(#[from] sqlx::Error),
    #[error("result structure incompatible: {0}")]
    Structure(String),
}

#[derive(Serialize, Debug, Clone)]
pub struct Query {
    key: &'static str,
    q: String,
    no_annotations: u8,
    limit: u8,
}

impl Query {
    pub fn new(query: String, annotations: bool) -> Query {
        Query {
            key: OPENCAGE_API_KEY.as_str(),
            q: query,
            no_annotations: if annotations { 0 } else { 1 },
            limit: 1,
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

#[derive(Serialize, Debug, Clone)]
pub struct GeomQuery {
    pub q: String,
    pub format: String,
    pub polygon_geojson: u8,
    pub polygon_threshold: f32,
    pub limit: u8,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GeomResult {
    pub licence: String,
    pub lon: String,
    pub lat: String,
    pub class: String,
    pub geojson: Option<serde_json::Value>,
}

impl GeomQuery {
    pub fn new<Q>(q: Q, threshold: f32) -> Self
    where
        Q: AsRef<str>,
    {
        GeomQuery {
            q: q.as_ref().to_string(),
            format: "json".to_string(),
            polygon_geojson: 1,
            polygon_threshold: threshold,
            limit: 1,
        }
    }

    pub async fn lookup(&self, db: &Database) -> Result<GeomResult, Error> {
        if let Some(result) = sqlx::query_as!(
            GeomResult,
            r#"
SELECT
  'public-domain' AS "licence!",
  ST_Y(ST_Centroid(geom))::text AS "lon!",
  ST_X(ST_Centroid(geom))::text AS "lat!",
  'boundary' AS "class!",
  ST_AsGeoJSON(ST_Simplify(geom, 0.5, true))::jsonb AS "geojson"
FROM "zip_code_tabulation_area"
WHERE "zcta5ce20" like $1;"#,
            self.q,
        )
        .fetch_optional(db)
        .await?
        {
            return Ok(result);
        }

        let results: Vec<GeomResult> = surf::get("https://nominatim.openstreetmap.org/search")
            .header("User-Agent", "ScienceNearMe.org")
            .query(self)
            .map_err(|err| Error::SurfGeom(self.clone(), err.to_string()))?
            .recv_json()
            .await
            .map_err(|err| Error::Structure(err.to_string()))?;

        Ok(results
            .into_iter()
            .next()
            .ok_or_else(|| Error::Structure("results empty".to_string()))?)
    }
}
