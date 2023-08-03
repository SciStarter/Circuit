use common::{
    jwt::issue_jwt,
    model::{
        invitation::{Invitation, InvitationMode},
        involvement::{self, Involvement},
        person::{JoinChannel, LogEvent, Permission},
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
use uuid::Uuid;

use crate::crypto::{KeyPair, Sealed};
use crate::ui::{okay, okay_with_cookie};

use super::{person_json, request_person, UI_AUDIENCE};

pub static COOKIE_DOMAIN: Lazy<String> =
    Lazy::new(|| std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()));

pub const SESSION_HOURS: i64 = 24 * 90;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("reset", |r| r.post(reset))
        .at("login", |r| r.post(login))
        .at("login-scistarter", |r| r.post(login_scistarter))
        .at("signup", |r| r.post(signup))
        .at("me", |r| r.get(me))
        .at("logout", |r| r.post(logout))
}

#[cfg(not(debug_assertions))]
pub const TOKEN_COOKIE: &'static str = "__Host-token";
#[cfg(debug_assertions)]
pub const TOKEN_COOKIE: &'static str = "token";

#[derive(Deserialize)]
struct ResetForm {
    email: String,
}

pub async fn reset(mut req: tide::Request<Database>) -> tide::Result {
    let form: ResetForm = req.body_json().await?;

    let person = Person::load_by_email(req.state(), &form.email).await?;

    let mut inv = Invitation::new(person.exterior.uid, InvitationMode::PasswordReset);
    inv.store(req.state()).await?;

    let template = common::emails::EmailMessage::load_or_default(
        req.state(),
        "password-reset",
        "Science Near Me one-time login",
        r#"<p>This is to confirm your request for a one-time login on Science Near Me.</p>
<p>If you did not make such a request, you can safely ignore this email. Nothing will change and whoever made the request will not be given access.</p>
<p>To confirm the request and log in, just <a href="https://sciencenearme.org/api/ui/invitation/{invitation}">click here</a>. If you wish to change your Science Near Me password, you'll have that option after logging in.</p>
<p>Regards,
~the Science Near Me team</p>
"#,
    )
        .await;

    let msg = template.materialize(vec![("invitation", inv.uid())]);

    common::emails::send_message(form.email, &msg).await;

    Ok("Confirmation message sent.".into())
}

#[derive(Default, Deserialize, Serialize)]
struct LoginForm {
    email: String,
    password: String,
}

pub fn token_cookie<'c>(jwt: String) -> Cookie<'c> {
    Cookie::build(TOKEN_COOKIE, jwt)
        .path("/")
        .max_age(Duration::hours(SESSION_HOURS))
        //.domain(&*COOKIE_DOMAIN)
        .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
        .http_only(true)
        .same_site(if cfg!(not(debug_assertions)) {
            tide::http::cookies::SameSite::None
        } else {
            tide::http::cookies::SameSite::Lax
        })
        .finish()
}

