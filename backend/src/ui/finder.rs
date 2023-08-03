use chrono::{DateTime, FixedOffset};
use common::{
    geo,
    model::{
        opportunity::{
            Cost, Descriptor, EntityType, OpportunityQuery, OpportunityQueryOrdering,
            OpportunityQueryPhysical, OpportunityQueryTemporal, Topic, VenueType,
        },
        person::Permission,
        Opportunity, OpportunityExterior, Pagination, Person, SelectOption,
    },
    Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::StatusCode;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::ui::{okay, request_person};

const FIFTY_MILES: f32 = 80467.0;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("partners", |r| r.get(partners))
        .at("descriptors", |r| r.get(descriptors))
        .at("topics", |r| r.get(topics))
        .at("activities", |r| r.get(activities))
        .at("metros", |r| r.get(metros))
        .at("states", |r| r.get(states))
        .at("random-categories", |r| r.get(random_categories))
        .at("search", |r| r.get(search))
        .at("geo", |r| r.post(geo))
        .at("geom", |r| r.post(geom))
}

pub async fn partners(req: tide::Request<Database>) -> tide::Result {
    let db = req.state();

    let refs = common::model::Partner::catalog(db).await?;

    okay(&refs)
}

pub async fn descriptors(_req: tide::Request<Database>) -> tide::Result {
    okay(&json!(Descriptor::all_options()
        .into_iter()
        .map(|(a, b, _)| (a, b))
        .collect::<Vec<_>>()))
}

pub async fn topics(_req: tide::Request<Database>) -> tide::Result {
    okay(&json!(Topic::all_options()
        .into_iter()
        .map(|(a, b, _)| (a, b))
        .collect::<Vec<_>>()))
}

