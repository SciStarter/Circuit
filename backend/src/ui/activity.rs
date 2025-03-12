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
        .at("view", |r| r.post(record_view))
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

// #[derive(serde::Deserialize)]
// struct Lookup {
//     // country_code2: String,
//     // country_name: String,
//     // state_prov: String, // State
//     // district: String,   // County
//     // city: String,
//     // zipcode: String,
//     longitude: String,
//     latitude: String,
// }

// async fn ip_coords_lookup(ip_address: String, view_id: i32, db: Database) {
//     if let Ok(key) = std::env::var("IPGEOLOCATION_KEY") {
//         match surf::get(format!(
//             "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
//             key, &ip_address
//         ))
//         .send()
//         .await
//         {
//             Ok(mut resp) => match resp.body_json().await {
//                 Ok(Lookup {
//                     longitude,
//                     latitude,
//                     ..
//                 }) => {
//                     let (Ok(lon), Ok(lat)) = (longitude.parse::<f32>(), latitude.parse::<f32>())
//                     else {
//                         eprintln!("Error parsing lon/lat for IP Geolocation");
//                         return;
//                     };

//                     if let Err(err) = sqlx::query!(
//                         r#"insert into c_ip_coords ("ip", "lon", "lat")
//                            values ($1, $2, $3)
//                            on conflict do nothing"#,
//                         ip_address,
//                         lon,
//                         lat
//                     )
//                     .execute(&db)
//                     .await
//                     {
//                         eprintln!("Error saving IP Geolocation: {}", err);
//                         return;
//                     }

//                     if let Err(err) = sqlx::query!(
//                         r#"update c_views set "lon" = $2, "lat" = $3 where "id" = $1"#,
//                         view_id,
//                         lon,
//                         lat
//                     )
//                     .execute(&db)
//                     .await
//                     {
//                         eprintln!("Error saving IP Geolocation for view: {}", err);
//                         return;
//                     }
//                 }
//                 Err(err) => eprintln!("Error while decoding IP Geolocation: {}", err),
//             },
//             Err(err) => eprintln!("Error while retriving IP Geolocation: {}", err),
//         }
//     }
// }

#[derive(serde::Deserialize)]
struct View {
    session: String,
    page: String,
    longitude: Option<f32>,
    latitude: Option<f32>,
}

/// Update the view log with a single page view.
pub async fn record_view(mut req: tide::Request<Database>) -> tide::Result {
    // dbg!(req.iter().collect::<Vec<_>>());

    let view: View = req.body_json().await?;

    let person_id = request_person(&mut req).await?.and_then(|p| p.id);

    let Some(ip_address) = req.remote() else {
        return Ok(Response::builder(StatusCode::BadRequest).build());
    };

    let ip_address = ip_address.to_owned();

    sqlx::query!(
        r#"insert into c_views ("when", "page", "user", "session", "ip", "lon", "lat")
               values (CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6)"#,
        view.page,
        person_id,
        view.session,
        ip_address,
        view.longitude,
        view.latitude,
    )
    .execute(req.state())
    .await?;

    // struct Coords {
    //     lon: f32,
    //     lat: f32,
    // }
    //
    // if let Some(coords) = sqlx::query_as!(
    //     Coords,
    //     r#"select "lon" as "lon!", "lat" as "lat!" from c_ip_coords where "ip" = $1"#,
    //     ip_address
    // )
    // .fetch_optional(req.state())
    // .await?
    // {
    //     sqlx::query!(
    //         r#"insert into c_views ("when", "page", "user", "session", "ip", "lon", "lat")
    //            values (CURRENT_TIMESTAMP, $1, $2, $3, $4, $5, $6)"#,
    //         view.page,
    //         person_id,
    //         view.session,
    //         ip_address,
    //         coords.lon,
    //         coords.lat,
    //     )
    //     .execute(req.state())
    //     .await?;
    // } else {
    //     let view_id = sqlx::query_scalar!(
    //         r#"insert into c_views ("when", "page", "user", "session", "ip")
    //            values (CURRENT_TIMESTAMP, $1, $2, $3, $4)
    //            returning "id""#,
    //         view.page,
    //         person_id,
    //         view.session,
    //         ip_address,
    //     )
    //     .fetch_one(req.state())
    //     .await?;

    //     let db = req.state().clone();
    //     async_std::task::spawn(ip_coords_lookup(ip_address, view_id, db));
    // }

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
