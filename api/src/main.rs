#![deny(unsafe_code)]

use sqlx::postgres::{PgPoolOptions, Postgres};
use sqlx::prelude::*;
use tide::prelude::*;
use tide_fluent_routes::prelude::*;
use tide_sqlx::{SQLxMiddleware, SQLxRequestExt};
use tide_websockets::{Message, WebSocket};

pub mod model;

async fn dummy(mut req: tide::Request<()>) -> tide::Result {
    let mut db = req.sqlx_conn::<Postgres>().await;
    println!("{:?}", req);
    Ok("Hello web world".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    let mut app = tide::new();
    app.with(SQLxMiddleware::from(pool));

    // https://crates.io/crates/tide-fluent-routes
    app.register(root().get(dummy).at("api/", |routes| routes.get(dummy)));

    println!("Listening on 0.0.0.0:8000");
    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
