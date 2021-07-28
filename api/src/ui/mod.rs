use common::jwt::issue_jwt;
use common::model::person::Permission;
use common::model::{self, Opportunity, Person};
use common::Database;
use http_types::{mime, Cookie};
use once_cell::sync::Lazy;
use serde_json::json;
//use std::convert::TryInto;
//use tide::http::mime;
use serde::Deserialize;
use tide::http::StatusCode;
use tide::Response;
//use tide::{prelude::*, ResponseBuilder};
use tide_fluent_routes::prelude::*;
use time::Duration;
//use uuid::Uuid;

//use common::jwt::{check_jwt, issue_jwt};

pub static UI_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("0be35cad-2b1f-4a45-a6da-b1051643c6f6").unwrap());

pub static COOKIE_DOMAIN: Lazy<String> =
    Lazy::new(|| std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()));

pub const SESSION_HOURS: i64 = 24 * 90;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("click", |r| r.post(record_click))
        .at("external", |r| r.post(record_external))
        .at("login", |r| r.post(login))
        .at("signup", |r| r.post(signup))
        .at("me", |r| r.get(me))
        .at("content", |r| r.get(content))
        .at("entity/:slug", |r| r.get(entity))
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
                token
            } else {
                return Ok(None);
            }
        } else {
            return Ok(None);
        }
    } else {
        return Ok(None);
    };

    let uid = match common::jwt::check_jwt(token, &UI_AUDIENCE) {
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
pub async fn login(mut req: tide::Request<Database>) -> tide::Result {
    let form: LoginForm = match req.body_json().await {
        Ok(parsed) => parsed,
        Err(_) => {
            return Ok(Response::builder(StatusCode::BadRequest)
                .body("email and password are required")
                .build())
        }
    };

    let db = req.state();

    let person = match Person::load_by_email(db, &form.email).await {
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
            .body(person_json(&person, &jwt))
            .build();

        resp.insert_cookie(
            Cookie::build("token", jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                .domain(&*COOKIE_DOMAIN)
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
    username: Option<String>,
    password: String,
    zip_code: Option<String>,
    phone: Option<String>,
}

/// Create an account, if the validations pass.
///
/// Set a token cookie with the HttpOnly, SameSite=Lax, and (when
/// ```#[cfg(not(debug_assertions))]```) Secure flags, and also
/// return the token in the response body for script use.
pub async fn signup(mut req: tide::Request<Database>) -> tide::Result {
    let form: SignupForm = match req.body_json().await {
        Ok(parsed) => parsed,
        Err(_) => {
            return Ok(Response::builder(StatusCode::BadRequest)
                .body("email and password are required")
                .build())
        }
    };

    let db = req.state();

    if Person::exists_by_email(db, &form.email).await? {
        return Ok(Response::builder(StatusCode::Forbidden)
            .body("That email is already in use")
            .build());
    }

    let mut person = Person::default();
    person.set_password(&form.password);
    person.exterior.username = form.username;
    person.interior.email = form.email;
    person.interior.zip_code = form.zip_code;
    person.interior.phone = form.phone;

    person.store(db).await?;

    let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

    let mut resp = Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(person_json(&person, &jwt))
        .build();

    resp.insert_cookie(
        Cookie::build("token", jwt)
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            .domain(&*COOKIE_DOMAIN)
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Lax)
            .finish(),
    );

    Ok(resp)
}

/// Retrieve UI-approriate information about the current user in JSON
/// format. Includes the token, uid, username, and so on.
pub async fn me(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(person) = request_person(&mut req).await? {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        let mut resp = Response::builder(StatusCode::Ok)
            .content_type(mime::JSON)
            .body(person_json(&person, &jwt))
            .build();

        resp.insert_cookie(
            Cookie::build("token", jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                .domain(&*COOKIE_DOMAIN)
                .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                .http_only(true)
                .same_site(tide::http::cookies::SameSite::Lax)
                .finish(),
        );

        Ok(resp)
    } else {
        Ok(Response::builder(StatusCode::Ok)
            .content_type(mime::JSON)
            .body(json!({"authenticated": false}))
            .build())
    }
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
        Ok(Response::builder(StatusCode::Ok)
            .body(block.content)
            .build())
    } else {
        Ok(Response::builder(StatusCode::NotFound)
            .body(format!("{} {} {}", query.language, query.group, query.item))
            .build())
    }
}

pub async fn entity(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();

    let opp = Opportunity::load_by_slug(db, slug).await?;

    let person = request_person(&mut req).await?;

    if opp.interior.accepted
        || person
            .map(|p| p.check_permission(&Permission::ManageOpportunities))
            .unwrap_or(false)
    {
        Ok(Response::builder(StatusCode::Ok)
            .content_type("application/json")
            .body(serde_json::to_value(opp.exterior)?)
            .build())
    } else {
        Ok(Response::builder(StatusCode::NotFound).build())
    }
}
