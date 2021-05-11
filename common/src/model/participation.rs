use super::Error;

use serde::{Deserialize, Serialize};
use sqlx;
use time::OffsetDateTime;
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
    pub when: OffsetDateTime,
    pub mode: Mode,
    #[serde(default)]
    pub keywords: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParticipationInterior {
    #[serde(default)]
    pub participant: Option<Uuid>,
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
        Ok(())
    }

    pub async fn load_by_id<'req, DB>(db: DB, id: i32) -> Result<Participation, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/participation/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Participation {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn store<'req, DB>(&mut self, db: DB, force_new: bool) -> Result<(), Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
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
            let rec = sqlx::query_file!(
                "db/participation/insert.sql",
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
