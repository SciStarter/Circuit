use once_cell::sync::Lazy;
pub use opportunity::{Opportunity, OpportunityExterior, OpportunityInterior};
pub use partner::Partner;
pub use person::Person;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod block;
pub mod geojson;
pub mod involvement;
pub mod opportunity;
pub mod participation;
pub mod partner;
pub mod person;

pub static ROOT_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("f6d641e2-75b3-4dce-be29-082f74f44b80").unwrap());

pub static PERSON_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::new_v5(&ROOT_NAMESPACE, b"person"));

pub static PARTNER_NAMESPACE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::new_v5(&ROOT_NAMESPACE, b"partner"));

#[derive(Debug, Error)]
pub enum Error {
    #[error("Required field is missing or empty: {0}")]
    Missing(String),
    #[error("SQLx error")]
    SQLx(#[from] sqlx::Error),
    #[error("JSON error")]
    JSON(#[from] serde_json::Error),
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Pagination {
    All,
    One,
    Page { index: u32, size: u32 },
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination::Page { index: 0, size: 10 }
    }
}

impl Pagination {
    pub fn expand(&self, total: u32) -> (u32, u32, u32) {
        if total == 0 {
            return (0, 0, 0);
        }

        let last_index = total - 1;

        match self {
            Pagination::All => (0, 0, total),
            Pagination::One => (0, 0, 1),
            Pagination::Page { index, size } => (
                *index,
                (last_index as f64 / *size as f64).floor() as u32,
                *size,
            ),
        }
    }
}

/// Returns a string containing spaces inserted before each capital
/// letter (except the first, if the first capital is also the first
/// letter of the input string)
// The vec! expressions are used to get variable length iterators, but
// it could be done better. Revisit if this becomes a bottleneck.
pub fn separate_camel_case(input: &str) -> String {
    input
        .chars()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && c.is_ascii_uppercase() {
                vec![' ', c].into_iter()
            } else {
                vec![c].into_iter()
            }
        })
        .collect()
}

pub trait SelectOption: Sized {
    fn all_options() -> Vec<(String, String, Self)>;
    fn to_option(&self) -> (String, String, Self);
}

// #[derive(Debug, Default, sqlx::Type)]
// #[sqlx(type_name = "Person")]
// pub struct Person {}

// #[derive(Debug, Default, sqlx::Type)]
// #[sqlx(type_name = "Participation")]
// pub struct Participation {}
