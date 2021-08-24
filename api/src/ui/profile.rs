use common::Database;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(get_profile)
        .put(save_profile)
        .delete(delete_profile)
        .at("saved", |r| {
            r.put(add_saved)
                .delete(delete_saved)
                .at("old", |r| r.delete(delete_old_saved))
        })
        .at("participated", |r| {
            r.put(add_participated)
                .delete(remove_participated)
                .get(get_participated)
                .at(":slug", |r| r.get(get_didit).put(set_didit))
        })
}

pub async fn get_profile(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn save_profile(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-save-profile", "");
    todo!()
}

pub async fn delete_profile(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-delete-profile", "");
    todo!()
}

pub async fn delete_old_saved(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-delete-old-saved", "");
    todo!()
}

pub async fn delete_saved(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-delete-saved", "");
    todo!()
}

pub async fn add_saved(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-add-saved", "");
    todo!()
}

pub async fn add_participated(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-add-participated", "");
    todo!()
}

pub async fn remove_participated(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-remove-participated", "");
    todo!()
}

pub async fn get_participated(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn get_didit(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn set_didit(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-set-didit", "");
    todo!()
}
