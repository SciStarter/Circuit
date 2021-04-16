use model::{partner::Partner, person::Person};
use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::http::{mime, StatusCode};
use tide::prelude::*;
use tide::Response;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;
//use tide_websockets::{Message, WebSocket};
use crate::model;
use hmac::{Hmac, NewMac};
use jwt::{SignWithKey, VerifyWithKey};
use once_cell::sync::Lazy;
use sha2::Sha256;
use std::convert::TryInto;
use time::OffsetDateTime;
use uuid::Uuid;

pub static JWT_SIGNING_KEY: Lazy<Hmac<Sha256>> =
    Lazy::new(|| Hmac::new_varkey(std::env::var("JWT_SIGNING_KEY").unwrap().as_bytes()).unwrap());

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("authorization", |r| {
            r.at("partner", |r| r.post(partner_authorize))
        })
        .at("opportunity", |r| {
            r.at(":uid", |r| {
                r.get(opportunity_get)
                    .put(opportunity_put)
                    .delete(opportunity_delete)
            })
            .post(opportunity_new)
        })
}

#[derive(Debug, Serialize, Deserialize)]
struct PartnerAuthorize {
    uid: Uuid,
    secret: String,
}

async fn partner_authorize(mut req: tide::Request<()>) -> tide::Result {
    let body: PartnerAuthorize = match req.body_json().await {
        Ok(data) => data,
        Err(x) => return Ok(error(StatusCode::BadRequest, x.to_string())),
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    let partner = match Partner::load_by_uid(db.acquire().await?, &body.uid).await {
        Ok(p) => p,
        Err(x) => {
            tide::log::warn!("Error loading partner for authorization: {:?}", x);
            return Ok(error(StatusCode::BadRequest, "Invalid uid or secret"));
        }
    };

    if let Some(hashed) = partner.interior.secret {
        if !djangohashers::check_password_tolerant(&body.secret, &hashed) {
            return Ok(error(StatusCode::BadRequest, "Invalid uid or secret"));
        }
    } else {
        return Ok(error(
            StatusCode::BadRequest,
            "Authentication is not enabled for that partner",
        ));
    }

    let now = (OffsetDateTime::now_utc() - OffsetDateTime::unix_epoch()).as_seconds_f64()
        as jwt::claims::SecondsSinceEpoch;

    let mut claims = jwt::RegisteredClaims::default();
    claims.subject = Some(partner.exterior.uid.to_string());
    claims.audience = Some(model::ROOT_NAMESPACE.to_string());
    claims.issuer = Some(model::ROOT_NAMESPACE.to_string());
    claims.issued_at = Some(now);
    claims.expiration = Some(now + (6 * 60 * 60));

    let token = claims.sign_with_key(&*JWT_SIGNING_KEY)?;

    success(&json!({ "token": token }))
}

async fn opportunity_new(mut req: tide::Request<()>) -> tide::Result {
    let authenticated = match header_check(&req) {
        Ok(x) => match x {
            Some(uid) => uid,
            None => return Ok(error(StatusCode::Unauthorized, "Authorization is required")),
        },
        Err(res) => return Ok(res),
    };

    let mut opp: model::Opportunity = req.body_json().await?;

    if let Err(err) = opp.validate() {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    let mut db = req.sqlx_conn::<Postgres>().await;

    if model::Opportunity::exists_by_uid(db.acquire().await?, &opp.exterior.uid).await? {
        return Ok(error(
            StatusCode::Conflict,
            "An opportunity with that uid (or partner_name and title) already exists.",
        ));
    }

    opp.store(db.acquire().await?).await?;

    let res = Response::builder(StatusCode::Created)
        .content_type(mime::JSON)
        .body(serde_json::to_value(opp)?)
        .build();

    Ok(res)
}

async fn opportunity_get(req: tide::Request<()>) -> tide::Result {
    let uid: Uuid = match req.param("uid")?.parse() {
        Ok(uid) => uid,
        Err(_) => {
            return Ok(error(
                StatusCode::BadRequest,
                "Unable to parse a UUID from the request path",
            ));
        }
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    println!(
        "{:?}",
        model::Opportunity::load_by_id(db.acquire().await?, 1).await
    );

    Ok("Hello web world".into())
}

async fn opportunity_put(req: tide::Request<()>) -> tide::Result {
    let uid: Uuid = match req.param("uid")?.parse() {
        Ok(uid) => uid,
        Err(_) => {
            return Ok(error(
                StatusCode::BadRequest,
                "Unable to parse a UUID from the request path",
            ));
        }
    };

    // Always set interior.withdrawn to true

    let mut db = req.sqlx_conn::<Postgres>().await;

    println!(
        "{:?}",
        model::Opportunity::load_by_id(db.acquire().await?, 1).await
    );

    Ok("Hello web world".into())
}

async fn opportunity_delete(req: tide::Request<()>) -> tide::Result {
    let uid: Uuid = match req.param("uid")?.parse() {
        Ok(uid) => uid,
        Err(_) => {
            return Ok(error(
                StatusCode::BadRequest,
                "Unable to parse a UUID from the request path",
            ));
        }
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    println!(
        "{:?}",
        model::Opportunity::load_by_id(db.acquire().await?, 1).await
    );
    Ok("Hello web world".into())
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
