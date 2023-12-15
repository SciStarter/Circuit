use chrono::Utc;
use common::{
    model::{
        involvement::{Involvement, Mode},
        opportunity::{
            Opportunity, OpportunityAll, OpportunityQuery, OpportunityQueryOrdering,
            OpportunityReference, ReviewStatus,
        },
        person::{LogEvent, LogIdentifier, Permission, PermitAction},
        Pagination, Partner, Person,
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

use crate::ui::okay_empty;

use super::{okay, opportunity, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at(":slug", |r| {
        r.get(entity)
            .put(save_entity)
            .at("me", |r| r.get(get_me))
            .at("shared", |r| r.post(shared))
            .at("calendar", |r| r.post(calendar))
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
            .at("status", |r| r.put(set_review_status))
    })
}

pub async fn entity(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let slug = req.param("slug")?;
    let db = req.state();

    let opp = Opportunity::load_by_slug_with_overlay(db, slug)
        .await
        .with_status(|| StatusCode::NotFound)?;

    let authorized = if let Some(p) = person.as_ref() {
        if p.check_permission(&Permission::ManageOpportunities)
            || p.check_authorization(db, &opp, PermitAction::Manage)
                .await?
        {
            PermitAction::Manage
        } else if p.check_authorization(db, &opp, PermitAction::Edit).await? {
            PermitAction::Edit
        } else {
            PermitAction::Nothing
        }
    } else {
        PermitAction::Nothing
    };

    if authorized != PermitAction::Nothing || opp.is_published(db).await? {
        common::log(
            person.as_ref().map(|p| &p.exterior.uid),
            "viewed",
            &json!({
                "opportunity": opp.uid}),
        );
        if let Some(person) = person {
            person
                .log(db, LogEvent::View(LogIdentifier::Uid(opp.uid)))
                .await?;
        } else if let Some(anonymous) = common::model::person::ANONYMOUS.get() {
            anonymous
                .log(db, LogEvent::View(LogIdentifier::Uid(opp.uid)))
                .await?;
        }

        okay(&opp.into_annotated_exterior(db, authorized).await?)
    } else {
        Err(tide::Error::from_str(StatusCode::NotFound, "not found"))
    }
}

pub async fn save_entity(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;

    let original = OpportunityAll::load_by_slug(req.state(), req.param("slug")?).await?;

    opportunity::save_opportunity(person, original, req).await
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
        Some(&person.exterior.uid),
        "ui-didit",
        &json!({ "opportunity": slug }),
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
        Some(&person.exterior.uid),
        "ui-remove-didit",
        &json!({ "opportunity": slug }),
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
        Some(&person.exterior.uid),
        "ui-save",
        &json!({ "opportunity": slug }),
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
        Some(&person.exterior.uid),
        "ui-remove-save",
        &json!({ "opportunity": slug }),
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
        person.as_ref().map(|p| &p.exterior.uid),
        "ui-like",
        &json!({ "opportunity": slug }),
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
        Some(&person.exterior.uid),
        "ui-remove-like",
        &json!({ "opportunity": slug }),
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
        Some(&person.exterior.uid),
        "ui-review",
        &json!({ "opportunity": slug }),
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
            Some(&person.exterior.uid),
            "ui-report-review",
            &json!({"review": report.id}),
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

    let point = if let Some(p) = opp.location_point.0 {
        Some((p.x(), p.y(), 32186.9)) // 32186.9 is 20 miles in meters
    } else {
        None
    };

    let ordering;
    let pagination = Pagination::Page { index: 0, size: 5 };
    let mut query = OpportunityQuery::default();

    query.accepted = Some(true);
    query.withdrawn = Some(false);
    query.beginning = Some(Utc::now().to_fixed_offset());
    query.exclude = Some(vec![opp.uid]);

    if let Some((lon, lat, dist)) = point {
        query.near_longitude = Some(lon);
        query.near_latitude = Some(lat);
        query.near_distance = Some(dist);
        ordering = OpportunityQueryOrdering::Closest;
    } else {
        let topics = opp.topics(db).await?;

        if !topics.is_empty() {
            // Look for opportunities with mostly the same topics.
            // "Mostly" here means that each topic of the current
            // opportunity after the first has a 25% chance to be dropped,
            // and then we look for opportunities with a superset of the
            // non-dropped topics.
            let mut has_one = false;
            query.topics = Some(
                topics
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
    }

    let matches = Opportunity::load_matching_refs(db, query, ordering, pagination).await?;

    okay(&matches)
}

pub async fn request_page_management(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?;
    common::log(
        person.as_ref().map(|p| &p.exterior.uid),
        "ui-request-page-management",
        "",
    );
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

#[derive(Deserialize)]
struct SharedForm {
    network: String,
}

pub async fn shared(mut req: tide::Request<Database>) -> tide::Result {
    let opp = OpportunityReference::load_by_slug(req.state(), req.param("slug")?).await?;
    let person = request_person(&mut req).await?;
    let form: SharedForm = req.body_json().await?;

    sqlx::query!(
        r#"INSERT INTO c_log ("action", "subject", "object") VALUES ($1, $2, $3)"#,
        format!("shared:{}", form.network),
        person.map(|p| p.exterior.uid),
        opp.uid
    )
    .execute(req.state())
    .await?;

    okay_empty()
}

pub async fn calendar(mut req: tide::Request<Database>) -> tide::Result {
    let opp = OpportunityReference::load_by_slug(req.state(), req.param("slug")?).await?;
    let person = request_person(&mut req).await?;
    let form: SharedForm = req.body_json().await?;

    sqlx::query!(
        r#"INSERT INTO c_log ("action", "subject", "object") VALUES ($1, $2, $3)"#,
        format!("calendar:{}", form.network),
        person.map(|p| p.exterior.uid),
        opp.uid
    )
    .execute(req.state())
    .await?;

    okay_empty()
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
        Some(&person.exterior.uid),
        "ui-interest",
        &json!({ "opportunity": slug }),
    );
    person
        .log(db, LogEvent::AddInterest(LogIdentifier::Slug(slug)))
        .await?;

    okay_empty()
}

#[derive(Debug, Deserialize)]
struct StatusForm {
    status: ReviewStatus,
}

pub async fn set_review_status(mut req: tide::Request<Database>) -> tide::Result {
    let person = match request_person(&mut req).await {
        Ok(Some(p)) => p,
        _ => {
            return Err(tide::Error::from_str(
                StatusCode::Unauthorized,
                "Not authorized",
            ))
        }
    };

    let mut opp = OpportunityAll::load_by_slug(req.state(), req.param("slug")?).await?;
    let form: StatusForm = req.body_json().await?;

    if person.check_permission(&Permission::ManageOpportunities)
        || person
            .check_authorization(req.state(), &opp.exterior.opp, PermitAction::Manage)
            .await?
    {
        opp.interior.review_status = form.status;
        opp.store(req.state()).await?;

        if let ReviewStatus::Publish = opp.interior.review_status {
            if let Some(person_uid) = opp.interior.submitted_by {
                let submitted_by = Person::load_by_uid(req.state(), &person_uid).await?;
                let partner = Partner::load_by_uid(req.state(), &opp.exterior.opp.partner).await?;

                let template = common::emails::EmailMessage::load_or_default(
                    req.state(),
                    "opportunity-approved",
                    "Published on Science Near Me: {title}",
                    r#"
<p>The opportunity <strong>{title}</strong> has been published on {partner_name} and Science Near Me.</p>
<p>You can view the opportunity on {partner_name}'s web site or at <a href="https://sciencenearme.org/{opp_slug}">Science Near Me</a>.</p>
"#,
                ).await;

                let msg = template.materialize(vec![
                    ("title", &opp.exterior.opp.title),
                    ("partner_name", &partner.exterior.name),
                    ("opp_slug", &opp.exterior.opp.slug),
                ]);

                common::emails::send_message(submitted_by.interior.email, &msg).await
            }
        } else if let ReviewStatus::Draft = opp.interior.review_status {
            if let Some(person_uid) = opp.interior.submitted_by {
                let submitted_by = Person::load_by_uid(req.state(), &person_uid).await?;

                let template = common::emails::EmailMessage::load_or_default(
                    req.state(),
                    "opportunity-returned-to-draft",
                    "Revisions needed on Science Near Me: {title}",
                    r#"
<p>The opportunity <strong>{title}</strong> has been returned to draft status, as it still requires work in order to be ready for publication.</p>
"#,
                ).await;

                let msg = template.materialize(vec![("title", &opp.exterior.opp.title)]);

                common::emails::send_message(submitted_by.interior.email, &msg).await
            }
        } else if let ReviewStatus::Reject = opp.interior.review_status {
            if let Some(person_uid) = opp.interior.submitted_by {
                let submitted_by = Person::load_by_uid(req.state(), &person_uid).await?;

                let template = common::emails::EmailMessage::load_or_default(
                    req.state(),
                    "opportunity-rejected",
                    "Not published on Science Near Me: {title}",
                    r#"
<p>We're sorry, but the opportunity <strong>{title}</strong> has been rejected for publication.</p>
"#,
                )
                .await;

                let msg = template.materialize(vec![("title", &opp.exterior.opp.title)]);

                common::emails::send_message(submitted_by.interior.email, &msg).await
            }
        }

        okay_empty()
    } else {
        Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "Not authorized",
        ))
    }
}