/// Log the current session in to an account, if the validations pass.
///
/// Set a token cookie with the HttpOnly, SameSite=None, and (when
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
                r#"Error: Wrong username or password. <a href="/login">Forgot password?</a>"#,
            ));
        }
    };

    if person.check_password(&form.password) {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        let mut p_json = person_json(&person, &jwt);
        p_json["num_partners"] = person.count_partners(db).await?.into();

        common::log(Some(&person.exterior.uid), "ui-login", &jwt);
        person.log(db, LogEvent::Login).await?;

        okay_with_cookie(&p_json, token_cookie(jwt))
    } else {
        Err(tide::Error::from_str(
            403,
            r#"Error: Wrong username or password. <a href="/login">Forgot password?</a>"#,
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
                    .body(r#"Oops! Looks like you already have a Science Near Me account with that email address. Please log in with your Science Near Me credentials <a href="/login">here</a>."#)
                    .build()
            );
        }
    }

    let snm_key = KeyPair::from_env("SNM_PAIR")?;
    let scistarter_key = KeyPair::from_env("SCI_PUB")?;

    let response = surf::post("https://scistarter.org/api/login-for-snm")
        .content_type("application/json")
        .body(serde_json::to_string(
            &snm_key.seal(&form, &scistarter_key)?,
        )?)
        .recv_json()
        .await;

    let sealed: Sealed = match response {
        Ok(sealed) => sealed,
        Err(_) => {
            return Ok(tide::Response::builder(StatusCode::Forbidden)
                .body("Incorrect SciStarter email or password")
                .build());
        }
    };

    match snm_key.open::<SciStarterPerson>(sealed, Some(&scistarter_key)) {
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
                person.interior.first_name = Some(ssp.first_name);
                person.interior.last_name = Some(ssp.last_name);
                person.interior.zip_code = ssp.zip_code;
                person.interior.phone = ssp.phone;
                person.interior.newsletter = ssp.newsletter;
                person.interior.join_channel = JoinChannel::SciStarter;

                // Do we actually want to do this? It could be a way
                // to spoof the system.
                for email in ssp.emails {
                    person.add_hash(&email)?;
                }

                person.store(req.state()).await?;

                let message = common::emails::EmailMessage::load(req.state(), "welcome-new-user")
                    .await
                    .ok();

                if let Some(message) = message {
                    common::emails::send_message(&person.interior.email, &message).await;
                }

                person
            };

            let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

            let mut p_json = person_json(&person, &jwt);
            p_json["num_partners"] = person.count_partners(req.state()).await?.into();

            common::log(Some(&person.exterior.uid), "ui-login-via-scistarter", &jwt);
            person.log(req.state(), LogEvent::Login).await?;

            okay_with_cookie(
                &p_json,
                Cookie::build(TOKEN_COOKIE, jwt)
                    .path("/")
                    .max_age(Duration::hours(SESSION_HOURS))
                    //.domain(&*COOKIE_DOMAIN)
                    .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
                    .http_only(true)
                    .same_site(if cfg!(not(debug_assertions)) {
                        tide::http::cookies::SameSite::None
                    } else {
                        tide::http::cookies::SameSite::Lax
                    })
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

#[derive(Default, Debug, Deserialize)]
struct SignupForm {
    email: String,
    username: Option<String>,
    password: String,
    zip_code: Option<String>,
    phone: Option<String>,
    newsletter: Option<bool>,
    exchange: Option<Uuid>,
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
                .body("Email and password are required")
                .build())
        }
    };

    let db = req.state();

    if Person::exists_by_email(db, &form.email).await? {
        let other = Person::load_by_email(db, &form.email).await?;

        return Ok(Response::builder(StatusCode::Forbidden)
            .body(if other.interior.join_channel == JoinChannel::SciStarter {
                r#"Oops! Looks like you already have a Science Near Me account with that email address, created by logging in with your SciStarter credentials. Please try logging in using the SciStarter password you imported to Science Near Me."#
            } else {
                r#"That email address is already associated with an account. Please try logging in instead."#
            })
            .build());
    }

    let mut person = Person::default();
    person.set_password(&form.password);
    person.exterior.username = form.username;
    person.interior.email = form.email;
    person.interior.zip_code = form.zip_code;
    person.interior.phone = form.phone;
    person.interior.newsletter = form.newsletter.unwrap_or(false);

    if let Some(uid) = form.exchange {
        person.interior.join_channel = JoinChannel::Exchange(uid);
    }

    person.store(db).await?;

    let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

    let mut p_json = person_json(&person, &jwt);
    p_json["num_partners"] = 0.into();

    common::log(Some(&person.exterior.uid), "ui-signup", &jwt);
    person.log(db, LogEvent::Signup).await?;

    let message = common::emails::EmailMessage::load(db, "welcome-new-user")
        .await
        .ok();

    if let Some(message) = message {
        common::emails::send(
            &person.interior.email,
            "Science Near Me <info@sciencenearme.org>",
            message.subject,
            message.body,
        )
        .await;
    }

    okay_with_cookie(
        &p_json,
        Cookie::build(TOKEN_COOKIE, jwt)
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            //.domain(&*COOKIE_DOMAIN)
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(if cfg!(not(debug_assertions)) {
                tide::http::cookies::SameSite::None
            } else {
                tide::http::cookies::SameSite::Lax
            })
            .finish(),
    )
}

/// Retrieve UI-approriate information about the current user in JSON
/// format. Includes the token, uid, username, and so on.
pub async fn me(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(person) = request_person(&mut req).await? {
        let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;

        person.log(req.state(), LogEvent::Session).await?;

        let mut p_json = person_json(&person, &jwt);
        p_json["num_partners"] = person
            .count_partners(req.state())
            .await?
            .max(
                if person.check_permission(&Permission::ManageOpportunities) {
                    1
                } else {
                    0
                },
            )
            .into();
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
                .same_site(if cfg!(not(debug_assertions)) {
                    tide::http::cookies::SameSite::None
                } else {
                    tide::http::cookies::SameSite::Lax
                })
                .finish(),
        )
    } else {
        okay(&json!({"authenticated": false}))
    }
}

pub async fn logout(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    common::log(person.as_ref().map(|p| &p.exterior.uid), "ui-logout", "");

    okay_with_cookie(
        &json!({"authenticated": false}),
        Cookie::build(TOKEN_COOKIE, "")
            .path("/")
            .max_age(Duration::hours(SESSION_HOURS))
            //.domain(&*COOKIE_DOMAIN)
            .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
            .http_only(true)
            .same_site(if cfg!(not(debug_assertions)) {
                tide::http::cookies::SameSite::None
            } else {
                tide::http::cookies::SameSite::Lax
            })
            .finish(),
    )
}
