use std::collections::BTreeMap;

use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{cache_json, Database};

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
            q: query.trim().to_string(),
            no_annotations: if annotations { 0 } else { 1 },
            limit: 1,
        }
    }

    pub fn with_limit(mut self, limit: u8) -> Query {
        self.limit = limit;
        self
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
            q: q.as_ref().trim().to_string(),
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

#[derive(Serialize)]
struct OverviewState {
    point: i64,
    polygon: i64,
}

#[derive(Serialize)]
struct OverviewDomain {
    name: String,
    value: i64,
}

/// Unlike the actual search function, this function and related
/// geo-explorer functions collapse opportunities of the same name
/// from the same partner. That produces a result more suitable for
/// maps.
pub async fn opp_regional_detailed_counts(
    db: Database,
    name: Option<String>,
) -> Result<serde_json::Value, sqlx::Error> {
    let anywhere_total = sqlx::query_scalar!(
        r#"
SELECT COUNT(x.*) AS "result!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
  FROM c_opportunity o
  WHERE
    o.exterior->>'location_type' = 'any' AND
    c_opportunity_is_current(o.interior, o.exterior)
  ORDER BY exterior->>'title', exterior->>'partner'
) x
"#
    )
    .fetch_one(&db)
    .await?;

    let anywhere_domains = sqlx::query!(
        r#"
SELECT x.exterior->>'pes_domain' AS "domain!", COUNT(x.*) AS "total!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') *
  FROM c_opportunity o
  WHERE
    c_opportunity_is_current(o.interior, o.exterior) AND
    o.exterior->>'pes_domain' != 'unspecified' AND
    o.exterior->>'location_type' = 'any'
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
  ) x
GROUP BY exterior->>'pes_domain'
ORDER BY "total!" DESC
"#
    )
    .map(|row| OverviewDomain {
        name: row.domain.to_owned(),
        value: row.total,
    })
    .fetch_all(&db)
    .await?;

    let (points_total, points_domains, polygons_total, polygons_domains) = if let Some(name) = name
    {
        let points_total = sqlx::query_scalar!(
            r#"
SELECT COUNT(x.*) AS "result!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
  FROM c_opportunity o JOIN c_region r ON r."name" = $1
  WHERE
    exterior->>'location_type' = 'at' AND
    c_opportunity_is_current(o.interior, o.exterior) AND
    ST_Intersects(o.location_point, r.geometry)
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
"#,
            &name
        )
        .fetch_one(&db)
        .await?;
        let points_domains = sqlx::query!(
            r#"
SELECT x.exterior->>'pes_domain' AS "domain!", COUNT(x.*) AS "total!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') *
  FROM c_opportunity o JOIN c_region r ON r."name" = $1
  WHERE
    c_opportunity_is_current(o.interior, o.exterior) AND
    o.exterior->>'pes_domain' != 'unspecified' AND
    o.exterior->>'location_type' = 'at' AND
    ST_Intersects(o.location_point, r.geometry)
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
GROUP BY x.exterior->>'pes_domain'
ORDER BY "total!" DESC
"#,
            &name
        )
        .map(|row| OverviewDomain {
            name: row.domain.to_owned(),
            value: row.total,
        })
        .fetch_all(&db)
        .await?;

        let polygons_total = sqlx::query_scalar!(
            r#"
SELECT COUNT(x.*) AS "result!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
  FROM c_opportunity o JOIN c_region r ON r."name" = $1
  WHERE
    o.exterior->>'location_type' = 'near' AND
    c_opportunity_is_current(o.interior, o.exterior) AND
    ST_Intersects(o.location_polygon, r.geometry)
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
"#,
            &name
        )
        .fetch_one(&db)
        .await?;

        let polygons_domains = sqlx::query!(
            r#"
SELECT x.exterior->>'pes_domain' AS "domain!", COUNT(x.*) AS "total!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') *
  FROM c_opportunity o JOIN c_region r ON r."name" = $1
  WHERE
    c_opportunity_is_current(o.interior, o.exterior) AND
    o.exterior->>'pes_domain' != 'unspecified' AND
    o.exterior->>'location_type' = 'near' AND
    ST_Intersects(o.location_polygon, r.geometry)
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
GROUP BY x.exterior->>'pes_domain'
ORDER BY "total!" DESC
"#,
            &name
        )
        .map(|row| OverviewDomain {
            name: row.domain.to_owned(),
            value: row.total,
        })
        .fetch_all(&db)
        .await?;

        (
            points_total,
            points_domains,
            polygons_total,
            polygons_domains,
        )
    } else {
        let points_total = sqlx::query_scalar!(
            r#"
SELECT COUNT(x.*) AS "result!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
  FROM c_opportunity o
  WHERE
    exterior->>'location_type' = 'at' AND
    c_opportunity_is_current(o.interior, o.exterior)
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
"#
        )
        .fetch_one(&db)
        .await?;

        let points_domains = sqlx::query!(
            r#"
SELECT x.exterior->>'pes_domain' AS "domain!", COUNT(x.*) AS "total!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') *
  FROM c_opportunity o
  WHERE
    c_opportunity_is_current(o.interior, o.exterior) AND
    o.exterior->>'pes_domain' != 'unspecified' AND
    o.exterior->>'location_type' = 'at'
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
GROUP BY x.exterior->>'pes_domain'
ORDER BY "total!" DESC
"#
        )
        .map(|row| OverviewDomain {
            name: row.domain.to_owned(),
            value: row.total,
        })
        .fetch_all(&db)
        .await?;

        let polygons_total = sqlx::query_scalar!(
            r#"
SELECT COUNT(x.*) AS "result!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
  FROM c_opportunity o
  WHERE
    o.exterior->>'location_type' = 'near' AND
    c_opportunity_is_current(o.interior, o.exterior) 
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
"#
        )
        .fetch_one(&db)
        .await?;

        let polygons_domains = sqlx::query!(
            r#"
SELECT x.exterior->>'pes_domain' AS "domain!", COUNT(x.*) AS "total!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') *
  FROM c_opportunity o
  WHERE
    c_opportunity_is_current(o.interior, o.exterior) AND
    o.exterior->>'pes_domain' != 'unspecified' AND
    o.exterior->>'location_type' = 'near'
  ORDER BY o.exterior->>'title', o.exterior->>'partner'
) x
GROUP BY x.exterior->>'pes_domain'
ORDER BY "total!" DESC
"#
        )
        .map(|row| OverviewDomain {
            name: row.domain.to_owned(),
            value: row.total,
        })
        .fetch_all(&db)
        .await?;

        (
            points_total,
            points_domains,
            polygons_total,
            polygons_domains,
        )
    };

    Ok(json!(
        {
            "max": points_total.max(polygons_total).max(anywhere_total),
            "points": {
                "total": points_total,
                "domains": points_domains
            },
            "polygons": {
                "total": polygons_total,
                "domains": polygons_domains
            },
            "anywhere": {
                "total": anywhere_total,
                "domains": anywhere_domains,
            },
        }
    ))
}

