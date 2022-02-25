use chrono::Utc;
use common::{
    model::{
        involvement::{Involvement, Mode},
        opportunity::{Opportunity, OpportunityQuery, OpportunityQueryOrdering},
        person::{LogEvent, LogIdentifier, Permission},
        Pagination,
    },
    Database, ToFixedOffset,
};
use serde::Deserialize;
use serde_json::json;
use tide::{Status, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::ui::okay_empty;

use super::{okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.post(new_entity).at(":slug", |r| {
        r.get(entity)
            .put(save_entity)
            .at("me", |r| r.get(get_me))
            .at("interest", |r| r.post(register_interest))
            .at("likes", |r| {
                r.get(get_likes).post(add_like).delete(remove_like)
            })
            .at("saves", |r| {
                r.get(get_saves).post(add_save).delete(remove_save)
            })
            .at("didit", |r| {
                r.get(get_didit).post(add_didit).delete(remove_didit)
            })
            .at("reviews", |r| r.post(add_review).get(reviews))
            .at("report-review", |r| r.post(report_review))
            .at("recommended", |r| r.get(recommended))
            .at("managers", |r| r.post(request_page_management))
    })
}

pub async fn entity(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let slug = req.param("slug")?;
    let db = req.state();

    let opp = Opportunity::load_by_slug(db, slug)
        .await
        .with_status(|| StatusCode::NotFound)?;

    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(db, &opp).await?
    } else {
        false
    };

    if authorized || (opp.interior.accepted.unwrap_or(false) && !opp.interior.withdrawn) {
        common::log(
            "viewed",
            &json!({
                "who": person.as_ref().map(|p| p.exterior.uid),
                "opp": opp.exterior.uid}),
        );
        if let Some(person) = person {
            person
                .log(db, LogEvent::View(LogIdentifier::Uid(opp.exterior.uid)))
                .await?;
        }
        okay(&opp.into_annotated_exterior(authorized))
    } else {
        Err(tide::Error::from_str(StatusCode::NotFound, "not found"))
    }
}

