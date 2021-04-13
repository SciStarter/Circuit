use once_cell::sync::Lazy;
pub use opportunity::{Opportunity, OpportunityExterior, OpportunityInterior};
use thiserror::Error;

pub mod opportunity;
pub mod person;

pub static ROOT_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("f6d641e2-75b3-4dce-be29-082f74f44b80").unwrap());

pub static PERSON_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::new_v5(&ROOT_NAMESPACE, b"person"));

pub static PARTNER_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::new_v5(&ROOT_NAMESPACE, b"partner"));

pub static OPPORTUNITY_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::new_v5(&ROOT_NAMESPACE, b"opportunity"));

#[derive(Debug, Error)]
pub enum Error {
    #[error("Required field is missing or empty: {0}")]
    Missing(String),
    #[error("SQLx error")]
    SQLx(#[from] sqlx::Error),
    #[error("JSON error")]
    JSON(#[from] serde_json::Error),
}

// #[derive(Debug, Default, sqlx::Type)]
// #[sqlx(type_name = "Person")]
// pub struct Person {}

// #[derive(Debug, Default, sqlx::Type)]
// #[sqlx(type_name = "Participation")]
// pub struct Participation {}
