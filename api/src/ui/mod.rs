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
        .at("content", |r| r.get(content))
}

pub async fn record_click(mut _req: tide::Request<()>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok).build())
}

pub async fn record_external(mut _req: tide::Request<()>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok).build())
}

pub async fn login(mut _req: tide::Request<()>) -> tide::Result {
    unimplemented!()
}

pub async fn signup(mut _req: tide::Request<()>) -> tide::Result {
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
