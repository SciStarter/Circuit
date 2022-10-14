use super::Error;
use crate::Database;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Mode {
    Signup,
    CollectData,
    AnalyzeData,
    Organize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationExterior {
    pub opportunity: Uuid,
    #[serde(default)]
    pub partner: Uuid,
    pub when: DateTime<FixedOffset>,
    pub mode: Mode,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationInterior {
    #[serde(default)]
    pub participant: Option<Uuid>,
    #[serde(default)]
    pub snml: Option<String>,
    #[serde(default)]
    pub location: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Participation {
    #[serde(default)]
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: ParticipationExterior,
    #[serde(flatten)]
    pub interior: ParticipationInterior,
}

impl Participation {
    pub fn validate(&mut self) -> Result<(), Error> {
        if self.interior.participant.is_some() && self.interior.snml.is_some() {
            Err(Error::Exclusive(
                String::from("participant"),
                String::from("snml"),
            ))
        } else if self.interior.participant.is_none() && self.interior.snml.is_none() {
            Err(Error::Missing(String::from("particpant or snml")))
        } else {
            Ok(())
        }
    }

    pub async fn load_by_id(db: &Database, id: i32) -> Result<Participation, Error> {
        let rec = sqlx::query_file!("db/participation/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Participation {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn store(&mut self, db: &Database, force_new: bool) -> Result<(), Error> {
        self.validate()?;

        if force_new {
            self.id = None;
        }

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/participation/update.sql",
                id,
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .execute(db)
            .await?;
        } else {
            if let Some(participant) = &self.interior.participant {
                super::involvement::Involvement::upgrade(
                    db,
                    participant,
                    &self.exterior.opportunity,
                    super::involvement::Mode::Contributed,
                    &self.interior.location,
                )
                .await?;
            }

            let rec = sqlx::query_file!(
                "db/participation/insert.sql",
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .fetch_one(db)
            .await?;

            self.id = Some(rec.id);

            crate::log("recorded participation", self);
        };

        Ok(())
    }
}
