use super::Error;
use crate::{Database, ToFixedOffset};

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

impl Mode {
    fn as_db_str(&self) -> &'static str {
        match self {
            Mode::Signup => "signup",
            Mode::CollectData => "collect-data",
            Mode::AnalyzeData => "analyze-data",
            Mode::Organize => "organize",
        }
    }

    fn from_db_str(s: &str) -> Result<Self, Error> {
        match s {
            "signup" => Ok(Mode::Signup),
            "collect-data" => Ok(Mode::CollectData),
            "analyze-data" => Ok(Mode::AnalyzeData),
            "organize" => Ok(Mode::Organize),
            other => Err(Error::Value(format!("Unknown participation mode: {}", other))),
        }
    }
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
            exterior: ParticipationExterior {
                opportunity: rec.opportunity,
                partner: rec.partner,
                when: rec.r#when.to_fixed_offset(),
                mode: Mode::from_db_str(&rec.mode)?,
                keywords: rec.keywords,
            },
            interior: ParticipationInterior {
                participant: rec.participant,
                snml: rec.snml,
                location: rec.location,
            },
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
                self.exterior.opportunity,
                self.exterior.partner,
                self.exterior.when,
                self.exterior.mode.as_db_str(),
                &self.exterior.keywords,
                self.interior.participant as Option<Uuid>,
                self.interior.snml.as_deref(),
                self.interior.location.clone() as Option<serde_json::Value>,
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
                self.exterior.opportunity,
                self.exterior.partner,
                self.exterior.when,
                self.exterior.mode.as_db_str(),
                &self.exterior.keywords,
                self.interior.participant as Option<Uuid>,
                self.interior.snml.as_deref(),
                self.interior.location.clone() as Option<serde_json::Value>,
            )
            .fetch_one(db)
            .await?;

            self.id = Some(rec.id);

            crate::log(
                self.interior.participant.as_ref(),
                "recorded participation",
                self,
            );
        };

        Ok(())
    }
}
