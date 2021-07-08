use super::{opportunity::Opportunity, person::Person, Error, PARTNER_NAMESPACE};

use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Contact {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub mailing: Option<String>,
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
    pub secret: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PartnerReference {
    pub id: i32,
    pub uid: Uuid,
    pub name: String,
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
    pub fn set_secret(&mut self, secret: &str) {
        self.interior.secret = Some(djangohashers::make_password(secret));
    }

    pub fn check_secret_full(&self, secret: &str) -> Option<bool> {
        if let Some(hashed) = &self.interior.secret {
            Some(djangohashers::check_password_tolerant(secret, &hashed))
        } else {
            None
        }
    }

    pub fn check_secret(&self, secret: &str) -> bool {
        if let Some(valid) = self.check_secret_full(secret) {
            valid
        } else {
            false
        }
    }

    pub async fn catalog<'req, DB>(db: DB) -> Result<Vec<PartnerReference>, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        sqlx::query_file!("db/partner/catalog.sql")
            .map(|row| {
                Ok(PartnerReference {
                    id: row.id,
                    uid: serde_json::from_value(
                        row.uid.ok_or_else(|| Error::Missing("uid".to_string()))?,
                    )?,
                    name: serde_json::from_value(
                        row.name.ok_or_else(|| Error::Missing("name".to_string()))?,
                    )?,
                })
            })
            .fetch_all(db)
            .await?
            .into_iter()
            .collect()
    }

    pub async fn load_persons<'req, DB>(&self, db: DB) -> Result<Vec<Result<Person, Error>>, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let mut persons = self.interior.authorized.clone();
        persons.push(self.interior.prime.clone());

        Ok(sqlx::query_file!(
            "db/partner/fetch_persons.sql",
            serde_json::to_value(persons)?
        )
        .map(|row| {
            Ok(Person {
                id: Some(row.id),
                exterior: serde_json::from_value(row.exterior)?,
                interior: serde_json::from_value(row.interior)?,
            })
        })
        .fetch_all(db)
        .await?)
    }

    pub async fn load_opportunities<'req, DB>(
        &self,
        db: DB,
    ) -> Result<Vec<Result<Opportunity, Error>>, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        Ok(sqlx::query_file!(
            "db/partner/fetch_opportunities.sql",
            serde_json::to_value(self.exterior.uid)?
        )
        .map(|row| {
            Ok(Opportunity {
                id: Some(row.id),
                exterior: serde_json::from_value(row.exterior)?,
                interior: serde_json::from_value(row.interior)?,
            })
        })
        .fetch_all(db)
        .await?)
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        self.exterior.name = self.exterior.name.trim_matches(char::is_whitespace).into();

        if self.exterior.name.is_empty() {
            return Err(Error::Missing("name".into()));
        }

        if self.interior.prime.is_nil() {
            return Err(Error::Missing("prime".into()));
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
        let rec = sqlx::query_file!("db/partner/get_by_uid.sql", serde_json::to_value(uid)?)
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
        let rec = sqlx::query_file!("db/partner/exists_by_uid.sql", serde_json::to_value(uid)?)
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
