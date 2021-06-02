use common::model::{opportunity::Opportunity, participation::Participation, person::Person};
use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::http::{mime, StatusCode};
use tide::Response;
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;

use super::{error, header_check};

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes.at(":hash", |r| r.post(participation_new))
}

async fn participation_new(mut req: tide::Request<()>) -> tide::Result {
    let auth = match header_check(&req) {
        Ok(x) => match x {
            Some(auth) => auth,
            None => return Ok(error(StatusCode::Unauthorized, "Authorization is required")),
        },
        Err(res) => return Ok(res),
    };

    let mut part: Participation = match req.body_json().await {
        Ok(data) => data,
        Err(err) => {
            return Ok(error(StatusCode::BadRequest, err.to_string()));
        }
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    let participant =
        match Person::load_by_email_hash(db.acquire().await?, req.param("hash")?).await {
            Ok(p) => p,
            Err(_) => {
                return Ok(error(StatusCode::NotFound, req.param("hash")?));
            }
        };

    let opp = match Opportunity::load_by_uid(db.acquire().await?, &part.exterior.opportunity).await
    {
        Ok(o) => o,
        Err(_) => {
            return Ok(error(
                StatusCode::NotFound,
                part.exterior.opportunity.to_string(),
            ));
        }
    };

    if opp.exterior.partner != auth {
        return Ok(error(StatusCode::Forbidden, req.param("hash")?));
    }

    part.interior.participant = Some(participant.exterior.uid);
    part.exterior.partner = auth;

    if let Err(err) = part.validate() {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    if let Err(err) = part.store(db.acquire().await?, true).await {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    let res = Response::builder(StatusCode::Created)
        .content_type(mime::JSON)
        .body(serde_json::to_value(part.exterior)?)
        .build();

    Ok(res)
}
