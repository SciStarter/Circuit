use std::fmt::Display;

use super::{
    opportunity::{Domain, Opportunity, OrganizationType},
    person::Person,
    Error, PARTNER_NAMESPACE,
};
use crate::{Database, INTERNAL_UID};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum LoggedErrorLevel {
    #[default]
    Debug,
    Warning,
    Error,
}

impl TryFrom<i16> for LoggedErrorLevel {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, <LoggedErrorLevel as TryFrom<i16>>::Error> {
        match value {
            0 => Ok(LoggedErrorLevel::Debug),
            1 => Ok(LoggedErrorLevel::Warning),
            2 => Ok(LoggedErrorLevel::Error),
            _ => Err(Error::OutOfBounds("error level".into())),
        }
    }
}

impl From<LoggedErrorLevel> for i16 {
    fn from(value: LoggedErrorLevel) -> Self {
        match value {
            LoggedErrorLevel::Debug => 0,
            LoggedErrorLevel::Warning => 1,
            LoggedErrorLevel::Error => 2,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct LoggedError {
    pub id: i64,
    #[serde(skip_serializing)]
    pub partner_id: i32,
    pub when: DateTime<FixedOffset>,
    pub title: Option<String>,
    pub raw: Option<String>,
    pub level: LoggedErrorLevel,
    pub message: String,
}

impl LoggedError {
    pub fn new(level: impl Into<LoggedErrorLevel>, message: impl AsRef<str>) -> LoggedError {
        LoggedError {
            level: level.into(),
            message: message.as_ref().to_string(),
            ..Default::default()
        }
    }

    pub fn set_title(mut self, title: impl AsRef<str>) -> Self {
        self.title = Some(title.as_ref().to_string());
        self
    }

    pub fn set_raw(mut self, raw: impl AsRef<str>) -> Self {
        self.raw = Some(raw.as_ref().to_string());
        self
    }

    pub async fn store(&self, db: &Database) -> Result<i64, Error> {
        if self.partner_id <= 0 {
            return Err(Error::Value("Can not save without partner_id".into()));
        }

        Ok(sqlx::query_scalar!(
            r#"
INSERT
  INTO c_partner_error_log (partner_id, "level", "message", "title", "raw")
  VALUES ($1, $2, $3, $4, $5)
  RETURNING id
"#,
            self.partner_id,
            i16::from(self.level),
            self.message,
            self.title,
            self.raw,
        )
        .fetch_one(db)
        .await?)
    }
}

impl<E: std::error::Error> From<E> for LoggedError {
    fn from(value: E) -> Self {
        LoggedError {
            level: LoggedErrorLevel::Error,
            message: value.to_string(),
            ..Default::default()
        }
    }
}

impl Display for LoggedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({})",
            self.message,
            self.title.as_deref().unwrap_or("")
        )
    }
}

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
    pub organization_type: OrganizationType,
    pub pes_domain: Domain,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub description: String,
    pub background_color: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub tertiary_color: Option<String>,
    pub under: Option<Uuid>,
    pub open_submission: Option<bool>,
    pub default_query: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct PartnerInterior {
    pub manager: Contact,
    pub contact: Option<Contact>,
    pub prime: Uuid,           // uid of the prime Person entry for this partner
    pub authorized: Vec<Uuid>, // uids of additional authorized Person entries
    pub pending: Vec<Uuid>,    // uids of Person entries that can be authorized
    pub secret: Option<String>,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PartnerReference {
    pub id: i32,
    pub uid: Uuid,
    pub name: String,
}

impl From<Partner> for PartnerReference {
    fn from(partner: Partner) -> Self {
        PartnerReference {
            id: partner.id.unwrap_or(0),
            uid: partner.exterior.uid,
            name: partner.exterior.name,
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PartnerListRow {
    pub id: i32,
    pub uid: Uuid,
    pub name: String,
    pub manager_name: String,
    pub manager_email: String,
    pub joined: DateTime<FixedOffset>,
    pub published: i64,
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
    pub fn person_has_permission(&self, uid: &Uuid) -> bool {
        if uid == &self.interior.prime {
            return true;
        }

        if self.interior.authorized.iter().any(|x| x == uid) {
            return true;
        }

        false
    }

    pub fn set_authorized(&mut self, uid: Uuid) {
        self.set_deauthorized(uid);
        self.interior.authorized.push(uid);
    }

    pub fn set_pending(&mut self, uid: Uuid) {
        self.set_deauthorized(uid);
        self.interior.pending.push(uid);
    }

    pub fn set_deauthorized(&mut self, uid: Uuid) {
        self.interior.pending.retain(|&x| x != uid);
        self.interior.authorized.retain(|&x| x != uid);
    }

    pub fn elide(mut self) -> Partner {
        self.id = None;
        self.interior.secret = None;

        self
    }

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

    pub async fn log_error(
        &self,
        db: &Database,
        level: LoggedErrorLevel,
        message: impl AsRef<str>,
        title: Option<impl AsRef<str>>,
        raw: Option<impl AsRef<str>>,
    ) -> Result<i64, Error> {
        let Some(partner_id) = self.id else {
            return Err(Error::Missing("id".into()));
        };

        let title = if let Some(val) = &title {
            Some(val.as_ref())
        } else {
            None
        };

        let raw = if let Some(val) = &raw {
            Some(val.as_ref())
        } else {
            None
        };

        Ok(sqlx::query_scalar!(
            r#"
INSERT
  INTO c_partner_error_log (partner_id, "level", "message", "title", "raw")
  VALUES ($1, $2, $3, $4, $5)
  RETURNING id
"#,
            partner_id,
            i16::from(level),
            message.as_ref(),
            title,
            raw,
        )
        .fetch_one(db)
        .await?)
    }

    pub async fn find_by_name(db: &Database, name: &str) -> Result<Vec<PartnerReference>, Error> {
        Ok(sqlx::query!(r#"SELECT id, (exterior -> 'uid') AS "uid", (exterior -> 'name') AS "name" FROM c_partner WHERE exterior -> 'uid' != $2 AND exterior ->> 'name' ILIKE $1"#, format!("%{name}%"), serde_json::to_value(*INTERNAL_UID)?)
            .map(|row| -> Result<PartnerReference, Error> {
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
            // remove this flatten() and the top-level Ok() to change
            // the semantics to fail if any row fails, rather than
            // ignoring failing rows
            .flatten()
            .collect())
    }

    pub async fn catalog(db: &Database) -> Result<Vec<PartnerReference>, Error> {
        Ok(sqlx::query_file!("db/partner/catalog.sql")
            .map(|row| -> Result<PartnerReference, Error> {
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
            .flatten()
            .collect())
    }

    pub async fn catalog_extra(db: &Database) -> Result<Vec<PartnerListRow>, Error> {
        Ok(sqlx::query_file!("db/partner/catalog_extra.sql")
            .map(|row| -> Result<PartnerListRow, Error> {
                Ok(PartnerListRow {
                    id: row.id,
                    uid: serde_json::from_value(
                        row.uid.ok_or_else(|| Error::Missing("uid".to_string()))?,
                    )?,
                    name: row.name.ok_or_else(|| Error::Missing("name".to_string()))?,
                    manager_name: row
                        .manager_name
                        .ok_or_else(|| Error::Missing("manager_name".to_string()))?,
                    manager_email: row
                        .manager_email
                        .ok_or_else(|| Error::Missing("manager_email".to_string()))?,
                    joined: row.joined.into(),
                    published: row
                        .published
                        .ok_or_else(|| Error::Missing("published".to_string()))?,
                })
            })
            .fetch_all(db)
            .await?
            .into_iter()
            .flatten()
            .collect())
    }

    pub async fn load_authorized_persons(
        &self,
        db: &Database,
    ) -> Result<Vec<Result<Person, Error>>, Error> {
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

    pub async fn load_pending_persons(
        &self,
        db: &Database,
    ) -> Result<Vec<Result<Person, Error>>, Error> {
        let persons = self.interior.pending.clone();

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

    pub async fn load_opportunities(
        &self,
        db: &Database,
    ) -> Result<Vec<Result<Opportunity, Error>>, Error> {
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

    pub async fn count_total_opportunities(&self, db: &Database) -> Result<u32, Error> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!: i64"
            FROM c_opportunity
            WHERE exterior -> 'partner' @> $1::jsonb
            "#,
            serde_json::to_value(self.exterior.uid)?
        )
        .fetch_one(db)
        .await?
        .count
        .try_into()
        .unwrap_or(0))
    }

    pub async fn count_current_opportunities(&self, db: &Database) -> Result<u32, Error> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!: i64"
            FROM c_opportunity
            WHERE exterior -> 'partner' @> $1::jsonb
            AND c_opportunity_is_current(interior, exterior) = true
            "#,
            serde_json::to_value(self.exterior.uid)?
        )
        .fetch_one(db)
        .await?
        .count
        .try_into()
        .unwrap_or(0))
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

    pub async fn load_by_id(db: &Database, id: i32) -> Result<Partner, Error> {
        let rec = sqlx::query_file!("db/partner/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Partner {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid(db: &Database, uid: &Uuid) -> Result<Partner, Error> {
        let rec = sqlx::query_file!("db/partner/get_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(Partner {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_uid(db: &Database, uid: &Uuid) -> Result<bool, Error> {
        let rec = sqlx::query_file!("db/partner/exists_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn store(&mut self, db: &Database) -> Result<(), Error> {
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
