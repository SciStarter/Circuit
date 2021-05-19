use common::model::Opportunity;
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

impl<Src, Fmt, Struct> Importer<Src, Fmt, Struct>
where
    Src: Source,
    Fmt: Format,
    Struct: Structure,
{
    pub fn new(source: Src, format: Fmt, structure: Struct) -> Self {
        Importer {
            source,
            format,
            structure,
        }
    }

    pub fn load(&self) -> Result<structure::OneOrMany<Struct::Data>, Error> {
        self.structure
            .interpret(self.format.decode(self.source.load()?)?)
    }
}

async fn maybe_store_opportunity<'db, DB>(db: DB, update: &mut Opportunity) -> Result<(), Error>
where
    DB: sqlx::Executor<'db, Database = sqlx::Postgres> + Clone,
{
    if update.exterior.uid.is_nil() {
        update.validate()?;
    }

    if let Ok(prior) = Opportunity::load_by_uid(db.clone(), &update.exterior.uid).await {
        match (
            prior.exterior.partner_updated,
            update.exterior.partner_updated,
        ) {
            (None, None) => {}
            (None, Some(_)) => {}
            (Some(_), None) => return Ok(()),
            (Some(p), Some(u)) => {
                if p >= u {
                    return Ok(());
                }
            }
        }
    }

    update.store(db).await?;

    Ok(())
}

impl<Src, Fmt, Struct> Importer<Src, Fmt, Struct>
where
    Src: Source,
    Fmt: Format,
    Struct: Structure<Data = common::model::Opportunity>,
{
    pub async fn update<'db, DB>(&self, db: DB) -> Result<(), Error>
    where
        DB: sqlx::Executor<'db, Database = sqlx::Postgres> + Clone,
    {
        // The self.load() call blocks the executor while doing
        // synchronous network I/O. This is not ideal, in general, but
        // the only reason we're even using an asynchronous executor
        // is because the SQLx API is asynchronous. We're handling the
        // data sources serially on purpose, so blocking the executor
        // while we download from one of them is not a problem.
        match self.load()? {
            structure::OneOrMany::One(mut value) => {
                maybe_store_opportunity(db, &mut value).await?;
            }
            structure::OneOrMany::Many(values) => {
                for mut value in values {
                    maybe_store_opportunity(db.clone(), &mut value).await?;
                }
            }
        }

        Ok(())
    }
}