pub async fn activities(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GeoPlace {
    near: String,
    longitude: f32,
    latitude: f32,
    proximity: f32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
enum GeoLookup {
    Coords,
    Near,
}

#[derive(Deserialize, Debug)]
struct GeoQuery {
    lookup: GeoLookup,
    place: GeoPlace,
}

#[derive(Serialize, Debug)]
struct GeoResult {
    places: Vec<GeoPlace>,
}

pub async fn geo(mut req: tide::Request<Database>) -> tide::Result {
    let search: GeoQuery = match req.body_json().await {
        Ok(search) => search,
        Err(_) => {
            return okay(&GeoResult {
                places: vec![GeoPlace {
                    near: String::new(),
                    longitude: 0.0,
                    latitude: 0.0,
                    proximity: 0.0,
                }],
            });
        }
    };

    if search.place.near.is_empty() && search.place.latitude == 0.0 && search.place.longitude == 0.0
    {
        return okay(&GeoResult {
            places: vec![GeoPlace {
                near: String::new(),
                longitude: 0.0,
                latitude: 0.0,
                proximity: 0.0,
            }],
        });
    }

    if let GeoLookup::Near = search.lookup {
        if (search.place.latitude != 0.0 || search.place.longitude != 0.0)
            && search.place.near.is_empty()
        {
            if let Some(mut person) = request_person(&mut req).await? {
                person.interior.recent_point = Some(
                    json!({"type": "Point", "geometry": [search.place.longitude, search.place.latitude]}),
                );
                person.store(req.state()).await?;
            }
        }
    }

    let proximity = search.place.proximity;

    let query = geo::Query::new(
        match search.lookup {
            GeoLookup::Coords => search.place.near,
            GeoLookup::Near => format!("{} {}", search.place.latitude, search.place.longitude),
        },
        false,
    );

    let result = query.lookup().await?;

    if result.status.code != 200 {
        return Err(tide::Error::from_str(
            result.status.code,
            "Geographic lookup failed",
        ));
    }

    let mut results = result.results.clone();
    results.sort_unstable_by_key(|m| (m.confidence as i32));

    let places = GeoResult {
        places: results
            .iter()
            .map(|m| GeoPlace {
                near: format!(
                    "{}, {} {}, {}",
                    if let Some(city) = &m.components.city {
                        city.clone()
                    } else if let Some(town) = &m.components.town {
                        town.clone()
                    } else if let Some(county) = &m.components.county {
                        county.clone()
                    } else {
                        String::new()
                    },
                    m.components.state_code,
                    if let Some(code) = &m.components.postcode {
                        if let Some((before, _)) = code.split_once('-') {
                            before.to_string()
                        } else {
                            code.to_string()
                        }
                    } else {
                        String::new()
                    },
                    m.components.country
                ),
                longitude: m.geometry.longitude,
                latitude: m.geometry.latitude,
                proximity,
            })
            .collect(),
    };

    okay(&places)
}

#[derive(Deserialize, Debug)]
struct GeomQuery {
    q: String,
}

enum End {
    Front,
    Back,
}

pub async fn geom(mut req: tide::Request<Database>) -> tide::Result {
    let search: GeomQuery = req.body_json().await?;
    let mut query = geo::GeomQuery::new(search.q, 0.5);

    let mut end = End::Back;

    while !query.q.is_empty() {
        match query.lookup(req.state()).await {
            Ok(result) => return okay(&result),
            Err(_) => {
                let mut buf: Vec<_> = query.q.split(',').collect();
                end = match end {
                    End::Front => {
                        buf = buf.into_iter().skip(1).collect();
                        End::Back
                    }
                    End::Back => {
                        buf.pop();
                        End::Front
                    }
                };
                query.q = buf.join(", ");
            }
        }
    }

    Err(tide::Error::from_str(
        StatusCode::BadRequest,
        "Could not find geometry for the requested location name",
    ))
}

#[derive(Deserialize)]
struct RandomCategoriesQuery {
    _num: u16,
}

pub async fn random_categories(req: tide::Request<Database>) -> tide::Result {
    let _query: RandomCategoriesQuery = req.query()?;

    todo!()
}

#[derive(Deserialize, Debug)]
struct SearchQuery {
    pub longitude: Option<f32>,
    pub latitude: Option<f32>,
    pub proximity: Option<f32>,
    pub online: Option<bool>,
    pub text: Option<String>,
    pub beginning: Option<DateTime<FixedOffset>>,
    pub ending: Option<DateTime<FixedOffset>>,
    pub physical: Option<OpportunityQueryPhysical>,
    pub temporal: Option<OpportunityQueryTemporal>,
    pub min_age: Option<i16>,
    pub max_age: Option<i16>,
    pub topics: Option<Vec<Topic>>,
    pub descriptors: Option<Vec<Descriptor>>,
    pub cost: Option<Cost>,
    pub venue_type: Option<VenueType>,
    pub host: Option<String>,
    pub partner: Option<Uuid>,
    pub prefer_partner: Option<Uuid>,
    pub mine: Option<bool>,
    pub sort: Option<OpportunityQueryOrdering>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub saved: Option<bool>,
    pub participated: Option<bool>,
    pub reviewing: Option<bool>,
    pub current: Option<bool>,
    pub withdrawn: Option<bool>,
    pub sample: Option<bool>,
    pub kids_only: Option<bool>,
    pub adults_only: Option<bool>,
    pub year: Option<u32>,
    pub month: Option<u8>,
    pub refs: Option<bool>,
}

pub async fn search(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;
    let person_uid = person.as_ref().map(|p| p.exterior.uid.clone());

    let db = req.state();

    let search: SearchQuery = req.query()?;

    if let (Some(person_id), Some(longitude), Some(latitude)) = (
        person.as_ref().and_then(|p| p.id.clone()),
        &search.longitude,
        &search.latitude,
    ) {
        // This query is constructed like this so as to be able to
        // detect changes, without creating a race condition
        let changed_location = sqlx::query_scalar!(
            r#"
UPDATE c_person post
SET
  "home_location" = COALESCE(post."home_location", ST_SetSRID(ST_Point($2, $3), 4326)),
  "last_location" = ST_SetSRID(ST_Point($2, $3), 4326)
FROM (SELECT "id", "home_location", "last_location" FROM c_person WHERE id = $1 FOR UPDATE) pre
WHERE post.id = pre.id
RETURNING coalesce(pre."home_location" != post."home_location", true) as "changed!"
"#,
            person_id,
            *longitude as f64,
            *latitude as f64
        )
        .fetch_one(db)
        .await?;

        if changed_location {
            Person::update_state_and_metro_by_id(db, person_id).await?;
        }
    }

    let mut query = OpportunityQuery::default();

    query.entity_type = Some(vec![
        EntityType::Opportunity,
        EntityType::Attraction,
        EntityType::Unspecified,
    ]);

    query.text = search.text;
    query.beginning = search.beginning;
    query.ending = search.ending;
    query.min_age = search.min_age;
    query.max_age = search.max_age;
    query.kids_only = search.kids_only;
    query.adults_only = search.adults_only;
    query.descriptors = search.descriptors;
    query.cost = search.cost;
    query.topics = search.topics;
    query.venue_type = search.venue_type;
    query.host = search.host;
    query.partner = search.partner;
    query.prefer_partner = search.prefer_partner;
    query.current = Some(search.current.unwrap_or(true));
    query.temporal = search.temporal;

    query.calendar = match (search.year, search.month) {
        (Some(y), Some(m)) => Some((y, m)),
        _ => None,
    };

    query.sample = if search.sample.unwrap_or(false) {
        Some(0.25)
    } else {
        None
    };

    if let Some(p) = person {
        if search.mine.unwrap_or(false) {
            if !p.check_permission(&Permission::ManageOpportunities) {
                query.partner_member = Some(p.exterior.uid);
            }
        }

        if search.saved.unwrap_or(false) {
            query.saved = Some(p.exterior.uid);
        }

        if search.participated.unwrap_or(false) {
            query.participated = Some(p.exterior.uid);
        }

        match (search.reviewing, search.withdrawn) {
            (Some(reviewing), None) => {
                if !p.check_permission(&Permission::ManageOpportunities) {
                    query.partner_member = Some(p.exterior.uid);
                }
                query.accepted = Some(!reviewing);
                query.withdrawn = Some(false);
            }
            (None, Some(withdrawn)) => {
                if !p.check_permission(&Permission::ManageOpportunities) {
                    query.partner_member = Some(p.exterior.uid);
                }
                query.accepted = None;
                query.withdrawn = Some(withdrawn);
            }
            (Some(reviewing), Some(withdrawn)) => {
                if !p.check_permission(&Permission::ManageOpportunities) {
                    query.partner_member = Some(p.exterior.uid);
                }
                query.accepted = Some(!reviewing);
                query.withdrawn = Some(withdrawn);
            }
            (None, None) => {
                query.accepted = Some(true);
                query.withdrawn = Some(false);
            }
        }
    } else {
        query.accepted = Some(true);
        query.withdrawn = Some(false);
    }

    if search.proximity != Some(0.0) {
        if let (Some(longitude), Some(latitude)) = (search.longitude, search.latitude) {
            query.near = Some((longitude, latitude, search.proximity.unwrap_or(FIFTY_MILES)));
        }
    }

    match (search.online, search.physical) {
        (Some(online), Some(physical)) => {
            query.physical = Some(match (online, physical) {
                (true, OpportunityQueryPhysical::InPerson) => {
                    OpportunityQueryPhysical::InPersonOrOnline
                }
                (true, p) => p,
                (false, _) => OpportunityQueryPhysical::InPerson,
            })
        }
        (None, Some(physical)) => query.physical = Some(physical),
        (Some(online), None) => {
            query.physical = Some(if online {
                OpportunityQueryPhysical::InPersonOrOnline
            } else {
                OpportunityQueryPhysical::InPerson
            })
        }
        (None, None) => {}
    }

    let pagination = if let Some(page) = search.page {
        Pagination::Page {
            index: page,
            size: search.per_page.unwrap_or(10).into(),
        }
    } else {
        Pagination::Page {
            index: 0,
            size: search.per_page.unwrap_or(10).into(),
        }
    };

    let matches: Vec<OpportunityExterior> =
        Opportunity::load_matching(db, &query, search.sort.unwrap_or_default(), pagination)
            .await?
            .into_iter()
            .map(|m| m.exterior)
            .collect();

    let total = Opportunity::count_matching(db, &query).await?;

    let (page_index, last_page, per_page) = pagination.expand(total);

    common::log(person_uid.as_ref(), "ui-search", &req.url().query());

    if search.refs.unwrap_or(false) {
        okay(&json!({
            "pagination": {
                "page_index": page_index,
                "per_page": per_page,
                "last_page": last_page,
                "total": total,
            },
            "matches": matches.into_iter().map(|x| x.into_reference()).collect::<Vec<_>>()
        }))
    } else {
        okay(&json!({
            "pagination": {
                "page_index": page_index,
                "per_page": per_page,
                "last_page": last_page,
                "total": total,
            },
            "matches": matches
        }))
    }
}

pub async fn metros(req: tide::Request<Database>) -> tide::Result {
    let rows = sqlx::query!(r#"SELECT DISTINCT "state" AS "state!", "metro" AS "metro!" FROM c_person WHERE "state" IS NOT NULL AND "metro" IS NOT NULL ORDER BY "state", "metro""#).map(|row| (row.state, row.metro)).fetch_all(req.state()).await?;
    Ok(serde_json::to_string(&rows)?.into())
}

pub async fn states(req: tide::Request<Database>) -> tide::Result {
    let rows = sqlx::query!(r#"SELECT DISTINCT "state" AS "state!" FROM c_person WHERE "state" IS NOT NULL ORDER BY "state""#).map(|row| row.state).fetch_all(req.state()).await?;
    Ok(serde_json::to_string(&rows)?.into())
}
