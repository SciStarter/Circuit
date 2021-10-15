use common::{
    jwt::issue_jwt,
    model::{
        involvement::{self, Involvement},
        person::JoinChannel,
        Person,
    },
    Database,
};
use http_types::Cookie;
use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::json;
use tide::{prelude::*, Response, StatusCode};
pub use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use time::Duration;

use crate::crypto::{KeyPair, Sealed};
use crate::ui::{okay, okay_with_cookie};

use super::{person_json, request_person, UI_AUDIENCE};

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

#[cfg(not(debug_assertions))]
const TOKEN_COOKIE: &'static str = "__Host-token";
#[cfg(debug_assertions)]
const TOKEN_COOKIE: &'static str = "token";

#[derive(Default, Deserialize, Serialize)]
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
    let form: LoginForm = req.body_json().await.map_err(|mut e| {
        e.set_status(400);
        e
    })?;

    let db = req.state();

    let person = match Person::load_by_email(db, &form.email).await {
        Ok(loaded) => loaded,
        Err(_) => {
            return Err(tide::Error::from_str(
                403,
                "email or password not recognized",
            ));
        }
    };

    if person.check_password(&form.password) {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        let mut p_json = person_json(&person, &jwt);
        p_json["num_partners"] = person.count_partners(db).await?.into();

        common::log("ui-login", &jwt);

        okay_with_cookie(
            &p_json,
            Cookie::build(TOKEN_COOKIE, jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                //.domain(&*COOKIE_DOMAIN)
                .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                .http_only(true)
                .same_site(tide::http::cookies::SameSite::Lax)
                .finish(),
        )
    } else {
        Err(tide::Error::from_str(
            403,
            "email or password not recognized",
        ))
    }
}

#[derive(Deserialize, Debug)]
struct SciStarterPerson {
    username: Option<String>,
    first_name: String,
    last_name: String,
    emails: Vec<String>,
    zip_code: Option<String>,
    phone: Option<String>,
    newsletter: bool,
}

pub async fn login_scistarter(mut req: tide::Request<Database>) -> tide::Result {
    let form: LoginForm = req.body_json().await.map_err(|mut e| {
        e.set_status(StatusCode::BadRequest);
        e
    })?;

    let existing = Person::load_by_email(req.state(), &form.email).await.ok();

    if let Some(person) = &existing {
        if person.interior.join_channel != JoinChannel::SciStarter {
            return Ok(
                tide::Response::builder(StatusCode::Forbidden)
                    .body("Can not log in to that account through SciStarter, because it was not created via SciStarter")
                    .build()
            );
        }
    }

    let snm_key = KeyPair::from_env("SNM_PAIR")?;
    let scistarter_key = KeyPair::from_env("SCI_PUB")?;

    let sealed: Sealed = surf::post("https://scistarter.org/api/login-for-snm")
        .content_type("application/json")
        .body(serde_json::to_string(
            &snm_key.seal(&form, &scistarter_key)?,
        )?)
        .recv_json()
        .await?;

    dbg!(&sealed);

    match dbg!(snm_key.open::<SciStarterPerson>(sealed, Some(&scistarter_key))) {
        Ok(ssp) => {
            let person = if let Some(person) = existing {
                person
            } else {
                let mut person = Person::default();
                person.set_password(&form.password);
                person.exterior.username = ssp.username;
                person.interior.email = ssp
                    .emails
                    .first()
                    .ok_or_else(|| {
                        tide::Error::from_str(
                            StatusCode::BadRequest,
                            "The SciStarter validation process failed",
                        )
                    })?
                    .to_owned();
                person.interior.zip_code = ssp.zip_code;
                person.interior.phone = ssp.phone;
                person.interior.newsletter = ssp.newsletter;

                // Do we actually want to do this? It could be a way
                // to spoof the system.
                for email in ssp.emails {
                    person.add_hash(&email)?;
                }

                person.store(req.state()).await?;

                person
            };

            let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

            let mut p_json = person_json(&person, &jwt);
            p_json["num_partners"] = person.count_partners(req.state()).await?.into();

            common::log("ui-login-via-scistarter", &jwt);

            okay_with_cookie(
                &p_json,
                Cookie::build(TOKEN_COOKIE, jwt)
                    .path("/")
                    .max_age(Duration::hours(SESSION_HOURS))
                    //.domain(&*COOKIE_DOMAIN)
                    .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                    .http_only(true)
                    .same_site(tide::http::cookies::SameSite::Lax)
                    .finish(),
            )
        }
        Err(_) => {
            return Ok(tide::Response::builder(StatusCode::Forbidden)
                .body("Incorrect SciStarter email or password")
                .build());
        }
    }
}

#[derive(Default, Deserialize)]
struct SignupForm {
    email: String,
    username: Option<String>,
    password: String,
    zip_code: Option<String>,
    phone: Option<String>,
    newsletter: Option<bool>,
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
    person.interior.newsletter = form.newsletter.unwrap_or(false);
    person.store(db).await?;

    let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

    let mut p_json = person_json(&person, &jwt);
    p_json["num_partners"] = 0.into();

    common::log("ui-signup", &jwt);

    okay_with_cookie(
        &p_json,
        Cookie::build(TOKEN_COOKIE, jwt)
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            //.domain(&*COOKIE_DOMAIN)
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

        let mut p_json = person_json(&person, &jwt);
        p_json["num_partners"] = person.count_partners(req.state()).await?.into();
        p_json["reports_pending"] = Involvement::count_for_participant(
            req.state(),
            &person.exterior.uid,
            Some(involvement::Mode::Interest),
            Some(involvement::Mode::Saved),
        )
        .await?
        .into();

        okay_with_cookie(
            &p_json,
            Cookie::build(TOKEN_COOKIE, jwt)
                .path("/")
                .max_age(Duration::hours(SESSION_HOURS))
                //.domain(&*COOKIE_DOMAIN)
                .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                .http_only(true)
                .same_site(tide::http::cookies::SameSite::Lax)
                .finish(),
        )
    } else {
        okay(&json!({"authenticated": false}))
    }
}

pub async fn logout(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-logout", "");

    okay_with_cookie(
        &json!({"authenticated": false}),
        Cookie::build(TOKEN_COOKIE, "")
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            //.domain(&*COOKIE_DOMAIN)
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Lax)
            .finish(),
    )
}
