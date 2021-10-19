use common::Database;
use once_cell::sync::Lazy;
use tide::{Response, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

static CLICK_ENDPOINT: Lazy<String> = Lazy::new(|| {
    format!(
        "http://{}:{}/internal/click",
        std::env::var("CIRCUIT_LOGGER_SERVICE_SERVICE_HOST").unwrap_or_else(|_| std::env::var(
            "CIRCUIT_LOGGER_SERVICE_BETA_SERVICE_HOST"
        )
        .unwrap_or_else(|_| "localhost".to_string())),
        std::env::var("CIRCUIT_LOGGER_SERVICE_SERVICE_PORT").unwrap_or_else(|_| std::env::var(
            "CIRCUIT_LOGGER_SERVICE_BETA_SERVICE_PORT"
        )
        .unwrap_or_else(|_| "9000".to_string())),
    )
});

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("click", |r| r.post(record_click))
        .at("external", |r| r.post(record_external))
}

/// Update the clickstream with a single on-site click instance. No-op
/// when compiled in debug mode.
pub async fn record_click(mut req: tide::Request<Database>) -> tide::Result {
    if cfg!(not(debug_assertions)) {
        async_std::task::spawn(
            surf::post(&*CLICK_ENDPOINT)
                .body(req.body_json::<serde_json::Value>().await?)
                .send(),
        );
    }

    Ok(Response::builder(StatusCode::Ok).build())
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
