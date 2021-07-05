#![forbid(unsafe_code)]

use common::model;
use sqlx::postgres::Postgres;
use sqlx::{postgres::PgPoolOptions, Pool};
use tide::log;
use tide_fluent_routes::{fs::ServeFs, prelude::*};
use tide_sqlx::SQLxMiddleware;

pub mod ui;
pub mod v1;

async fn initialize(db: &Pool<Postgres>) -> tide::Result {
    let superuser_email = std::env::var("SUPERUSER_EMAIL")?;
    let superuser_password = std::env::var("SUPERUSER_PASSWORD")?;

    if !model::person::Person::exists_by_email(db, &superuser_email).await? {
        log::info!("Creating superuser...");
        let mut superuser = model::person::Person::default();
        superuser.exterior.username = Some("System".to_string());
        superuser.interior.email = superuser_email;
        superuser.set_password(&superuser_password);
        superuser
            .interior
            .permissions
            .push(model::person::Permission::All);
        superuser.store(db).await?;
    }

    Ok("initialized".into())
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .connect(&std::env::var("DATABASE_URL")?)
        .await?;

    common::migrate(&pool).await?;

    #[cfg(not(debug_assertions))]
    log::with_level(log::LevelFilter::Warn);
    #[cfg(debug_assertions)]
    log::with_level(log::LevelFilter::Info);

    initialize(&pool).await?;

    let mut app = tide::new();
    app.with(SQLxMiddleware::from(pool));

    // https://crates.io/crates/tide-fluent-routes
    app.register(
        root()
            .at("api/v1/", v1::routes)
            .at("api/ui/", ui::routes)
            .at("api/docs/", |routes| {
                routes
                    .serve_dir("static/")
                    .expect("Unable to serve static docs dir")
            }),
    );

    app.listen("0.0.0.0:8000").await?;

    Ok(())
}
