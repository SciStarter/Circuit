use common::model;
use std::convert::TryInto;
use tide::http::{mime, StatusCode};
use tide::Response;
use tide::{prelude::*, ResponseBuilder};
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

use common::jwt::{check_jwt, issue_jwt};

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("click", |r| r.post(record_click))
        .at("external", |r| r.post(record_external))
        .at("login", |r| r.post(login))
        .at("signup", |r| r.post(signup))
}

pub async fn record_click(mut req: tide::Request<()>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok).build())
}

pub async fn record_external(mut req: tide::Request<()>) -> tide::Result {
    Ok(Response::builder(StatusCode::Ok).build())
}

pub async fn login(mut req: tide::Request<()>) -> tide::Result {
    unimplemented!()
}

pub async fn signup(mut req: tide::Request<()>) -> tide::Result {
    unimplemented!()
}
