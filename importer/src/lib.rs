use thiserror::Error;

pub mod format;
pub mod source;
pub mod structure;

pub use format::Format;
pub use source::Source;
pub use structure::Structure;

#[derive(Debug, Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
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
}

pub struct Importer<Src: Source, Fmt: Format, Struct: Structure> {
    source: Src,
    format: Fmt,
    structure: Struct,
}

impl<Src: Source, Fmt: Format, Struct: Structure> Importer<Src, Fmt, Struct> {
    pub fn new(source: Src, format: Fmt, structure: Struct) -> Self {
        Importer {
            source,
            format,
            structure,
        }
    }

    pub async fn update_db<'req, DB>(&self, db: DB) -> Result<(), Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        todo!()
    }
}
