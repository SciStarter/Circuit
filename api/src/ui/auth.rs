use common::{jwt::issue_jwt, model::Person, Database};
use http_types::Cookie;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::json;
use tide::{Response, StatusCode};
pub use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use time::Duration;

use crate::ui::{okay, okay_with_cookie};

use super::{error, person_json, request_person, UI_AUDIENCE};

pub static COOKIE_DOMAIN: Lazy<String> =
    Lazy::new(|| std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()));

pub const SESSION_HOURS: i64 = 24 * 90;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("login", |r| r.post(login))
        .at("login-scistarter", |r| r.post(login_scistarter))
        .at("signup", |r| r.post(signup))
        .at("me", |r| r.get(me))
        .at("logout", |r| r.post(logout))
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
            return error(400, "Login failed", &["email and password are required"]);
        }
    };

    let db = req.state();

    let person = match Person::load_by_email(db, &form.email).await {
        Ok(loaded) => loaded,
        Err(_) => {
            return error(403, "Login failed", &["email or password not recognized"]);
        }
    };

    if person.check_password(&form.password) {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        okay_with_cookie(
            "Logged in",
            &person_json(&person, &jwt),
            Cookie::build("token", jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                .domain(&*COOKIE_DOMAIN)
                .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                .http_only(true)
                .same_site(tide::http::cookies::SameSite::Lax)
                .finish(),
        )
    } else {
        error(403, "", &["email or password not recognized"])
    }
}

pub async fn login_scistarter(mut req: tide::Request<Database>) -> tide::Result {
    let _form: LoginForm = match req.body_json().await {
        Ok(parsed) => parsed,
        Err(_) => {
            return error(400, "Login failed", &["email and password are required"]);
        }
    };

    todo!()
}

#[derive(Default, Deserialize)]
struct SignupForm {
    email: String,
    username: Option<String>,
    password: String,
    zip_code: Option<String>,
    phone: Option<String>,
    _newsletter: Option<bool>,
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

    okay_with_cookie(
        "Your account has been created",
        &person_json(&person, &jwt),
        Cookie::build("token", jwt)
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            .domain(&*COOKIE_DOMAIN)
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Lax)
            .finish(),
    )
}

/// Retrieve UI-approriate information about the current user in JSON
/// format. Includes the token, uid, username, and so on.
pub async fn me(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(person) = request_person(&mut req).await? {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        okay_with_cookie(
            "",
            &person_json(&person, &jwt),
            Cookie::build("token", jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                .domain(&*COOKIE_DOMAIN)
                .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                .http_only(true)
                .same_site(tide::http::cookies::SameSite::Lax)
                .finish(),
        )
    } else {
        okay("", &json!({"authenticated": false}))
    }
}

pub async fn logout(mut req: tide::Request<Database>) -> tide::Result {
    okay_with_cookie(
        "",
        &json!({"authenticated": false}),
        Cookie::build("token", "")
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            .domain(&*COOKIE_DOMAIN)
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Lax)
            .finish(),
    )
}
