use common::model;
use sqlx::postgres::Postgres;
//use std::convert::TryInto;
//use tide::http::mime;
use serde::Deserialize;
use tide::http::StatusCode;
use tide::Response;
//use tide::{prelude::*, ResponseBuilder};
use sqlx::prelude::*;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;
//use uuid::Uuid;

//use common::jwt::{check_jwt, issue_jwt};

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("click", |r| r.post(record_click))
        .at("external", |r| r.post(record_external))
        .at("login", |r| r.post(login))
        .at("signup", |r| r.post(signup))
        .at("me", |r| r.get(me))
        .at("content", |r| r.get(content))
}

/// Update the clickstream with a single on-site click instance. No-op
/// when compiled in debug mode.
pub async fn record_click(mut _req: tide::Request<()>) -> tide::Result {
    if cfg!(not(debug_assertions)) {
        todo!()
    } else {
        Ok(Response::builder(StatusCode::Ok).build())
    }
}

/// Record an instance of a user clicking on an external link. No-op
/// when compiled in debug mode.
pub async fn record_external(mut _req: tide::Request<()>) -> tide::Result {
    if cfg!(not(debug_assertions)) {
        todo!()
    } else {
        Ok(Response::builder(StatusCode::Ok).build())
    }
}

/// Log the current session in to an account, if the validations pass.
///
/// Set a token cookie with the HttpOnly, SameSite=Lax, and (when
/// ```#[cfg(not(debug_assertions))]```) Secure flags, and also
/// return the token in the response body for script use.
pub async fn login(mut _req: tide::Request<()>) -> tide::Result {
    unimplemented!()
}

/// Create an account, if the validations pass.
///
/// Set a token cookie with the HttpOnly, SameSite=Lax, and (when
/// ```#[cfg(not(debug_assertions))]```) Secure flags, and also
/// return the token in the response body for script use.
pub async fn signup(mut _req: tide::Request<()>) -> tide::Result {
    unimplemented!()
}

/// Retrieve UI-approriate information about the current user in JSON
/// format. Includes the token, display name, and so on.
pub async fn me(mut _req: tide::Request<()>) -> tide::Result {
    unimplemented!()
}

#[derive(Deserialize)]
struct ContentQuery {
    pub language: String,
    pub group: String,
    pub item: String,
}

pub async fn content(req: tide::Request<()>) -> tide::Result {
    let query: ContentQuery = req.query()?;

    let mut db = req.sqlx_conn::<Postgres>().await;

    if let Ok(block) = model::block::Block::load(
        db.acquire().await?,
        &query.language,
        &query.group,
        &query.item,
    )
    .await
    {
        Ok(Response::builder(StatusCode::Ok)
            .body(block.content)
            .build())
    } else {
        Ok(Response::builder(StatusCode::NotFound)
            .body(format!("{} {} {}", query.language, query.group, query.item))
            .build())
    }
}
