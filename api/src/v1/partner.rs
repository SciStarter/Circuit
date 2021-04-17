use crate::model::Partner;
use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::http::StatusCode;
use tide::prelude::*;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;
use uuid::Uuid;

use super::{error, issue_jwt, success};

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

    if let Some(valid) = partner.check_secret_full(&body.secret) {
        if !valid {
            return Ok(error(StatusCode::Forbidden, "Invalid uid or secret"));
        }
    } else {
        return Ok(error(
            StatusCode::Forbidden,
            "Authentication is not enabled for that partner",
        ));
    }

    let token = issue_jwt(&partner.exterior.uid)?;

    success(&json!({ "token": token }))
}
