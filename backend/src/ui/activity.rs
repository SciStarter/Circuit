use common::Database;
use once_cell::sync::Lazy;
use serde::Deserialize;
use tide::{Response, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use super::request_person;

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
        .at("transit", |r| r.post(record_transit))
        .at("external", |r| r.post(record_external))
        .at("widget", |r| r.post(record_widget))
}

#[derive(Debug, Deserialize)]
struct TransitInfo {
    prior: Uuid,
    postor: Uuid,
}

pub async fn record_transit(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .and_then(|x| Some(x.exterior.uid));

    let info: TransitInfo = req.body_json().await?;

    let db = req.state();

    sqlx::query!(
        r#"insert into c_transit ("prior", "postor", "actor") values ($1, $2, $3)"#,
        info.prior,
        info.postor,
        person
    )
    .execute(db)
    .await?;

    Ok(Response::builder(StatusCode::NoContent).build())
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
pub async fn record_external(mut req: tide::Request<Database>) -> tide::Result {
    let body = req.body_json::<serde_json::Value>().await?;

    sqlx::query!(
        r#"INSERT INTO c_log ("action", "object") values ('external', $1)"#,
        Uuid::try_parse(
            body["object"]
                .as_str()
                .unwrap_or("00000000-0000-0000-0000-000000000000")
        )
        .unwrap_or_else(|_| Uuid::nil())
    )
    .execute(req.state())
    .await?;

    if cfg!(not(debug_assertions)) {
        async_std::task::spawn(surf::post(&*CLICK_ENDPOINT).body(body).send());
    }

    Ok(Response::builder(StatusCode::Ok).build())
}

#[derive(Deserialize, Debug)]
struct RecordWidgetForm {
    site: String,
}

/// Record when a widget has been loaded on an external site
pub async fn record_widget(mut req: tide::Request<Database>) -> tide::Result {
    if let Ok(body) = dbg!(req.body_json::<RecordWidgetForm>().await) {
        sqlx::query(r#"insert into c_widget_views ("site") values ($1)"#)
            .bind(body.site)
            .execute(req.state())
            .await
            .ok();
    }

    Ok(Response::builder(StatusCode::Ok).build())
}
