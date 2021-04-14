use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::http::{mime, StatusCode};
use tide::prelude::*;
use tide::Response;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;
//use tide_websockets::{Message, WebSocket};
use std::convert::TryInto;
use uuid::Uuid;

use crate::model;

pub(crate) fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("opportunity", |r| r.post(opportunity_new))
        .at("opportunity/:uid", |r| {
            r.get(opportunity_get)
                .put(opportunity_put)
                .delete(opportunity_delete)
        })
}

async fn opportunity_new(mut req: tide::Request<()>) -> tide::Result {
    if let Some(res) = sanity_check(&req, true) {
        return Ok(res);
    }

    let mut opp: model::Opportunity = req.body_json().await?;

    if let Err(err) = opp.validate() {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    let mut db = req.sqlx_conn::<Postgres>().await;

    if model::Opportunity::exists_by_uid(db.acquire().await?, &opp.exterior.uid).await? {
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

async fn opportunity_get(mut req: tide::Request<()>) -> tide::Result {
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

    println!(
        "{:?}",
        model::Opportunity::load_by_id(db.acquire().await?, 1).await
    );

    Ok("Hello web world".into())
}

async fn opportunity_put(mut req: tide::Request<()>) -> tide::Result {
    let uid: Uuid = match req.param("uid")?.parse() {
        Ok(uid) => uid,
        Err(_) => {
            return Ok(error(
                StatusCode::BadRequest,
                "Unable to parse a UUID from the request path",
            ));
        }
    };

    // Always set interior.withdrawn to true

    let mut db = req.sqlx_conn::<Postgres>().await;

    println!(
        "{:?}",
        model::Opportunity::load_by_id(db.acquire().await?, 1).await
    );

    Ok("Hello web world".into())
}

async fn opportunity_delete(mut req: tide::Request<()>) -> tide::Result {
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

    println!(
        "{:?}",
        model::Opportunity::load_by_id(db.acquire().await?, 1).await
    );
    Ok("Hello web world".into())
}

fn error<S, M>(code: S, msg: M) -> Response
where
    S: TryInto<StatusCode>,
    S::Error: std::fmt::Debug,
    M: AsRef<str>,
{
    Response::builder(code)
        .content_type(mime::JSON)
        .body(json!({ "message": msg.as_ref() }))
        .build()
}

fn sanity_check(req: &tide::Request<()>, authorization: bool) -> Option<Response> {
    if let Some(ct) = req.content_type() {
        if ct != mime::JSON {
            return Some(error(
                StatusCode::BadRequest,
                "Content-Type header must specify application/json",
            ));
        }
    } else {
        return Some(error(
            StatusCode::BadRequest,
            "Content-Type header is required",
        ));
    }

    if authorization {
        if let Some(auth) = req.header("Authorization") {
            if !auth.last().as_str().starts_with("Bearer ") {
                return Some(error(
                    StatusCode::Unauthorized,
                    "The Authorization header must contain the string 'Bearer ' followed by a partner authorization token",
                ));
            }
        } else {
            return Some(error(
                StatusCode::Unauthorized,
                "The Authorization header is required",
            ));
        }
    }

    None
}
