#![deny(unsafe_code)]

use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide_sqlx::{SQLxMiddleware, SQLxRequestExt};

use tide::prelude::*;
use tide_fluent_routes::prelude::*;
use tide_websockets::{Message, WebSocket};

async fn dummy(mut _req: tide::Request<()>) -> tide::Result {
    Ok("Hello web world".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // https://crates.io/crates/tide-fluent-routes
    app.register(root().get(dummy));

    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
