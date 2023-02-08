use std::fmt::Debug;

use async_trait::async_trait;
use common::model::{partner::LoggedError, Partner};

pub mod format;
pub mod source;
pub mod structure;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
    #[error("Incorrectly encoded data")]
    Unicode(#[from] std::str::Utf8Error),
    #[error("HTTP error")]
    Http(#[from] ureq::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("ICal parse error")]
    IcalParse(#[from] ical::parser::ParserError),
    #[error("{0}")]
    Logged(LoggedError),
    #[error("Model error")]
    Model(#[from] common::model::Error),
    #[error("Incorrectly structured data")]
    Structure(String),
    #[error("Unable to comprehend data")]
    Comprehension,
    #[error("{0}")]
    Misc(String),
}

impl From<LoggedError> for Error {
    fn from(value: LoggedError) -> Self {
        Error::Logged(value)
    }
}

#[async_trait]
pub trait Importer: Debug {
    async fn import(
        &self,
        db: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<std::time::Duration>, Error>;

    async fn load_partner(&self, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Partner, Error>;
}
