use super::{partner::Partner, Error, Pagination};
use crate::{Database, ToFixedOffset};

use chrono::{DateTime, FixedOffset, TimeZone, Utc};
use futures::Stream;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::prelude::*;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(sqlx::Type)]
#[sqlx(type_name = "c_person_goals_status", rename_all = "snake_case")]
pub enum GoalStatus {
    Canceled,
    Failed,
    Working,
    Succeeded,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
#[serde(bound = "")] // The derive macro assumes an unecessary trait bound
pub struct Goal<Offset>
where
    Offset: TimeZone,
    DateTime<Offset>: DeserializeOwned,
{
    #[serde(default)]
    pub id: i32,
    #[serde(skip)]
    pub person_id: i32,
    pub category: String,
    pub progress: i32,
    pub target: i32,
    pub begin: DateTime<Offset>,
    pub end: DateTime<Offset>,
    pub status: GoalStatus,
}

impl<Offset> Goal<Offset>
where
    Offset: TimeZone,
    DateTime<Offset>: DeserializeOwned + ToFixedOffset,
{
    pub fn into_fixed_offset(self) -> Goal<FixedOffset> {
        Goal::<FixedOffset> {
            id: self.id,
            person_id: self.person_id,
            category: self.category,
            progress: self.progress,
            target: self.target,
            begin: self.begin.to_fixed_offset(),
            end: self.end.to_fixed_offset(),
            status: self.status,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    #[serde(other)]
    Unspecified,
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EducationLevel {
    GradeSchool,
    HighSchool,
    TradeSchool,
    University,
    Graduate,
    Doctorate,
    #[serde(other)]
    Unspecified,
}

impl Default for EducationLevel {
    fn default() -> Self {
        EducationLevel::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum IncomeLevel {
    Poor,
    LowerMiddleClass,
    MiddleClass,
    UpperMiddleClass,
    Wealthy,
    #[serde(other)]
    Unspecified,
}

impl Default for IncomeLevel {
    fn default() -> Self {
        IncomeLevel::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    All,
    ManageOpportunities,
    ManagePartners,
    ManagePersons,
    ManageContent,
    ManageSomething,
}

impl Permission {
    pub fn grants(grantor: &Permission, grantee: &Permission) -> bool {
        if grantor == grantee {
            return true;
        }

        match (grantor, grantee) {
            (Permission::All, _) => true,
            (Permission::ManageOpportunities, Permission::ManageSomething) => true,
            (Permission::ManagePartners, Permission::ManageSomething) => true,
            (Permission::ManagePersons, Permission::ManageSomething) => true,
            (Permission::ManageContent, Permission::ManageSomething) => true,

            _ => false,
        }
    }

    pub fn check(assigned: &Vec<Permission>, requested: &Permission) -> bool {
        for perm in assigned {
            if Permission::grants(perm, requested) {
                return true;
            }
        }

        false
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct PersonExterior {
    pub uid: Uuid,
    pub username: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonInterior {
    pub email: String,
    pub email_hashes: Vec<String>,
    pub password: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub gender: Gender,
    pub home_location: Option<serde_json::Value>,
    pub last_location: Option<serde_json::Value>,
    pub joined_at: DateTime<FixedOffset>,
    pub active_at: DateTime<FixedOffset>,
    pub phone: Option<String>,
    pub whatsapp: Option<String>,
    pub zip_code: Option<String>,
    pub birth_year: Option<u32>,
    pub race: Option<String>,
    pub ethnicity: Option<String>,
    pub family_income: Option<IncomeLevel>,
    pub education_level: Option<EducationLevel>,
    pub opt_in_research: Option<bool>,
    pub opt_in_volunteer: Option<bool>,
    pub permissions: Vec<Permission>,
}

impl Default for PersonInterior {
    fn default() -> Self {
        PersonInterior {
            email: Default::default(),
            email_hashes: Default::default(),
            first_name: Default::default(),
            last_name: Default::default(),
            password: Default::default(),
            gender: Default::default(),
            home_location: Default::default(),
            last_location: Default::default(),
            joined_at: Utc::now().to_fixed_offset(),
            active_at: Utc::now().to_fixed_offset(),
            phone: Default::default(),
            whatsapp: Default::default(),
            zip_code: Default::default(),
            birth_year: Default::default(),
            race: Default::default(),
            ethnicity: Default::default(),
            family_income: Default::default(),
            education_level: Default::default(),
            opt_in_research: Default::default(),
            opt_in_volunteer: Default::default(),
            permissions: Default::default(),
        }
    }
}

fn normalize_email(email: &str) -> String {
    email
        .trim_matches(char::is_whitespace)
        // We're going to ignore the fact that the local-part of the
        // email address is actually case sensitive. That is rarely
        // enforced anymore (the RFC recommends that it not be) and
        // people misunderstanding it has been a big problem for
        // SciStarter. We're using to_ascii_lowercase instead of
        // to_lowercase because non-ASCII email addresses do not have
        // the same recommendation.
        .to_ascii_lowercase()
        .into()
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Person {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: PersonExterior,
    #[serde(flatten)]
    pub interior: PersonInterior,
}

impl Person {
    pub async fn participation_between(
        &self,
        db: &Database,
        begin: &DateTime<FixedOffset>,
        end: &DateTime<FixedOffset>,
    ) -> Result<i32, Error> {
        Ok(sqlx::query_file!(
            "db/person/count_participation_between.sql",
            dbg!(self.exterior.uid.to_string()),
            dbg!(begin.to_rfc3339()),
            dbg!(end.to_rfc3339()),
        )
        .fetch_one(db)
        .await?
        .total as i32)
    }

    pub async fn goal_by_id(&self, db: &Database, id: i32) -> Result<Goal<Utc>, Error> {
        if let Some(person_id) = self.id {
            Ok(
                sqlx::query_file_as!(Goal, "db/person/fetch_goal_by_id.sql", person_id, id)
                    .fetch_one(db)
                    .await?,
            )
        } else {
            Err(Error::NoSuch("Person has no id"))
        }
    }

    pub async fn goals_by_status<'db>(
        &self,
        db: &'db Database,
        status: GoalStatus,
    ) -> Result<impl Stream<Item = Result<Goal<Utc>, sqlx::Error>> + 'db, Error> {
        if let Some(id) = self.id {
            Ok(sqlx::query_file_as!(
                Goal,
                "db/person/fetch_goals_by_status.sql",
                id,
                status as GoalStatus
            )
            .fetch(db))
        } else {
            Err(Error::NoSuch("Person has no id"))
        }
    }

    pub async fn add_goal(&self, db: &Database, goal: Goal<FixedOffset>) -> Result<i32, Error> {
        if let Some(id) = self.id {
            Ok(sqlx::query_file!(
                "db/person/add_goal.sql",
                id,
                goal.category,
                goal.target,
                goal.begin,
                goal.end,
                goal.status as GoalStatus
            )
            .fetch_one(db)
            .await?
            .id)
        } else {
            Err(Error::NoSuch("Person has no id"))
        }
    }

    pub async fn save_goal(&self, db: &Database, goal: Goal<FixedOffset>) -> Result<(), Error> {
        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/person/save_goal.sql",
                goal.id,
                id,
                goal.category,
                goal.target,
                goal.begin,
                goal.end,
                goal.status as GoalStatus
            )
            .execute(db)
            .await?;
            Ok(())
        } else {
            Err(Error::NoSuch("Person has no id"))
        }
    }

    pub async fn total(db: &Database) -> Result<u32, Error> {
        Ok(sqlx::query("SELECT COUNT(*) FROM c_person")
            .fetch_one(db)
            .await?
            .get::<i64, usize>(0) as u32)
    }

    pub async fn catalog(db: &Database, pagination: Pagination) -> Result<Vec<Person>, Error> {
        let (limit, offset) = match pagination {
            Pagination::All => (None, None),
            Pagination::One => (Some(1), None),
            Pagination::Page { index, size } => {
                (Some(size as i64), Some(index as i64 * size as i64))
            }
        };

        sqlx::query_file!("db/person/catalog.sql", limit, offset)
            .map(|rec| {
                Ok(Person {
                    id: Some(rec.id),
                    exterior: serde_json::from_value(rec.exterior)
                        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
                    interior: serde_json::from_value(rec.interior)
                        .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
                })
            })
            .fetch_all(db)
            .await?
            .into_iter()
            .collect()
    }

    pub fn check_permission(&self, perm: &Permission) -> bool {
        Permission::check(&self.interior.permissions, perm)
    }

    pub fn set_password(&mut self, password: &str) {
        self.interior.password = Some(djangohashers::make_password(password));
    }

    /// Checks whether the person has a password, and if so whether
    /// the password matches. The three possible outcomes are
    /// represented by the Option<bool> return type.
    pub fn check_password_full(&self, password: &str) -> Option<bool> {
        if let Some(hashed) = &self.interior.password {
            Some(djangohashers::check_password_tolerant(password, &hashed))
        } else {
            None
        }
    }

    /// Checks whether the password is the person's password. If the
    /// person has no password, returns false for any parameter value.
    pub fn check_password(&self, password: &str) -> bool {
        if let Some(valid) = self.check_password_full(password) {
            valid
        } else {
            false
        }
    }

    pub async fn count_partners(&self, db: &Database) -> Result<i32, Error> {
        Ok(sqlx::query_file!(
            "db/person/count_partners.sql",
            serde_json::to_value(self.exterior.uid)?
        )
        .map(|row| row.total)
        .fetch_one(db)
        .await?
        .unwrap_or(0) as i32)
    }

    pub async fn load_partners(&self, db: &Database) -> Result<Vec<Result<Partner, Error>>, Error> {
        Ok(sqlx::query_file!(
            "db/person/fetch_partners.sql",
            serde_json::to_value(self.exterior.uid)?
        )
        .map(|row| {
            Ok(Partner {
                id: Some(row.id),
                exterior: serde_json::from_value(row.exterior)?,
                interior: serde_json::from_value(row.interior)?,
            })
        })
        .fetch_all(db)
        .await?)
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        self.interior.email = normalize_email(&self.interior.email);

        if self.interior.email.is_empty() {
            return Err(Error::Missing("email".to_string()));
        }

        if self.exterior.uid.is_nil() {
            self.exterior.uid = Uuid::new_v4();
        }

        let mut hasher = Sha256::new();
        // Note, email is in UTF-8, has had whitespace trimmed, and
        // ascii characters have been reduced to lowercase.
        hasher.update(&self.interior.email);
        // Salt the hash with a common suffix, to move the hashes into
        // a distinct 'namespace' and prevent hashes computed for
        // other purposes from being used.
        hasher.update(b":science-link");
        // Lowercase hexidecimal representation
        let hashed = hex::encode(hasher.finalize());

        if !self.interior.email_hashes.iter().any(|x| x == &hashed) {
            self.interior.email_hashes.push(hashed);
        }

        Ok(())
    }

    pub async fn note_activity(&mut self, db: &Database) -> Result<(), Error> {
        self.interior.active_at = Utc::now().to_fixed_offset();
        self.store(db).await
    }

    pub async fn load_by_id(db: &Database, id: i32) -> Result<Person, Error> {
        let rec = sqlx::query_file!("db/person/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Person {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid(db: &Database, uid: &Uuid) -> Result<Person, Error> {
        let rec = sqlx::query_file!("db/person/get_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(Person {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_uid(db: &Database, uid: &Uuid) -> Result<bool, Error> {
        let rec = sqlx::query_file!("db/person/exists_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn load_by_email(db: &Database, email: &str) -> Result<Person, Error> {
        let rec = sqlx::query_file!(
            "db/person/get_by_email.sql",
            serde_json::to_value(normalize_email(email))?
        )
        .fetch_one(db)
        .await?;

        Ok(Person {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_email(db: &Database, email: &str) -> Result<bool, Error> {
        let rec = sqlx::query_file!(
            "db/person/exists_by_email.sql",
            serde_json::to_value(normalize_email(email))?
        )
        .fetch_one(db)
        .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn load_by_email_hash(db: &Database, hash: &str) -> Result<Person, Error> {
        let rec = sqlx::query_file!("db/person/get_by_email_hash.sql", hash)
            .fetch_one(db)
            .await?;

        Ok(Person {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_email_hash(db: &Database, hash: &str) -> Result<bool, Error> {
        let rec = sqlx::query_file!("db/person/exists_by_email_hash.sql", hash)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn all_by_email_hash(
        db: &Database,
        hash: &str,
    ) -> Result<Vec<Result<Person, Error>>, Error> {
        Ok(sqlx::query_file!("db/person/all_by_email_hash.sql", hash)
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

    pub async fn store(&mut self, db: &Database) -> Result<(), Error> {
        self.validate()?;

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/person/update.sql",
                id,
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/person/insert.sql",
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
