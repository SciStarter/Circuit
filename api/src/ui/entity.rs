use common::{
    model::{
        opportunity::{Opportunity, OpportunityQuery, OpportunityQueryOrdering},
        person::Permission,
        Pagination,
    },
    Database,
};
use tide::{Status, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

use super::{okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at(":slug", |r| {
        r.get(entity)
            .at("likes", |r| r.get(get_likes).post(add_like))
            .at("saves", |r| r.get(get_saves))
            .at("didit", |r| r.get(get_didit))
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

    if opp.interior.accepted
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
    okay(&common::model::opportunity::didit_for_slug(db, &slug).await?)
}

pub async fn get_saves(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::saves_for_slug(db, &slug).await?)
}

pub async fn get_likes(req: tide::Request<Database>) -> tide::Result {
    let slug = req.param("slug")?;
    let db = req.state();
    okay(&common::model::opportunity::likes_for_slug(db, &slug).await?)
}

pub async fn add_like(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-like", "");
    todo!()
}

pub async fn add_review(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-review", "");
    todo!()
}

pub async fn report_review(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-report-review", "");
    todo!()
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

    if point.is_some() {
        query.near = point;
        ordering = OpportunityQueryOrdering::Closest;
    } else {
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
        // Then we randomly drop half of the matches. Our
        // recommendations are the first five survivors
        query.sample = Some(0.5);
        ordering = OpportunityQueryOrdering::Any;
    }

    let matches = Opportunity::load_matching(db, &query, ordering, pagination).await?;

    okay(&matches)
}

pub async fn request_page_management(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-request-page-management", "");
    todo!()
}
