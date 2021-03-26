#![deny(unsafe_code)]

use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide_sqlx::{SQLxMiddleware, SQLxRequestExt};

use tide::prelude::*;
use tide_fluent_routes::prelude::*;
use tide_websockets::{Message, WebSocket};

async fn dummy(mut req: tide::Request<()>) -> tide::Result {
    println!("{:?}", req);
    Ok("Hello web world".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // https://crates.io/crates/tide-fluent-routes
    app.register(root().get(dummy).at("api/", |routes| routes.get(dummy)));

    println!("Listening on 0.0.0.0:8000");
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
