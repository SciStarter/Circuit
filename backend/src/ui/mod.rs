pub mod activity;
pub mod auth;
pub mod entity;
pub mod finder;
pub mod invitation;
pub mod misc;
pub mod opportunity;
pub mod organization;
pub mod profile;

use chrono::NaiveDate;
use common::model::{self, Person};
use common::Database;
use http_types::{mime, Cookie};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::http::StatusCode;
use tide::Response;
use tide_fluent_routes::prelude::*;

use self::auth::TOKEN_COOKIE;

pub static UI_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("0be35cad-2b1f-4a45-a6da-b1051643c6f6").unwrap());

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("activity/", activity::routes)
        .at("auth/", auth::routes)
        .at("entity/", entity::routes)
        .at("finder/", finder::routes)
        .at("profile/", profile::routes)
        .at("organization/", organization::routes)
        .at("opportunity/", opportunity::routes)
        .at("invitation/", invitation::routes)
        .at("misc/", misc::routes)
        .at("content", |r| r.get(content))
        .at("timezone", |r| r.get(timezone))
}

pub fn okay_empty() -> tide::Result<Response> {
    Ok(Response::builder(StatusCode::NoContent).build())
}

pub fn okay<P>(payload: &P) -> tide::Result<Response>
where
    P: Serialize + ?Sized,
{
    Ok(Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(serde_json::to_value(payload)?)
        .build())
}

pub fn okay_with_cookie<P>(payload: &P, cookie: Cookie<'static>) -> tide::Result<Response>
where
    P: Serialize + ?Sized,
{
    match okay(payload) {
        Ok(mut resp) => {
            resp.insert_cookie(cookie);
            Ok(resp)
        }
        Err(e) => Err(e),
    }
}

/// Generates a JSON object which represents a person, suitable for
/// sending to the front-end.
fn person_json(person: &Person, token: &String) -> serde_json::Value {
    json!({
        "authenticated": true,
        "uid": person.exterior.uid.to_string(),
        "token": token.clone(),
        "username": person.exterior.username.clone(),
        "image_url": person.exterior.image_url.clone(),
    })
}

async fn request_person(req: &mut tide::Request<Database>) -> tide::Result<Option<Person>> {
    let token = if let Some(val) = req.header("Authorization").and_then(|vals| vals.get(0)) {
        if let Some((mode, token)) = val.as_str().split_once(" ") {
            if mode == "Bearer" {
                token.to_string()
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
    } else {
        if let Some(val) = req.cookie(TOKEN_COOKIE) {
            val.value().to_string()
        } else {
            return Ok(None);
        }
    };

    let uid = match common::jwt::check_jwt(&token, &UI_AUDIENCE) {
        Ok(checked) => checked,
        Err(_) => return Ok(None),
    };

    let db = req.state();

    let person = match Person::load_by_uid(db, &uid).await {
        Ok(loaded) => loaded,
        Err(_) => return Ok(None),
    };

    Ok(Some(person))
}

#[derive(Deserialize)]
struct ContentQuery {
    pub language: String,
    pub group: String,
    pub item: String,
}

pub async fn content(req: tide::Request<Database>) -> tide::Result {
    let query: ContentQuery = req.query()?;

    let db = req.state();

    if let Ok(block) =
        model::block::Block::load(db, &query.language, &query.group, &query.item).await
    {
        if !block.content.is_empty() {
            Ok(Response::builder(StatusCode::Ok)
                .body(serde_json::to_value(block)?)
                .build())
        } else {
            Ok(Response::builder(StatusCode::NotFound)
                .body(format!("{} {} {}", query.language, query.group, query.item))
                .build())
        }
    } else {
        // This was meant to make editing easier, but actually just
        // resulted in flooding the menus with the results of various
        // attempts to use SQL injection attacks on the site.

        // let mut block = model::block::Block {
        //     id: None,
        //     language: query.language.to_string(),
        //     group: query.group.to_string(),
        //     item: query.item.to_string(),
        //     ..Default::default()
        // };

        // block.store(db).await?;

        Ok(Response::builder(StatusCode::NotFound)
            .body(format!("{} {} {}", query.language, query.group, query.item))
            .build())
    }
}

#[derive(Deserialize, Debug)]
struct TimezoneForm {
    name: String,
    date: NaiveDate,
}

pub async fn timezone(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    if person.is_none() {
        return Err(tide::Error::from_str(StatusCode::Forbidden, "forbidden"));
    }

    if let Ok(form) = req.query::<TimezoneForm>() {
        okay(&common::timezones::timezone(form.name, form.date)?)
    } else {
        okay(&common::timezones::timezones())
    }
}
