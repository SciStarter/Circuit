use sqlx::{postgres::Postgres, Pool};
use thiserror::Error;
pub mod jwt;
pub mod model;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Authorization failed")]
    Auth(String),
    #[error("JWT operation failed")]
    JWT(#[from] ::jwt::Error),
    #[error("UUID operation failed")]
    Uuid(#[from] uuid::Error),
    #[error("Migration failed")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub async fn migrate(pool: &Pool<Postgres>) -> Result<(), Error> {
    sqlx::migrate!().run(pool).await?;
    Ok(())
}
