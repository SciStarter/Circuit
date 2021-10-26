use chrono::{DateTime, FixedOffset};
use common::{
    geo,
    model::{
        opportunity::{
            Cost, Descriptor, EntityType, OpportunityQuery, OpportunityQueryOrdering,
            OpportunityQueryPhysical, Topic, VenueType,
        },
        Opportunity, OpportunityExterior, Pagination, SelectOption,
    },
    Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::ui::{okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("partners", |r| r.get(partners))
        .at("descriptors", |r| r.get(descriptors))
        .at("topics", |r| r.get(topics))
        .at("activities", |r| r.get(activities))
        .at("random-categories", |r| r.get(random_categories))
        .at("search", |r| r.get(search))
        .at("geo", |r| r.post(geo))
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

#[derive(Serialize, Deserialize, Clone)]
struct GeoPlace {
    near: String,
    longitude: f32,
    latitude: f32,
    proximity: f32,
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

pub async fn geo(mut req: tide::Request<Database>) -> tide::Result {
    let search: GeoQuery = req.body_json().await?;
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
    results.sort_unstable_by_key(|m| -(m.confidence as i32));

    let places = GeoResult {
        places: results
            .iter()
            .map(|m| GeoPlace {
                near: m.formatted.to_string(),
                longitude: m.geometry.longitude,
                latitude: m.geometry.latitude,
                proximity,
            })
            .collect(),
    };

    okay(&places)
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
    pub min_age: Option<i16>,
    pub max_age: Option<i16>,
    pub topics: Option<Vec<Topic>>,
    pub descriptors: Option<Vec<Descriptor>>,
    pub cost: Option<Cost>,
    pub venue_type: Option<VenueType>,
    pub host: Option<String>,
    pub partner: Option<Uuid>,
    pub sort: Option<OpportunityQueryOrdering>,
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub saved: Option<bool>,
    pub participated: Option<bool>,
    pub reviewing: Option<bool>,
    pub withdrawn: Option<bool>,
}

pub async fn search(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let db = req.state();

    let search: SearchQuery = req.query()?;

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
    query.descriptors = search.descriptors;
    query.cost = search.cost;
    query.topics = search.topics;
    query.venue_type = search.venue_type;
    query.host = search.host;
    query.partner = search.partner;

    if let Some(p) = person {
        if search.saved.unwrap_or(false) {
            query.saved = Some(p.exterior.uid.clone());
        }

        if search.participated.unwrap_or(false) {
            query.participated = Some(p.exterior.uid.clone());
        }

        match (search.reviewing, search.withdrawn) {
            (Some(reviewing), None) => {
                query.partner_member = Some(p.exterior.uid.clone());
                query.accepted = Some(!reviewing);
                query.withdrawn = Some(false);
            }
            (None, Some(withdrawn)) => {
                query.partner_member = Some(p.exterior.uid.clone());
                query.accepted = None;
                query.withdrawn = Some(withdrawn);
            }
            (Some(reviewing), Some(withdrawn)) => {
                query.partner_member = Some(p.exterior.uid.clone());
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

    if let (Some(longitude), Some(latitude), Some(proximity)) =
        (search.longitude, search.latitude, search.proximity)
    {
        query.near = Some((longitude, latitude, proximity));
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

    let matches: Vec<OpportunityExterior> = Opportunity::load_matching(
        db,
        dbg!(&query),
        search.sort.unwrap_or_default(),
        pagination,
    )
    .await?
    .into_iter()
    .map(|m| m.exterior)
    .collect();

    let total = Opportunity::count_matching(db, &query).await?;

    let (page_index, last_page, per_page) = pagination.expand(total);

    common::log("ui-search", &req.url().query());

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
