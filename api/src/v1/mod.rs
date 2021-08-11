use common::Database;
use once_cell::sync::Lazy;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::convert::TryInto;
use tide::http::{mime, StatusCode};
use tide::prelude::*;
use tide::Response;
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

use common::jwt::{check_jwt, issue_jwt};

pub mod manage;
pub mod opportunity;
pub mod participation;
pub mod partner;

pub static API_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("023fda90-f6be-43ff-9b92-fa6ac89b2023").unwrap());

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("partner/", partner::routes)
        .at("opportunity/", opportunity::routes)
        .at("participation/", participation::routes)
        .at("manage/", manage::routes)
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
            .secure(cfg!(not(debug_assertions)))
            .http_only(true)
            .same_site(tide::http::cookies::SameSite::Strict)
            .finish(),
    );

    resp
}

pub fn check_csrf(req: &tide::Request<Database>, csrf: &str) -> bool {
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

pub fn header_check(req: &tide::Request<Database>, aud: &Uuid) -> Result<Option<Uuid>, Response> {
    if req.method() != tide::http::Method::Get {
        if let Some(ct) = req.content_type() {
            if ct != mime::JSON {
                return Err(error(
                    StatusCode::BadRequest,
                    "Content-Type must be application/json",
                ));
            }
        } else {
            return Err(error(
                StatusCode::BadRequest,
                "Content-Type header is required",
            ));
        }
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
            return Ok(Some(check_jwt(&token, aud).map_err(|_e| {
                error(
                    StatusCode::Unauthorized,
                    "The Authorization header must contain a partner authorization token",
                )
            })?));
        } else {
            return Err(error(
                StatusCode::Unauthorized,
                "The Authorization header must contain a partner authorization token",
            ));
        }
    }

    Ok(None)
}
