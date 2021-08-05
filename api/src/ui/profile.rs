use common::Database;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at("saved", |r| {
        r.at("delete-old", |r| r.put(delete_old_saved))
            .at("delete", |r| r.put(delete_saved))
    })
}

pub async fn delete_old_saved(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn delete_saved(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}
