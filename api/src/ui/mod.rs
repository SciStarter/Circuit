use common::jwt::issue_jwt;
use common::model::{self, Person};
use http_types::{mime, Cookie};
use once_cell::sync::Lazy;
use serde_json::json;
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
use time::Duration;
//use uuid::Uuid;

//use common::jwt::{check_jwt, issue_jwt};

pub static UI_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("0be35cad-2b1f-4a45-a6da-b1051643c6f6").unwrap());

pub const SESSION_HOURS: i64 = 24 * 90;

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

#[derive(Default, Deserialize)]
struct LoginForm {
    email: String,
    password: String,
}

/// Log the current session in to an account, if the validations pass.
///
/// Set a token cookie with the HttpOnly, SameSite=Lax, and (when
/// ```#[cfg(not(debug_assertions))]```) Secure flags, and also
/// return the token in the response body for script use.
pub async fn login(mut req: tide::Request<()>) -> tide::Result {
    let form: LoginForm = match req.body_json().await {
        Ok(parsed) => parsed,
        Err(_) => {
            return Ok(Response::builder(StatusCode::BadRequest)
                .body("email and password are required")
                .build())
        }
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    let person = match Person::load_by_email(db.acquire().await?, &form.email).await {
        Ok(loaded) => loaded,
        Err(_) => {
            return Ok(Response::builder(StatusCode::Forbidden)
                .body("email or password not recognized")
                .build())
        }
    };

    if person.check_password(&form.password) {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        let mut resp = Response::builder(StatusCode::Ok)
            .content_type(mime::JSON)
            .body(json!({"uid": person.exterior.uid.to_string(), "token": jwt.clone()}))
            .build();

        resp.insert_cookie(
            Cookie::build("token", jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                .http_only(true)
                .same_site(tide::http::cookies::SameSite::Lax)
                .finish(),
        );

        Ok(resp)
    } else {
        Ok(Response::builder(StatusCode::Forbidden)
            .body("email or password not recognized")
            .build())
    }
}

#[derive(Default, Deserialize)]
struct SignupForm {
    email: String,
    password: String,
    username: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
}

/// Create an account, if the validations pass.
///
/// Set a token cookie with the HttpOnly, SameSite=Lax, and (when
/// ```#[cfg(not(debug_assertions))]```) Secure flags, and also
/// return the token in the response body for script use.
pub async fn signup(mut req: tide::Request<()>) -> tide::Result {
    let form: SignupForm = match req.body_json().await {
        Ok(parsed) => parsed,
        Err(_) => {
            return Ok(Response::builder(StatusCode::BadRequest)
                .body("email and password are required")
                .build())
        }
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    if Person::exists_by_email(db.acquire().await?, &form.email).await? {
        return Ok(Response::builder(StatusCode::Forbidden)
            .body("That email is already in use")
            .build());
    }

    let mut person = Person::default();
    person.set_password(&form.password);
    person.exterior.username = form.username;
    person.interior.email = form.email;
    person.interior.first_name = form.first_name;
    person.interior.last_name = form.last_name;

    person.store(db.acquire().await?).await?;

    let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

    let mut resp = Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(json!({"uid": person.exterior.uid.to_string(), "token": jwt.clone()}))
        .build();

    resp.insert_cookie(
        Cookie::build("token", jwt)
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Lax)
            .finish(),
    );

    Ok(resp)
}

/// Retrieve UI-approriate information about the current user in JSON
/// format. Includes the token, uid, username, and so on.
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
