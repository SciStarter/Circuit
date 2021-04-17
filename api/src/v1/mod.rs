use crate::model;
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use jwt::VerifyWithKey;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sha2::Sha256;
use std::convert::TryInto;
use tide::http::{mime, StatusCode};
use tide::prelude::*;
use tide::Response;
use tide_fluent_routes::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

pub mod manage;
pub mod opportunity;
pub mod partner;

pub static JWT_SIGNING_KEY: Lazy<Hmac<Sha256>> =
    Lazy::new(|| Hmac::new_varkey(std::env::var("JWT_SIGNING_KEY").unwrap().as_bytes()).unwrap());

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("partner", partner::routes)
        .at("opportunity", opportunity::routes)
        .at("manage", manage::routes)
}

pub fn success<J>(json: &J) -> tide::Result
where
    J: serde::Serialize + ?Sized,
{
    Ok(Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(serde_json::to_string(json)?)
        .build())
}

pub fn error<S, M>(code: S, msg: M) -> Response
where
    S: TryInto<StatusCode>,
    S::Error: std::fmt::Debug,
    M: AsRef<str>,
{
    Response::builder(code)
        .content_type(mime::JSON)
        .body(json!({ "error": msg.as_ref() }))
        .build()
}

pub fn redirect(dest: &str) -> Response {
    Response::builder(StatusCode::SeeOther)
        .header("Location", dest)
        .build()
}

pub fn set_csrf_cookie(mut resp: Response, csrf: &str) -> Response {
    resp.insert_cookie(
        tide::http::Cookie::build("csrftoken", csrf.to_string())
            .path("/")
            .secure(true)
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Strict)
            .finish(),
    );

    resp
}

pub fn check_csrf(req: &tide::Request<()>, csrf: &str) -> bool {
    if let Some(cookie) = req.cookie("csrftoken") {
        csrf == cookie.value()
    } else {
        false
    }
}

pub fn random_string() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

pub fn issue_jwt(uid: &Uuid) -> Result<String, jwt::Error> {
    let now = (OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch()).as_seconds_f64()
        as jwt::claims::SecondsSinceEpoch;

    let mut claims = jwt::RegisteredClaims::default();
    claims.subject = Some(uid.to_string());
    claims.audience = Some(model::ROOT_NAMESPACE.to_string());
    claims.issuer = Some(model::ROOT_NAMESPACE.to_string());
    claims.issued_at = Some(now);
    claims.expiration = Some(now + (6 * 60 * 60));

    claims.sign_with_key(&*JWT_SIGNING_KEY)
}

pub fn check_jwt(token: &str) -> Result<Option<Uuid>, Response> {
    let claims: jwt::RegisteredClaims = token
        .verify_with_key(&*JWT_SIGNING_KEY)
        .map_err(|_| error(StatusCode::Forbidden, "Invalid authorization token"))?;

    let now = (OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch()).as_seconds_f64()
        as jwt::claims::SecondsSinceEpoch;

    if claims.expiration.unwrap_or(u64::MAX) >= now
        || claims.audience != Some(model::ROOT_NAMESPACE.to_string())
        || claims.issuer != Some(model::ROOT_NAMESPACE.to_string())
    {
        return Err(error(StatusCode::Forbidden, "Invalid authorization token"));
    }

    claims
        .subject
        .map(|s| -> Result<Uuid, Response> {
            Ok(Uuid::parse_str(&s)
                .map_err(|_| error(StatusCode::Forbidden, "Invalid authorization token"))?)
        })
        .transpose()
}

pub fn header_check(req: &tide::Request<()>) -> Result<Option<Uuid>, Response> {
    if let Some(ct) = req.content_type() {
        if ct != mime::JSON {
            return Err(error(
                StatusCode::BadRequest,
                "Content-Type header must specify application/json",
            ));
        }
    } else {
        return Err(error(
            StatusCode::BadRequest,
            "Content-Type header is required",
        ));
    }

    if let Some(header) = req.header("Authorization") {
        let mut parts = header.last().as_str().split_ascii_whitespace();

        match parts.next() {
            Some("Bearer") => {}
            Some(_) => {
                return Err(error(
                        StatusCode::Unauthorized,
                        "The Authorization header must contain the string 'Bearer ' followed by a partner authorization token",
                    ));
            }
            None => {
                return Err(error(
                        StatusCode::Unauthorized,
                        "The Authorization header must not be empty. It should contain the string 'Bearer ' followed by a partner authorization token",
                    ));
            }
        }

        if let Some(token) = parts.next() {
            return check_jwt(&token);
        } else {
            return Err(error(
                StatusCode::Unauthorized,
                "The Authorization header must contain a partner authorization token",
            ));
        }
    }

    Ok(None)
}
