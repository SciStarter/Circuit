use common::model::opportunity::{
    EntityType, Opportunity, OpportunityImportRecord, OpportunityQuery, OpportunityQueryOrdering,
};
use common::model::Pagination;
use common::Database;
use serde_json::json;
use tide::http::{mime, StatusCode};
use tide::Response;
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

use super::{error, header_check, success};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .post(opportunity_new)
        .get(opportunity_search)
        .at("recommend", |r| r.get(opportunity_recommend))
        .at(
            ":uid",
            |r| r.get(opportunity_get).put(opportunity_put), /*.patch(opportunity_patch)*/
        )
}

async fn opportunity_new(mut req: tide::Request<Database>) -> tide::Result {
    let auth = match header_check(&req, &super::API_AUDIENCE) {
        Ok(x) => match x {
            Some(auth) => auth,
            None => return Ok(error(StatusCode::Unauthorized, "Authorization is required")),
        },
        Err(res) => return Ok(res),
    };

    let mut opp: Opportunity = match req.body_json().await {
        Ok(data) => data,
        Err(err) => {
            return Ok(error(StatusCode::BadRequest, err.to_string()));
        }
    };

    if let EntityType::Page(_) = opp.exterior.entity_type {
        return Ok(error(
            StatusCode::BadRequest,
            "Page entities can not be created via the API".to_string(),
        ));
    }

    opp.exterior.partner = auth;
    opp.interior.accepted = Some(true); // Policy now to trust partners by default

    if let Err(err) = opp.validate().await {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    let db = req.state();

    if Opportunity::exists_by_uid(db, &opp.exterior.uid).await? {
        return Ok(error(
            StatusCode::Conflict,
            "An opportunity with that uid (or partner_name and title) already exists.",
        ));
    }

    if let Err(err) = opp.store(db).await {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    OpportunityImportRecord::store(db, &opp.exterior.partner, &opp.exterior.uid, true, false)
        .await?;

    let res = Response::builder(StatusCode::Created)
        .content_type(mime::JSON)
        .body(serde_json::to_value(opp)?)
        .build();

    Ok(res)
}

async fn opportunity_search(req: tide::Request<Database>) -> tide::Result {
    let auth = match header_check(&req, &super::API_AUDIENCE) {
        Ok(x) => x,
        Err(res) => return Ok(res),
    };

    let mut query: OpportunityQuery = req.query()?;

    if auth.is_some() && query.partner == auth {
        // Request is authenticated and the authenticated partner
        // is the target of the query, so we allow searches to
        // include non-accepted and withdrawn opportunities.
    } else {
        query.accepted = Some(true);
        query.withdrawn = Some(false);
    }

    // Filter out EntityType::Page entries, even if they were
    // requested. They are not meant to be addressed via the API.
    if let Some(requested) = query.entity_type {
        query.entity_type = Some(
            requested
                .into_iter()
                .filter(|t| {
                    if let EntityType::Page(_) = t {
                        false
                    } else {
                        true
                    }
                })
                .collect(),
        );
    } else {
        query.entity_type = Some(vec![EntityType::Opportunity, EntityType::Attraction])
    }

    let db = req.state();

    let matches = Opportunity::load_matching_refs(
        db,
        &query,
        OpportunityQueryOrdering::Alphabetical,
        Pagination::All,
    )
    .await?;

    Ok(Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(json!({ "matches": matches }))
        .build())
}

#[derive(serde::Deserialize)]
struct RecommendQuery {
    _tags: Option<Vec<String>>,
    _topics: Option<Vec<common::model::opportunity::Topic>>,
    r#_abstract: Option<String>,
    _longitude: Option<f32>,
    _latitude: Option<f32>,
}

async fn opportunity_recommend(req: tide::Request<Database>) -> tide::Result {
    let _auth = match header_check(&req, &super::API_AUDIENCE) {
        Ok(x) => x,
        Err(res) => return Ok(res),
    };

    let mut _query: RecommendQuery = req.query()?;

    // !!! TODO

    Ok(Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(json!({ "recommended": [] }))
        .build())
}

async fn opportunity_get(req: tide::Request<Database>) -> tide::Result {
    let auth = match header_check(&req, &super::API_AUDIENCE) {
        Ok(x) => x,
        Err(res) => return Ok(res),
    };

    let uid: Uuid = match req.param("uid")?.parse() {
        Ok(uid) => uid,
        Err(_) => {
            return Ok(error(
                StatusCode::BadRequest,
                "Unable to parse a UUID from the request path",
            ));
        }
    };

    let db = req.state();

    let opp = match Opportunity::load_by_uid(db, &uid).await {
        Ok(opp) => opp,
        Err(_) => {
            return Ok(error(
                StatusCode::NotFound,
                "Could not load opportunity with that uid",
            ));
        }
    };

    match (auth, opp.interior.withdrawn) {
        (Some(auth), _) if auth == opp.exterior.partner => success(&opp),
        (_, false) => success(&opp.exterior),
        _ => Ok(error(
            StatusCode::NotFound,
            "Could not load opportunity with that id",
        )),
    }
}

async fn opportunity_put(mut req: tide::Request<Database>) -> tide::Result {
    let auth = match header_check(&req, &super::API_AUDIENCE) {
        Ok(x) => match x {
            Some(auth) => auth,
            None => return Ok(error(StatusCode::Unauthorized, "Authorization is required")),
        },
        Err(res) => return Ok(res),
    };

    let uid: Uuid = match req.param("uid")?.parse() {
        Ok(uid) => uid,
        Err(_) => {
            return Ok(error(
                StatusCode::BadRequest,
                "Unable to parse a UUID from the request path",
            ));
        }
    };

    let mut new_opp: Opportunity = match req.body_json().await {
        Ok(data) => data,
        Err(err) => {
            return Ok(error(StatusCode::BadRequest, err.to_string()));
        }
    };

    if let EntityType::Page(_) = new_opp.exterior.entity_type {
        return Ok(error(
            StatusCode::BadRequest,
            "Page entities can not be created via the API".to_string(),
        ));
    }

    let db = req.state();

    let old_opp = match Opportunity::load_by_uid(db, &uid).await {
        Ok(opp) => opp,
        Err(_) => {
            return Ok(error(
                StatusCode::NotFound,
                "Could not load opportunity with that uid",
            ));
        }
    };

    if auth != old_opp.exterior.partner {
        return Ok(error(
            StatusCode::Forbidden,
            "Not authorized to edit that opportunity",
        ));
    } else {
        new_opp.exterior.partner = auth;
    }

    if uid != old_opp.exterior.uid {
        return Ok(error(StatusCode::Conflict, "uid mismatch"));
    } else {
        new_opp.exterior.uid = uid;
    }

    new_opp.id = old_opp.id;
    new_opp.interior.accepted = old_opp.interior.accepted;

    if let Err(err) = new_opp.store(db).await {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    OpportunityImportRecord::store(
        db,
        &new_opp.exterior.partner,
        &new_opp.exterior.uid,
        false,
        false,
    )
    .await?;

    success(&new_opp)
}
