use common::model::Partner;
use common::Database;
use tide::http::{mime, StatusCode};
use tide::prelude::*;
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

use super::{error, issue_jwt, success};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at("authorize", |r| r.post(partner_authorize))
}

#[derive(Debug, Serialize, Deserialize)]
struct PartnerAuthorize {
    uid: Uuid,
    secret: String,
}

pub async fn partner_authorize(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(ct) = req.content_type() {
        if ct != mime::JSON {
            return Ok(error(
                StatusCode::BadRequest,
                "Content-Type header must specify application/json",
            ));
        }
    } else {
        return Ok(error(
            StatusCode::BadRequest,
            "Content-Type header is required",
        ));
    }

    let body: PartnerAuthorize = match req.body_json().await {
        Ok(data) => data,
        Err(x) => return Ok(error(StatusCode::BadRequest, x.to_string())),
    };

    let db = req.state();

    let partner = match Partner::load_by_uid(db, &body.uid).await {
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

    let token = issue_jwt(&partner.exterior.uid, &super::API_AUDIENCE, 6)?;

    success(&json!({ "token": token }))
}
