use super::Error;

use serde::{Deserialize, Serialize};
use sqlx;
use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use uuid::Uuid;

use super::PARTNER_NAMESPACE;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Contact {
    name: String,
    email: String,
    phone: Option<String>,
    mailing: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PartnerExterior {
    pub uid: Uuid,
    pub name: String,
    pub image_url: Option<String>,
    pub description: String,
    pub under: Option<Uuid>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PartnerInterior {
    pub manager: Contact,
    pub contact: Option<Contact>,
    pub prime: Uuid,           // uid of the prime Person entry for this partner
    pub authorized: Vec<Uuid>, // uids of additional authorized Person entries
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Partner {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: PartnerExterior,
    #[serde(flatten)]
    pub interior: PartnerInterior,
}

impl Partner {
    pub fn validate(&mut self) -> Result<(), Error> {
        self.exterior.name = self.exterior.name.trim_matches(char::is_whitespace).into();

        if self.exterior.name.is_empty() {
            return Err(Error::Missing("name".into()));
        }

        if self.interior.prime.is_nil() {
            return Err(Error::Missing("manager".into()));
        }

        if self.exterior.uid.is_nil() {
            self.exterior.uid = Uuid::new_v5(&PARTNER_NAMESPACE, self.exterior.name.as_ref());
        }

        Ok(())
    }

    pub async fn load_by_id<'req, DB>(db: DB, id: i32) -> Result<Partner, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/partner/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Partner {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid<'req, DB>(db: DB, uid: &Uuid) -> Result<Partner, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/partner/get_by_uid.sql", uid)
            .fetch_one(db)
            .await?;

        Ok(Partner {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_uid<'req, DB>(db: DB, uid: &Uuid) -> Result<bool, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/partner/exists_by_uid.sql", uid)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn store<'req, DB>(&mut self, db: DB) -> Result<(), Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        self.validate()?;

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/partner/update.sql",
                id,
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/partner/insert.sql",
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