pub async fn new_entity(mut req: tide::Request<Database>) -> tide::Result {
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
            "permissin denied",
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

pub async fn save_entity(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let original = Opportunity::load_by_slug(req.state(), req.param("slug")?).await?;

    let authorized = if let Some(p) = person.as_ref() {
        p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(req.state(), &original).await?
    } else {
        false
    };

    if !authorized {
        return Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "permissin denied",
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

pub async fn get_didit(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::for_slug::didits_for_slug(db, &slug).await?)
}

pub async fn add_didit(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;
    let db = req.state();

    common::model::opportunity::for_slug::add_didit_for_slug(db, &slug, &person.exterior.uid)
        .await?;

    common::log(
        "ui-didit",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::AddDidit(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}

pub async fn remove_didit(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;
    let db = req.state();

    common::model::opportunity::for_slug::remove_didit_for_slug(db, &slug, &person.exterior.uid)
        .await?;

    common::log(
        "ui-remove-didit",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::DeleteDidit(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}

pub async fn get_saves(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();

    okay(&common::model::opportunity::for_slug::saves_for_slug(db, &slug).await?)
}

pub async fn add_save(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;
    let db = req.state();

    common::model::opportunity::for_slug::add_save_for_slug(db, &slug, &person.exterior.uid)
        .await?;

    common::log(
        "ui-save",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::AddSave(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}

pub async fn remove_save(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;
    let db = req.state();

    common::model::opportunity::for_slug::remove_save_for_slug(db, &slug, &person.exterior.uid)
        .await?;

    common::log(
        "ui-remove-save",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::DeleteSave(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}

pub async fn get_likes(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::for_slug::likes_for_slug(db, &slug).await?)
}

pub async fn add_like(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req).await?;
    let db = req.state();

    common::model::opportunity::for_slug::add_like_for_slug(
        db,
        &slug,
        &person.as_ref().map(|p| p.exterior.uid),
    )
    .await?;

    common::log(
        "ui-like",
        &json!({"person": person.as_ref().map(|p| p.exterior.uid), "opportunity": slug}),
    );
    if let Some(person) = person {
        person
            .log(db, LogEvent::AddLike(LogIdentifier::Slug(slug)))
            .await?;
    }

    okay_empty()
}

pub async fn remove_like(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = match request_person(&mut req).await? {
        Some(person) => person,
        None => return okay_empty(),
    };
    let db = req.state();

    common::model::opportunity::for_slug::remove_like_for_slug(db, &slug, &person.exterior.uid)
        .await?;

    common::log(
        "ui-remove-like",
        &json!({"person": &person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::DeleteLike(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}

#[derive(Deserialize)]
struct AddReview {
    rating: i16,
    comment: String,
}

pub async fn add_review(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let review: AddReview = req.body_json().await?;

    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;

    let db = req.state();

    let id = common::model::opportunity::for_slug::add_review_for_slug(
        db,
        &slug,
        &person.exterior.uid,
        review.rating,
        &review.comment,
    )
    .await?;

    common::log(
        "ui-review",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::AddReview(LogIdentifier::Slug(slug)))
        .await?;

    okay(&json!({ "id": id }))
}

#[derive(Deserialize)]
struct ReviewReport {
    id: i32,
}

pub async fn report_review(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(person) = request_person(&mut req).await? {
        let report: ReviewReport = req.body_json().await?;
        let db = req.state();
        common::model::opportunity::for_slug::report_review(db, report.id).await?;
        common::log(
            "ui-report-review",
            &json!({"person": person.exterior.uid, "review": report.id}),
        );
        person
            .log(db, LogEvent::ReportReview(LogIdentifier::Id(report.id)))
            .await?;
    }
    okay_empty()
}

pub async fn reviews(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::for_slug::reviews_for_slug(db, &slug).await?)
}

pub async fn recommended(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();

    let opp = Opportunity::load_by_slug(db, slug)
        .await
        .with_status(|| StatusCode::NotFound)?;

    let point = if let Some(json) = opp.exterior.location_point {
        let lon: Option<f64> = json["coordinates"][0].as_f64();
        let lat: Option<f64> = json["coordinates"][1].as_f64();

        if let (Some(lon), Some(lat)) = (lon, lat) {
            Some((lon as f32, lat as f32, 32186.9)) // 32186.9 is 20 miles in meters
        } else {
            None
        }
    } else {
        None
    };

    let ordering;
    let pagination = Pagination::Page { index: 0, size: 5 };
    let mut query = OpportunityQuery::default();

    query.accepted = Some(true);
    query.withdrawn = Some(false);
    query.beginning = Some(Utc::now().to_fixed_offset());
    query.exclude = Some(vec![opp.exterior.uid.clone()]);

    if point.is_some() {
        query.near = point;
        ordering = OpportunityQueryOrdering::Closest;
    } else if !opp.exterior.opp_topics.is_empty() {
        // Look for opportunities with mostly the same topics.
        // "Mostly" here means that each topic of the current
        // opportunity after the first has a 25% chance to be dropped,
        // and then we look for opportunities with a superset of the
        // non-dropped topics.
        let mut has_one = false;
        query.topics = Some(
            opp.exterior
                .opp_topics
                .into_iter()
                .filter(move |_| {
                    if has_one {
                        rand::random::<f32>() > 0.25
                    } else {
                        has_one = true;
                        true
                    }
                })
                .collect(),
        );
        ordering = OpportunityQueryOrdering::Soonest;
    } else {
        // Sample all published upportunities, since this opportunity
        // doesn't seem have a location or topics. Random
        // opportunities are better than no opportunities.
        query.sample = Some(0.5);
        ordering = OpportunityQueryOrdering::Soonest;
    }

    let matches = Opportunity::load_matching(db, &query, ordering, pagination).await?;

    okay(&matches)
}

pub async fn request_page_management(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-request-page-management", "");
    todo!()
}

pub async fn get_me(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(person) = request_person(&mut req).await? {
        let slug = req.param("slug")?.to_string();
        let db = req.state();

        let opp = Opportunity::uid_by_slug(db, &slug)
            .await?
            .ok_or_else(|| tide::Error::from_str(404, "no such opportunity"))?;

        if let Some(involvement) =
            Involvement::load_by_participant_and_opportunity(db, &person.exterior.uid, &opp).await?
        {
            okay(&json!({
                "like": common::model::opportunity::for_slug::likes_for_slug_and_person(db, &slug, &person.exterior.uid).await? > 0,
                "save": involvement.exterior.mode >= Mode::Saved,
                "didit": involvement.exterior.mode >= Mode::Logged,
            }))
        } else {
            okay(&json!({
                "like": common::model::opportunity::for_slug::likes_for_slug_and_person(db, &slug, &person.exterior.uid).await? > 0,
                "save": false,
                "didit": false,
            }))
        }
    } else {
        okay(&json!({
            "like": false,
            "save": false,
            "didit": false,
        }))
    }
}

pub async fn register_interest(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = if let Some(person) = request_person(&mut req).await? {
        person
    } else {
        return okay_empty();
    };

    let db = req.state();

    common::model::opportunity::for_slug::add_interest_for_slug(db, &slug, &person.exterior.uid)
        .await?;

    common::log(
        "ui-interest",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );
    person
        .log(db, LogEvent::AddInterest(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}
