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
}
