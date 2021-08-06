use common::Database;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .post(add_organization)
        .at("all", |r| r.get(my_organizations))
        .at("exists", |r| r.get(check_organization))
        .at(":uid", |r| {
            r.get(get_organization)
                .put(save_organization)
                .at("pending-managers", |r| {
                    r.get(get_pending_managers)
                        .post(add_pending_manager)
                        .delete(remove_pending_manager)
                })
                .at("managers", |r| {
                    r.get(get_managers).post(add_manager).delete(remove_manager)
                })
                .at("opportunities", |r| {
                    r.post(add_opportunity).at(":opp_uid", |r| {
                        r.post(duplicate_opportunity).delete(withdraw_opportunity)
                    })
                })
        })
}

pub async fn add_organization(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

#[derive(serde::Deserialize, Debug)]
struct CheckOrganizationQuery {
    name: String,
}

pub async fn check_organization(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn my_organizations(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn get_organization(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn save_organization(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn get_pending_managers(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn add_pending_manager(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn remove_pending_manager(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn get_managers(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn add_manager(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn remove_manager(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn add_opportunity(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn duplicate_opportunity(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

// If the opportunity is not approved, delete it instead of marking it
// withdrawn.
pub async fn withdraw_opportunity(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}
