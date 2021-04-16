use super::{partner::Partner, Error};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx;
use time::OffsetDateTime;
use uuid::Uuid;

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum Permission {
    All,
    ManageOpportunities,
    ManagePartners,
    ManagePersons,
}

impl Permission {
    pub fn grants(grantor: &Permission, grantee: &Permission) -> bool {
        match (grantor, grantee) {
            (Permission::All, _) => true,
            (Permission::ManageOpportunities, Permission::ManageOpportunities) => true,
            (Permission::ManagePartners, Permission::ManagePartners) => true,
            (Permission::ManagePersons, Permission::ManagePersons) => true,
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
    pub joined_at: OffsetDateTime,
    pub active_at: OffsetDateTime,
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
            joined_at: OffsetDateTime::now_utc(),
            active_at: OffsetDateTime::now_utc(),
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
    pub fn set_password(&mut self, password: &str) {
        self.interior.password = Some(djangohashers::make_password(password));
    }

    pub async fn load_partners<'req, DB>(
        &self,
        db: DB,
    ) -> Result<Vec<Result<Partner, Error>>, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
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

    pub async fn note_activity<'req, DB>(&mut self, db: DB) -> Result<(), Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        self.interior.active_at = OffsetDateTime::now_utc();
        self.store(db).await
    }

    pub async fn load_by_id<'req, DB>(db: DB, id: i32) -> Result<Person, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/person/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Person {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid<'req, DB>(db: DB, uid: &Uuid) -> Result<Person, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/person/get_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(Person {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_uid<'req, DB>(db: DB, uid: &Uuid) -> Result<bool, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/person/exists_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn load_by_email<'req, DB>(db: DB, email: &str) -> Result<Person, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
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

    pub async fn exists_by_email<'req, DB>(db: DB, email: &str) -> Result<bool, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!(
            "db/person/exists_by_email.sql",
            serde_json::to_value(normalize_email(email))?
        )
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
