use std::fmt::Display;

use super::{
    opportunity::{Domain, Opportunity, OrganizationType},
    person::Person,
    Error, PARTNER_NAMESPACE,
};
use crate::{Database, ToFixedOffset, INTERNAL_UID};

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx;
use uuid::Uuid;

#[derive(Default, Debug, Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
pub enum LoggedErrorLevel {
    Info,
    #[default]
    Debug,
    Warning,
    Error,
    TerminateProcessing,
}

impl TryFrom<i16> for LoggedErrorLevel {
    type Error = Error;

    fn try_from(value: i16) -> Result<Self, <LoggedErrorLevel as TryFrom<i16>>::Error> {
        match value {
            -1 => Ok(LoggedErrorLevel::Info),
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
            LoggedErrorLevel::Info => -1,
            LoggedErrorLevel::Debug => 0,
            LoggedErrorLevel::Warning => 1,
            LoggedErrorLevel::Error => 2,
            LoggedErrorLevel::TerminateProcessing => {
                panic!("TerminateProcessing should never be saved or restored.")
            }
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

#[derive(Debug, Serialize)]
pub struct PartnerReportRow {
    pub name: String,
    pub contact_name: String,
    pub contact_email: String,
    pub joined: DateTime<FixedOffset>,
    pub total_opportunities: i64,
    pub current_opportunities: i64,
    pub current_opportunities_one_month_ago: i64,
    pub most_recent_opportunity: Option<DateTime<FixedOffset>>,
}

#[derive(Debug, Serialize)]
pub struct ExchangeReportRow {
    pub name: String,
    pub contact_name: String,
    pub contact_email: String,
    pub total_opportunities: i64,
    pub current_opportunities: i64,
    pub current_opportunities_one_month_ago: i64,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct Partner {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: PartnerExterior,
    #[serde(flatten)]
    pub interior: PartnerInterior,
}

pub fn partner_from_row(
    id: i32,
    uid: Uuid,
    name: String,
    organization_type: String,
    pes_domain: String,
    url: Option<String>,
    image_url: Option<String>,
    description: String,
    background_color: Option<String>,
    primary_color: Option<String>,
    secondary_color: Option<String>,
    tertiary_color: Option<String>,
    under: Option<Uuid>,
    open_submission: Option<bool>,
    default_query: Option<String>,
    manager: serde_json::Value,
    contact: Option<serde_json::Value>,
    prime: Uuid,
    authorized: Vec<Uuid>,
    pending: Vec<Uuid>,
    secret: Option<String>,
) -> Result<Partner, Error> {
    Ok(Partner {
        id: Some(id),
        exterior: PartnerExterior {
            uid,
            name,
            organization_type: serde_json::from_value(
                serde_json::Value::String(organization_type),
            )
            .unwrap_or_default(),
            pes_domain: serde_json::from_value(serde_json::Value::String(pes_domain))
                .unwrap_or_default(),
            url,
            image_url,
            description,
            background_color,
            primary_color,
            secondary_color,
            tertiary_color,
            under,
            open_submission,
            default_query,
        },
        interior: PartnerInterior {
            manager: serde_json::from_value(manager).unwrap_or_default(),
            contact: contact.and_then(|v| serde_json::from_value(v).ok()),
            prime,
            authorized,
            pending,
            secret,
        },
    })
}

pub fn serialize_enum<T: Serialize>(val: &T) -> String {
    serde_json::to_string(val)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
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
        Ok(sqlx::query!(
            r#"SELECT id, uid, "name" FROM c_partner WHERE uid != $2 AND "name" ILIKE $1"#,
            format!("%{name}%"),
            *INTERNAL_UID,
        )
        .map(|row| PartnerReference {
            id: row.id,
            uid: row.uid,
            name: row.name,
        })
        .fetch_all(db)
        .await?)
    }

    pub async fn catalog(db: &Database) -> Result<Vec<PartnerReference>, Error> {
        Ok(sqlx::query_file!("db/partner/catalog.sql")
            .map(|row| PartnerReference {
                id: row.id,
                uid: row.uid,
                name: row.name,
            })
            .fetch_all(db)
            .await?)
    }

    pub async fn catalog_extra(db: &Database) -> Result<Vec<PartnerListRow>, Error> {
        Ok(sqlx::query_file!("db/partner/catalog_extra.sql")
            .map(|row| -> Result<PartnerListRow, Error> {
                Ok(PartnerListRow {
                    id: row.id,
                    uid: row.uid,
                    name: row.name,
                    manager_name: row
                        .manager_name
                        .ok_or_else(|| Error::Missing("manager_name".to_string()))?,
                    manager_email: row
                        .manager_email
                        .ok_or_else(|| Error::Missing("manager_email".to_string()))?,
                    joined: row.joined.to_fixed_offset(),
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

    pub async fn report(db: &Database) -> Result<Vec<PartnerReportRow>, Error> {
        Ok(sqlx::query_file!("db/partner/report.sql")
            .map(|row| -> Result<PartnerReportRow, Error> {
                Ok(PartnerReportRow {
                    name: row.name,
                    contact_name: row
                        .contact_name
                        .ok_or_else(|| Error::Missing("contact_name".to_string()))?,
                    contact_email: row
                        .contact_email
                        .ok_or_else(|| Error::Missing("contact_email".to_string()))?,
                    joined: row.joined.to_fixed_offset(),
                    total_opportunities: row
                        .total_opportunities
                        .ok_or_else(|| Error::Missing("total_opportunities".to_string()))?,
                    current_opportunities: row
                        .current_opportunities
                        .ok_or_else(|| Error::Missing("current_opportunities".to_string()))?,
                    current_opportunities_one_month_ago: row
                        .current_opportunities_one_month_ago
                        .ok_or_else(|| Error::Missing("current_opportunities_one_month_ago".to_string()))?,
                    most_recent_opportunity: row.most_recent_opportunity.map(|d| d.to_fixed_offset()),
                })
            })
            .fetch_all(db)
            .await?
            .into_iter()
            .flatten()
            .collect())
    }

    pub async fn exchanges(db: &Database) -> Result<Vec<ExchangeReportRow>, Error> {
        Ok(sqlx::query_file!("db/partner/exchanges.sql")
            .map(|row| -> Result<ExchangeReportRow, Error> {
                Ok(ExchangeReportRow {
                    name: row.name,
                    contact_name: row
                        .contact_name
                        .ok_or_else(|| Error::Missing("contact_name".to_string()))?,
                    contact_email: row
                        .contact_email
                        .ok_or_else(|| Error::Missing("contact_email".to_string()))?,
                    total_opportunities: row
                        .total_opportunities
                        .ok_or_else(|| Error::Missing("total_opportunities".to_string()))?,
                    current_opportunities: row
                        .current_opportunities
                        .ok_or_else(|| Error::Missing("current_opportunities".to_string()))?,
                    current_opportunities_one_month_ago: row
                        .current_opportunities_one_month_ago
                        .ok_or_else(|| Error::Missing("current_opportunities_one_month_ago".to_string()))?,
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
            &persons,
        )
        .map(|row| {
            super::person::person_from_row(
                row.id, row.uid, row.username, row.person_image_url,
                row.email, row.email_hashes, row.password,
                row.join_channel, row.join_channel_detail,
                row.first_name, row.last_name, row.genders, row.gender_other,
                row.joined_at, row.active_at, row.phone, row.whatsapp,
                row.zip_code, row.birth_year, row.ethnicities, row.ethnicity_other,
                row.family_income, row.education_level, row.opt_in_research, row.opt_in_volunteer,
                row.permissions, row.private, row.newsletter, row.allow_emails,
                row.recent_point, row.last_used_people_recruiter, row.extra,
            )
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
            &persons,
        )
        .map(|row| {
            super::person::person_from_row(
                row.id, row.uid, row.username, row.person_image_url,
                row.email, row.email_hashes, row.password,
                row.join_channel, row.join_channel_detail,
                row.first_name, row.last_name, row.genders, row.gender_other,
                row.joined_at, row.active_at, row.phone, row.whatsapp,
                row.zip_code, row.birth_year, row.ethnicities, row.ethnicity_other,
                row.family_income, row.education_level, row.opt_in_research, row.opt_in_volunteer,
                row.permissions, row.private, row.newsletter, row.allow_emails,
                row.recent_point, row.last_used_people_recruiter, row.extra,
            )
        })
        .fetch_all(db)
        .await?)
    }

    pub async fn load_opportunities(
        &self,
        db: &Database,
    ) -> Result<Vec<Result<Opportunity, Error>>, Error> {
        use super::opportunity::opportunity_from_row;
        Ok(sqlx::query_file!(
            "db/partner/fetch_opportunities.sql",
            self.exterior.uid
        )
        .map(|row| {
            opportunity_from_row(
                row.id, row.uid, row.slug, row.partner_name, row.partner_website, row.partner_logo_url,
                row.partner_created, row.partner_updated, row.partner_opp_url,
                row.organization_name, row.organization_type, row.organization_website, row.organization_logo_url,
                row.entity_type, row.opp_venue, row.opp_descriptor, row.min_age, row.max_age, row.pes_domain,
                row.tags, row.opp_topics, row.ticket_required,
                row.title, row.description, row.short_desc, row.image_url, row.image_credit,
                row.start_datetimes, row.has_end, row.end_datetimes, row.recurrence, row.end_recurrence, row.timezone,
                row.attraction_hours, row.cost, row.languages, row.is_online,
                row.location_type, row.location_name, row.location_point_geojson, row.location_polygon_geojson,
                row.address_street, row.address_city, row.address_state, row.address_country, row.address_zip,
                row.opp_hashtags, row.opp_social_handles, row.opp_partner,
                row.accepted, row.withdrawn, row.submitted_by, row.review_status,
                row.contact_name, row.contact_email, row.contact_phone, row.extra_data,
            )
        })
        .fetch_all(db)
        .await?)
    }

    pub async fn count_total_opportunities(&self, db: &Database) -> Result<u32, Error> {
        Ok(sqlx::query!(
            r#"
            SELECT COUNT(*) AS "count!: i64"
            FROM c_opportunity
            WHERE opp_partner = $1
            "#,
            self.exterior.uid
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
            WHERE opp_partner = $1
            AND c_opportunity_is_current(c_opportunity) = true
            "#,
            self.exterior.uid
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

        partner_from_row(
            rec.id, rec.uid, rec.name, rec.organization_type, rec.pes_domain,
            rec.url, rec.image_url, rec.description,
            rec.background_color, rec.primary_color, rec.secondary_color, rec.tertiary_color,
            rec.under, rec.open_submission, rec.default_query,
            rec.manager, rec.contact, rec.prime, rec.authorized, rec.pending, rec.secret,
        )
    }

    pub async fn load_by_uid(db: &Database, uid: &Uuid) -> Result<Partner, Error> {
        let rec = sqlx::query_file!("db/partner/get_by_uid.sql", *uid)
            .fetch_one(db)
            .await?;

        partner_from_row(
            rec.id, rec.uid, rec.name, rec.organization_type, rec.pes_domain,
            rec.url, rec.image_url, rec.description,
            rec.background_color, rec.primary_color, rec.secondary_color, rec.tertiary_color,
            rec.under, rec.open_submission, rec.default_query,
            rec.manager, rec.contact, rec.prime, rec.authorized, rec.pending, rec.secret,
        )
    }

    pub async fn exists_by_uid(db: &Database, uid: &Uuid) -> Result<bool, Error> {
        let rec = sqlx::query_file!("db/partner/exists_by_uid.sql", *uid)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn store(&mut self, db: &Database) -> Result<(), Error> {
        self.validate()?;

        let org_type = serialize_enum(&self.exterior.organization_type);
        let domain = serialize_enum(&self.exterior.pes_domain);
        let manager_json = serde_json::to_value(&self.interior.manager)?;
        let contact_json = self
            .interior
            .contact
            .as_ref()
            .map(|c| serde_json::to_value(c))
            .transpose()?;

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/partner/update.sql",
                id,
                self.exterior.uid,
                self.exterior.name,
                org_type,
                domain,
                self.exterior.url.as_deref(),
                self.exterior.image_url.as_deref(),
                self.exterior.description,
                self.exterior.background_color.as_deref(),
                self.exterior.primary_color.as_deref(),
                self.exterior.secondary_color.as_deref(),
                self.exterior.tertiary_color.as_deref(),
                self.exterior.under as Option<Uuid>,
                self.exterior.open_submission,
                self.exterior.default_query.as_deref(),
                manager_json,
                contact_json as Option<serde_json::Value>,
                self.interior.prime,
                &self.interior.authorized,
                &self.interior.pending,
                self.interior.secret.as_deref(),
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/partner/insert.sql",
                self.exterior.uid,
                self.exterior.name,
                org_type,
                domain,
                self.exterior.url.as_deref(),
                self.exterior.image_url.as_deref(),
                self.exterior.description,
                self.exterior.background_color.as_deref(),
                self.exterior.primary_color.as_deref(),
                self.exterior.secondary_color.as_deref(),
                self.exterior.tertiary_color.as_deref(),
                self.exterior.under as Option<Uuid>,
                self.exterior.open_submission,
                self.exterior.default_query.as_deref(),
                manager_json,
                contact_json as Option<serde_json::Value>,
                self.interior.prime,
                &self.interior.authorized,
                &self.interior.pending,
                self.interior.secret.as_deref(),
            )
            .fetch_one(db)
            .await?;

            self.id = Some(rec.id);
        };

        Ok(())
    }
}
