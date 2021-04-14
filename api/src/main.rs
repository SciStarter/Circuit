#![forbid(unsafe_code)]

use sqlx::postgres::PgPoolOptions;
use tide::log;
use tide_fluent_routes::{fs::ServeFs, prelude::*};
use tide_sqlx::SQLxMiddleware;

pub mod model;
pub mod v1;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    log::with_level(log::LevelFilter::Info);

    let mut app = tide::new();

    app.with(SQLxMiddleware::from(pool));

    // https://crates.io/crates/tide-fluent-routes
    app.register(root().at("api/v1/", v1::routes).at("api/docs/", |routes| {
        routes
            .serve_dir("static/")
            .expect("Unable to serve static docs dir")
    }));

    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
