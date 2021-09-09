use chrono::{DateTime, Utc};
use common::{
    model::{
        involvement::{Involvement, Mode},
        opportunity::{Opportunity, OpportunityQuery, OpportunityQueryOrdering},
        person::Permission,
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

use super::{okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at(":slug", |r| {
        r.get(entity)
            .at("me", |r| r.get(get_me))
            .at("likes", |r| r.get(get_likes).post(add_like))
            .at("saves", |r| r.get(get_saves).post(add_save))
            .at("didit", |r| r.get(get_didit).post(add_didit))
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

    if opp.interior.accepted.unwrap_or(false)
        || person
            .map(|p| p.check_permission(&Permission::ManageOpportunities))
            .unwrap_or(false)
    {
        okay(&opp.exterior)
    } else {
        Err(tide::Error::from_str(StatusCode::NotFound, "not found"))
    }
}

pub async fn get_didit(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::didits_for_slug(db, &slug).await?)
}

pub async fn add_didit(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;
    let db = req.state();

    common::model::opportunity::add_save_for_slug(db, &slug, &person.exterior.uid).await?;

    common::log(
        "ui-didit",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );

    okay("")
}

pub async fn get_saves(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();

    okay(&common::model::opportunity::saves_for_slug(db, &slug).await?)
}

pub async fn add_save(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(400, "authentication required"))?;
    let db = req.state();

    common::model::opportunity::add_save_for_slug(db, &slug, &person.exterior.uid).await?;

    common::log(
        "ui-save",
        &json!({"person": person.exterior.uid, "opportunity": slug}),
    );

    okay("")
}

pub async fn get_likes(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::likes_for_slug(db, &slug).await?)
}

pub async fn add_like(mut req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?.to_string();
    let person = request_person(&mut req).await?;
    let db = req.state();

    common::model::opportunity::add_like_for_slug(
        db,
        &slug,
        &person.as_ref().map(|p| p.exterior.uid),
    )
    .await?;

    common::log(
        "ui-like",
        &json!({"person": person.map(|p| p.exterior.uid), "opportunity": slug}),
    );

    okay("")
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

    let id = common::model::opportunity::add_review_for_slug(
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
        common::model::opportunity::report_review(db, report.id).await?;
        common::log(
            "ui-report-review",
            &json!({"person": person.exterior.uid, "review": report.id}),
        );
    }
    okay("")
}

pub async fn reviews(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::reviews_for_slug(db, &slug).await?)
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
                "like": common::model::opportunity::likes_for_slug_and_person(db, &slug, &person.exterior.uid).await? > 0,
                "save": involvement.exterior.mode >= Mode::Saved,
                "didit": involvement.exterior.mode >= Mode::Logged,
            }))
        } else {
            okay(&json!({
                "like": common::model::opportunity::likes_for_slug_and_person(db, &slug, &person.exterior.uid).await? > 0,
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
