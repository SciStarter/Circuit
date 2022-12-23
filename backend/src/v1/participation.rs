use common::model::{opportunity::Opportunity, participation::Participation, person::Person};
use common::Database;
use tide::http::{mime, StatusCode};
use tide::Response;
use tide_fluent_routes::prelude::*;

use super::{error, header_check};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at(":hash", |r| r.post(participation_new))
}

async fn participation_new(mut req: tide::Request<Database>) -> tide::Result {
    let auth = match header_check(&req, &super::API_AUDIENCE) {
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

    let db = req.state();

    let opp = match Opportunity::load_by_uid(db, &part.exterior.opportunity).await {
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

    part.exterior.partner = auth;

    if let Ok(participant) = Person::load_by_email_hash(db, req.param("hash")?).await {
        part.interior.participant = Some(participant.exterior.uid);
    } else if let Ok(hash) = req.param("hash") {
        part.interior.snml = Some(String::from(hash));
    }

    if let Err(err) = part.validate() {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    if let Err(err) = part.store(db, true).await {
        return Ok(error(StatusCode::BadRequest, err.to_string()));
    }

    common::log(Some(&auth), "participation", &part);

    let res = Response::builder(StatusCode::Created)
        .content_type(mime::JSON)
        .body(serde_json::to_value(part.exterior)?)
        .build();

    Ok(res)
}
