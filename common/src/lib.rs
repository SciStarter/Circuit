use sqlx::{postgres::Postgres, Pool};
pub mod model;

pub async fn migrate(pool: &Pool<Postgres>) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!().run(pool).await
}
