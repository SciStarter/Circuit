use crate::model::opportunity::Opportunity;
use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::http::{mime, StatusCode};
use tide::Response;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;
use uuid::Uuid;

use super::{error, header_check, success};

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .post(opportunity_new)
        .at(":uid", |r| r.get(opportunity_get).put(opportunity_put))
}

async fn opportunity_new(mut req: tide::Request<()>) -> tide::Result {
    let auth = match header_check(&req) {
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

    opp.exterior.partner = auth;
    opp.interior.accepted = false;

    if let Err(err) = opp.validate() {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    let mut db = req.sqlx_conn::<Postgres>().await;

    if Opportunity::exists_by_uid(db.acquire().await?, &opp.exterior.uid).await? {
        return Ok(error(
            StatusCode::Conflict,
            "An opportunity with that uid (or partner_name and title) already exists.",
        ));
    }

    opp.store(db.acquire().await?).await?;

    let res = Response::builder(StatusCode::Created)
        .content_type(mime::JSON)
        .body(serde_json::to_value(opp)?)
        .build();

    Ok(res)
}

async fn opportunity_get(req: tide::Request<()>) -> tide::Result {
    let auth = match header_check(&req) {
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

    let mut db = req.sqlx_conn::<Postgres>().await;

    let opp = match Opportunity::load_by_uid(db.acquire().await?, &uid).await {
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

async fn opportunity_put(mut req: tide::Request<()>) -> tide::Result {
    let auth = match header_check(&req) {
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

    let mut db = req.sqlx_conn::<Postgres>().await;

    let old_opp = match Opportunity::load_by_uid(db.acquire().await?, &uid).await {
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

    new_opp.store(db.acquire().await?).await?;

    success(&new_opp)
}
