use common::{
    model::{
        person::{LogEvent, LogIdentifier, Permission},
        Opportunity, Person,
    },
    Database,
};
use serde_json::json;
use tide::{Status, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use super::{okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(blank_opp)
        .post(add_opp)
        .at(":uid", |r| r.get(load_opp).put(save_opp))
}

pub async fn blank_opp(_: tide::Request<Database>) -> tide::Result {
    okay(&Opportunity::default())
}

pub async fn add_opp(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let mut opp: Opportunity = req.body_json().await?;

    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(req.state(), &opp).await?
    } else {
        false
    };

    if !authorized {
        return Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "permission denied",
        ));
    }

    opp.id = None;
    opp.exterior.uid = Uuid::nil();
    opp.exterior.slug = String::new();
    opp.interior.accepted = Some(true);

    opp.store(req.state()).await?;

    common::log(
        "add-opportunity",
        &json!({
            "who": person.as_ref().map(|p| p.exterior.uid),
            "opp": opp.exterior.uid}),
    );

    if let Some(person) = person {
        person
            .log(
                req.state(),
                LogEvent::EditOpportunity(LogIdentifier::Uid(opp.exterior.uid)),
            )
            .await?;
    }

    okay(&opp)
}

/// Unlike the entity::entity function, returns the entire
/// opportunity. This endpoint is for people who have authority over
/// the opportuity.
pub async fn load_opp(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let opp = Opportunity::load_by_uid(req.state(), &Uuid::parse_str(req.param("uid")?)?)
        .await
        .with_status(|| StatusCode::NotFound)?;

    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(req.state(), &opp).await?
    } else {
        false
    };

    if !authorized {
        return Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "permission denied",
        ));
    }

    okay(&opp)
}

pub async fn save_opp(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let original =
        Opportunity::load_by_uid(req.state(), &Uuid::parse_str(req.param("uid")?)?).await?;

    save_opportunity(person, original, req).await
}

/// Called by multiple endpoints to save an opportunity.
pub async fn save_opportunity(
    person: Option<Person>,
    original: Opportunity,
    mut req: tide::Request<Database>,
) -> tide::Result {
    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(req.state(), &original).await?
    } else {
        false
    };

    if !authorized {
        return Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "permission denied",
        ));
    }

    let mut opp: Opportunity = req.body_json().await?;

    opp.id = original.id;
    opp.exterior.uid = original.exterior.uid;
    opp.exterior.slug = original.exterior.slug;
    opp.exterior.partner = original.exterior.partner;
    opp.interior.accepted = match (original.interior.accepted, opp.interior.accepted) {
        (None, _) => None,
        (Some(false), _) => Some(false),
        (Some(true), None) => Some(true),
        (Some(true), Some(x)) => Some(x),
    };

    opp.store(req.state()).await?;

    common::log(
        "save-opportunity",
        &json!({
            "who": person.as_ref().map(|p| p.exterior.uid),
            "opp": opp.exterior.uid}),
    );

    if let Some(person) = person {
        person
            .log(
                req.state(),
                LogEvent::EditOpportunity(LogIdentifier::Uid(opp.exterior.uid)),
            )
            .await?;
    }

    okay(&opp)
}
