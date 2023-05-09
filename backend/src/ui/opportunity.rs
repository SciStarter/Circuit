use common::{
    model::{
        opportunity::ReviewStatus,
        person::{LogEvent, LogIdentifier, Permission, PermitAction},
        Opportunity, Partner, Person,
    },
    Database,
};
use once_cell::sync::Lazy;
use serde_json::json;
use tide::{Status, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use super::{okay, request_person};

static STAFF_REVIEWS: Lazy<[Uuid; 0]> = Lazy::new(|| {
    [
        // Alabama STEM Council
        //Uuid::parse_str("b9224b48-dcc3-5153-9c31-7b53ff24a380").unwrap(),
    ]
});

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(blank_opp)
        .post(add_opp)
        .at(":uid", |r| r.get(load_opp).put(save_opp))
}

pub async fn blank_opp(_: tide::Request<Database>) -> tide::Result {
    let mut opp = Opportunity::default();
    opp.exterior.max_age = 999;
    opp.interior.withdrawn = true;
    okay(&opp)
}

async fn notify_pending_approval(
    req: &tide::Request<Database>,
    partner: &Partner,
    opp: &Opportunity,
) -> Result<(), tide::Error> {
    let template = common::emails::EmailMessage::load_or_default(
            req.state(),
            "opportunity-pending-approval",
            "Pending Approval on Science Near Me: {title}",
            r#"
<p>The {org_name} opportunity <strong>{title}</strong> on {partner_name} / Science Near Me has been created or updated, and is pending approval for publication.</p>
<p>Please evaluate the opportunity and <a href="https://sciencenearme.org/exchange/{partner_uid}/{opp_slug}">approve, reject, or send it back to draft</a> it.</p>
"#,
        ).await;

    let msg = template.materialize(vec![
        ("title", &opp.exterior.title),
        ("partner_name", &partner.exterior.name),
        ("partner_uid", &partner.exterior.uid.to_string()),
        ("org_name", &opp.exterior.organization_name),
        ("opp_slug", &opp.exterior.slug),
    ]);

    if STAFF_REVIEWS.iter().any(|x| *x == partner.exterior.uid) {
        for reviewer in Person::all_by_permission(req.state(), &Permission::ManageOpportunities)
            .await?
            .into_iter()
        {
            if let Ok(person) = reviewer {
                common::emails::send_message(person.interior.email, &msg).await;
            }
        }
    } else {
        for reviewer in partner
            .load_authorized_persons(req.state())
            .await?
            .into_iter()
        {
            if let Ok(person) = reviewer {
                common::emails::send_message(person.interior.email, &msg).await;
            }
        }
    }

    Ok(())
}

pub async fn add_opp(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let mut opp: Opportunity = req.body_json().await?;

    let partner = Partner::load_by_uid(req.state(), &opp.exterior.partner).await?;

    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(req.state(), &opp, PermitAction::Add)
                .await?
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

    opp.interior.submitted_by = person.as_ref().map(|x| x.exterior.uid);
    opp.interior.review_status = if partner.exterior.open_submission.unwrap_or_default() {
        match opp.interior.submitted_by {
            Some(uid) if partner.person_has_permission(&uid) => ReviewStatus::NotRequired,
            _ => ReviewStatus::Draft,
        }
    } else {
        ReviewStatus::NotRequired
    };

    opp.store(req.state()).await?;

    common::log(
        person.as_ref().map(|p| &p.exterior.uid),
        "add-opportunity",
        &json!({
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

    if let ReviewStatus::Pending = opp.interior.review_status {
        notify_pending_approval(&req, &partner, &opp).await?;
    }

    okay(&opp)
}

/// Unlike the entity::entity function, returns the entire
/// opportunity. This endpoint is for people who have authority over
/// the opportunity.
pub async fn load_opp(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let opp = Opportunity::load_by_uid(req.state(), &Uuid::parse_str(req.param("uid")?)?)
        .await
        .with_status(|| StatusCode::NotFound)?;

    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(req.state(), &opp, PermitAction::Edit)
                .await?
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
            || p.check_authorization(req.state(), &original, PermitAction::Edit)
                .await?
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
    let partner = Partner::load_by_uid(req.state(), &opp.exterior.partner).await?;

    opp.id = original.id;
    opp.exterior.uid = original.exterior.uid;
    opp.exterior.slug = original.exterior.slug;
    opp.exterior.partner = original.exterior.partner;
    opp.interior.submitted_by = original.interior.submitted_by;

    opp.interior.review_status = if opp.interior.review_status.requires_manager() {
        if partner
            .person_has_permission(&person.as_ref().map(|x| x.exterior.uid).unwrap_or_default())
        {
            opp.interior.review_status
        } else {
            original.interior.review_status
        }
    } else {
        opp.interior.review_status
    };

    opp.interior.accepted = match (original.interior.accepted, opp.interior.accepted) {
        (None, _) => None,
        (Some(false), _) => Some(false),
        (Some(true), None) => Some(true),
        (Some(true), Some(x)) => Some(x),
    };

    opp.store(req.state()).await?;

    common::log(
        person.as_ref().map(|p| &p.exterior.uid),
        "save-opportunity",
        &json!({
            "opportunity": opp.exterior.uid}),
    );

    if let Some(person) = person {
        person
            .log(
                req.state(),
                LogEvent::EditOpportunity(LogIdentifier::Uid(opp.exterior.uid)),
            )
            .await?;
    }

    if let ReviewStatus::Draft = original.interior.review_status {
        if let ReviewStatus::Pending = opp.interior.review_status {
            notify_pending_approval(&req, &partner, &opp).await?;
        }
    }

    okay(&opp)
}
