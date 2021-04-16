use crate::model;
use hmac::{Hmac, NewMac};
use jwt::VerifyWithKey;
use once_cell::sync::Lazy;
use sha2::Sha256;
use std::convert::TryInto;
use tide::http::{mime, StatusCode};
use tide::prelude::*;
use tide::Response;
use tide_fluent_routes::prelude::*;
use time::OffsetDateTime;
use uuid::Uuid;

pub mod opportunity;
pub mod partner;

pub static JWT_SIGNING_KEY: Lazy<Hmac<Sha256>> =
    Lazy::new(|| Hmac::new_varkey(std::env::var("JWT_SIGNING_KEY").unwrap().as_bytes()).unwrap());

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("partner", partner::routes)
        .at("opportunity", opportunity::routes)
}

fn success<J>(json: &J) -> tide::Result
where
    J: serde::Serialize + ?Sized,
{
    Ok(Response::builder(StatusCode::Ok)
        .content_type(mime::JSON)
        .body(serde_json::to_string(json)?)
        .build())
}

fn error<S, M>(code: S, msg: M) -> Response
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

fn header_check(req: &tide::Request<()>) -> Result<Option<Uuid>, Response> {
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

            return Ok(claims
                .subject
                .map(|s| -> Result<Uuid, Response> {
                    Ok(Uuid::parse_str(&s)
                        .map_err(|_| error(StatusCode::Forbidden, "Invalid authorization token"))?)
                })
                .transpose()?);
        } else {
            return Err(error(
                StatusCode::Unauthorized,
                "The Authorization header must contain a partner authorization token",
            ));
        }
    }

    Ok(None)
}
