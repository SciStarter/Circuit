use std::fmt::Debug;
use std::time::Duration;

use async_trait::async_trait;
use thiserror::Error;

pub mod format;
pub mod source;
pub mod structure;

#[derive(Debug, Error)]
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
    #[error("Model error")]
    Model(#[from] common::model::Error),
    #[error("Incorrectly structured data")]
    Structure(String),
    #[error("Unable to comprehend data")]
    Comprehension,
}

#[async_trait]
pub trait Importer: Debug {
    async fn import(
        &self,
        db: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<std::time::Duration>, Error>;
}
