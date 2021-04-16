use crate::model::{self, partner::Partner};
use jwt::SignWithKey;
use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::http::StatusCode;
use tide::prelude::*;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;
use time::OffsetDateTime;
use uuid::Uuid;

use super::{error, success, JWT_SIGNING_KEY};

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes.at("authorize", |r| r.post(partner_authorize))
}

#[derive(Debug, Serialize, Deserialize)]
struct PartnerAuthorize {
    uid: Uuid,
    secret: String,
}

pub async fn partner_authorize(mut req: tide::Request<()>) -> tide::Result {
    let body: PartnerAuthorize = match req.body_json().await {
        Ok(data) => data,
        Err(x) => return Ok(error(StatusCode::BadRequest, x.to_string())),
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    let partner = match Partner::load_by_uid(db.acquire().await?, &body.uid).await {
        Ok(p) => p,
        Err(x) => {
            tide::log::warn!("Error loading partner for authorization: {:?}", x);
            return Ok(error(StatusCode::Forbidden, "Invalid uid or secret"));
        }
    };

    if let Some(hashed) = partner.interior.secret {
        if !djangohashers::check_password_tolerant(&body.secret, &hashed) {
            return Ok(error(StatusCode::Forbidden, "Invalid uid or secret"));
        }
    } else {
        return Ok(error(
            StatusCode::Forbidden,
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
