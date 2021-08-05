use common::Database;
use tide::{Response, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("click", |r| r.post(record_click))
        .at("external", |r| r.post(record_external))
}

/// Update the clickstream with a single on-site click instance. No-op
/// when compiled in debug mode.
pub async fn record_click(mut _req: tide::Request<Database>) -> tide::Result {
    if cfg!(not(debug_assertions)) {
        todo!()
    } else {
        Ok(Response::builder(StatusCode::Ok).build())
    }
}

/// Record an instance of a user clicking on an external link. No-op
/// when compiled in debug mode.
pub async fn record_external(mut _req: tide::Request<Database>) -> tide::Result {
    if cfg!(not(debug_assertions)) {
        todo!()
    } else {
        Ok(Response::builder(StatusCode::Ok).build())
    }
}
