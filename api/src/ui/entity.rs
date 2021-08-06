use common::{
    model::{person::Permission, Opportunity},
    Database,
};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

use super::{error, okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at(":slug", |r| {
        r.get(entity)
            .at("like", |r| r.post(add_like))
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

    let opp = Opportunity::load_by_slug(db, slug).await?;

    if opp.interior.accepted
        || person
            .map(|p| p.check_permission(&Permission::ManageOpportunities))
            .unwrap_or(false)
    {
        okay("", &opp.exterior)
    } else {
        error(404, "not found", &[slug])
    }
}

pub async fn add_like(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn add_review(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn report_review(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn reviews(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn recommended(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn request_page_management(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}