pub async fn opps_regional_overview_calc(db: Database) -> Result<serde_json::Value, sqlx::Error> {
    let anywhere = sqlx::query_scalar!(
        r#"
SELECT COUNT(x.*) AS "result!"
FROM (
  SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
  FROM c_opportunity o
  WHERE
    o.exterior->>'location_type' = 'any' AND
    c_opportunity_is_current(o.interior, o.exterior)
  ORDER BY exterior->>'title', exterior->>'partner'
) x
"#
    )
    .fetch_one(&db)
    .await?;

    // !! SQLx versions <= 0.7.1 fail to parse the EXPLAIN output for this query during compile
    //     let states: BTreeMap<String, OverviewState> = sqlx::query!(
    //         r#"
    // SELECT
    //   r."name" AS "name!",
    //   (
    //     SELECT COUNT(x.*)
    //     FROM (
    //       SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
    //       FROM c_opportunity o
    //       WHERE
    //         c_opportunity_is_current(o.interior, o.exterior) AND
    //         ST_Intersects(r.geometry, o.location_point)
    //       ORDER BY o.exterior->>'title', o.exterior->>'partner'
    //     ) x
    //   ) AS "point!",
    //   (
    //     SELECT COUNT(x.*)
    //     FROM (
    //       SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
    //       FROM c_opportunity o
    //       WHERE
    //         c_opportunity_is_current(o.interior, o.exterior) AND
    //         ST_Intersects(r.geometry, o.location_polygon)
    //       ORDER BY o.exterior->>'title', o.exterior->>'partner'
    //     ) x
    //   ) AS "polygon!"
    //   FROM c_region r
    // "#
    //     )
    //     .map(|row| {
    //         (
    //             row.name.to_owned(),
    //             OverviewState {
    //                 point: row.point,
    //                 polygon: row.polygon,
    //             },
    //         )
    //     })
    //     .fetch_all(&db)
    //     .await?
    //     .into_iter()
    //     .collect();

    // Temporary runtime solution instead
    use sqlx::Row;
    let states: BTreeMap<String, OverviewState> = sqlx::query(
        r#"
SELECT
  r."name" AS "name!",
  (
    SELECT COUNT(x.*)
    FROM (
      SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
      FROM c_opportunity o
      WHERE
        c_opportunity_is_current(o.interior, o.exterior) AND
        ST_Intersects(r.geometry, o.location_point)
      ORDER BY o.exterior->>'title', o.exterior->>'partner'
    ) x
  ) AS "point!",
  (
    SELECT COUNT(x.*)
    FROM (
      SELECT DISTINCT ON (o.exterior->>'title', o.exterior->>'partner') 1
      FROM c_opportunity o
      WHERE
        c_opportunity_is_current(o.interior, o.exterior) AND
        ST_Intersects(r.geometry, o.location_polygon)
      ORDER BY o.exterior->>'title', o.exterior->>'partner'
    ) x
  ) AS "polygon!"
FROM c_region r
"#,
    )
    .map(|row: sqlx::postgres::PgRow| {
        (
            row.get("name!"),
            OverviewState {
                point: row.get("point!"),
                polygon: row.get("polygon!"),
            },
        )
    })
    .fetch_all(&db)
    .await?
    .into_iter()
    .collect();
    // End of temporary runtime solution

    let counts = opp_regional_detailed_counts(db.clone(), None).await?;

    let json = json!({"data": {"anywhere": anywhere, "states": states}, "counts": counts});

    cache_json(&db, "geoexp-regional-overview", &json).await?;

    Ok(json)
}
