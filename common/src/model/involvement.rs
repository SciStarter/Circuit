use chrono::{DateTime, FixedOffset};
use futures::{Stream, TryStreamExt};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use uuid::Uuid;

use crate::{model::Error, Database};

#[derive(Debug, Serialize_repr, Deserialize_repr, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    Saved = 1,
    Logged = 2,
    Contributed = 3,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvolvementExterior {
    pub opportunity: Uuid,
    pub first: DateTime<FixedOffset>,
    pub latest: DateTime<FixedOffset>,
    pub mode: Mode,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InvolvementInterior {
    pub participant: Uuid,
    pub location: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Involvement {
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: InvolvementExterior,
    #[serde(flatten)]
    pub interior: InvolvementInterior,
}

impl Involvement {
    pub fn validate(&mut self) -> Result<(), Error> {
        Ok(())
    }

    pub async fn upgrade(
        db: &Database,
        participant: &Uuid,
        opportunity: &Uuid,
        mode: Mode,
        location: &Option<serde_json::Value>,
    ) -> Result<(), Error> {
        sqlx::query_file!(
            "db/involvement/upgrade.sql",
            serde_json::to_value(participant)?,
            serde_json::to_value(opportunity)?,
            serde_json::to_value(mode)?,
            serde_json::to_value(location)?
        )
        .execute(db)
        .await?;

        Ok(())
    }

    pub async fn all_for_participant<'db>(
        db: &'db Database,
        participant: &Uuid,
    ) -> Result<impl Stream<Item = Result<Involvement, Error>> + 'db, Error> {
        Ok(sqlx::query_file!(
            "db/involvement/all_by_participant.sql",
            serde_json::to_value(participant)?
        )
        .try_map(|rec| {
            Ok(Involvement {
                id: Some(rec.id),
                exterior: serde_json::from_value(rec.exterior)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
                interior: serde_json::from_value(rec.interior)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            })
        })
        .fetch(db)
        .err_into())
    }

    pub async fn all_for_opportunity<'db>(
        db: &'db Database,
        opportunity: &Uuid,
    ) -> Result<impl Stream<Item = Result<Involvement, Error>> + 'db, Error> {
        Ok(sqlx::query_file!(
            "db/involvement/all_by_opportunity.sql",
            serde_json::to_value(opportunity)?,
        )
        .try_map(|rec| {
            Ok(Involvement {
                id: Some(rec.id),
                exterior: serde_json::from_value(rec.exterior)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
                interior: serde_json::from_value(rec.interior)
                    .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            })
        })
        .fetch(db)
        .err_into())
    }

    pub async fn load_by_id(db: &Database, id: i32) -> Result<Involvement, Error> {
        let rec = sqlx::query_file!("db/involvement/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Involvement {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_participant_and_opportunity(
        db: &Database,
        participant: &Uuid,
        opportunity: &Uuid,
    ) -> Result<Option<Involvement>, Error> {
        if let Some(rec) = sqlx::query_file!(
            "db/involvement/get_by_participant_and_opportunity.sql",
            serde_json::to_value(participant)?,
            serde_json::to_value(opportunity)?
        )
        .fetch_optional(db)
        .await?
        {
            Ok(Some(Involvement {
                id: Some(rec.id),
                exterior: serde_json::from_value(rec.exterior)?,
                interior: serde_json::from_value(rec.interior)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn store(&mut self, db: &Database) -> Result<(), Error> {
        self.validate()?;

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/involvement/update.sql",
                id,
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/involvement/insert.sql",
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .fetch_one(db)
            .await?;

            self.id = Some(rec.id);
        };

        Ok(())
    }
}
