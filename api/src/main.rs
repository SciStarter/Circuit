#![forbid(unsafe_code)]

use common::{model, Database, INTERNAL_UID};
use sqlx::postgres::PgPoolOptions;
use tide::log;
use tide_fluent_routes::{fs::ServeFs, prelude::*};

pub mod ui;
pub mod v1;

async fn initialize(db: &Database) -> tide::Result {
    let superuser_email = std::env::var("SUPERUSER_EMAIL")?;
    let superuser_password = std::env::var("SUPERUSER_PASSWORD")?;

    // Ensure that the superuser exists
    let superuser = match model::person::Person::load_by_email(db, &superuser_email).await {
        Ok(person) => person,
        Err(_) => {
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
            superuser
        }
    };

    if !model::partner::Partner::exists_by_uid(db, &INTERNAL_UID).await? {
        log::info!("Creating internal partner entry...");
        let mut internal = model::partner::Partner {
            id: None,
            exterior: model::partner::PartnerExterior {
                uid: INTERNAL_UID.clone(),
                name: "Internal".to_string(),
                image_url: None,
                description: "Partner entry representing internal operations".to_string(),
                under: None,
            },
            interior: model::partner::PartnerInterior {
                manager: model::partner::Contact {
                    name: superuser
                        .exterior
                        .username
                        .unwrap_or_else(|| "System".to_string()),
                    email: superuser.interior.email,
                    phone: None,
                    mailing: None,
                },
                contact: None,
                prime: superuser.exterior.uid,
                authorized: vec![],
                pending: vec![],
                secret: None,
            },
        };

        internal.store(db).await?;
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
    log::with_level(log::LevelFilter::Debug);

    initialize(&pool).await?;

    let mut app = tide::with_state(pool);

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
