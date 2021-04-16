use super::Error;

use serde::{Deserialize, Serialize};
use sqlx;
use std::collections::{HashMap, HashSet};
use time::OffsetDateTime;
use uuid::Uuid;

use super::PARTNER_NAMESPACE;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrganizationType {
    MuseumOrScienceCenter,
    Festival,
    Library,
    CollegeUniversity,
    #[serde(rename = "pk12school")]
    PK12School,
    CommunityOrganization,
    Club,
    Zoo,
    Aquarium,
    Planetarium,
    BotanicalGarden,
    ParksAndRec,
    HistoricalSite,
    MakerOrganization,
    Company,
    GovtAgency,
    #[serde(other)]
    Unspecified,
}

impl Default for OrganizationType {
    fn default() -> Self {
        OrganizationType::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Opportunity,
    Attraction,
    #[serde(other)]
    Unspecified,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Opportunity
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VenueType {
    MuseumOrScienceCenter,
    Library,
    #[serde(rename = "pk12school")]
    PK12School,
    CommunityOrganization,
    Bar,
    Outdoors,
    CollegeUniversity,
    #[serde(other)]
    Unspecified,
}

impl Default for VenueType {
    fn default() -> Self {
        VenueType::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    CitizenScience,
    LiveScience,
    Maker,
    Policy,
    OutOfSchoolTimeProgram,
    #[serde(other)]
    Unspecified,
}

impl Default for Domain {
    fn default() -> Self {
        Domain::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Descriptor {
    AdvocacyDays,
    Bioblitz,
    Camp,
    CitizenScience,
    CleanUp,
    Club,
    Community,
    Competition,
    Concert,
    Conference,
    #[serde(rename = "create-a-thon")]
    Createathon,
    Dance,
    Exhibition,
    ExpoStyle,
    Festival,
    Forum,
    Fundraising,
    #[serde(rename = "hack-a-thon")]
    Hackathon,
    Lecture,
    LiveScience,
    #[serde(rename = "make-a-thon")]
    Makeathon,
    Maker,
    MakerFaire,
    Media,
    Outreach,
    Overnight,
    Panel,
    Policy,
    ProfessionalDevelopment,
    Research,
    ScienceBlogging,
    ScienceCafeOrPub,
    ScienceOnTap,
    SciencePoetrySlam,
    ScienceSlam,
    Service,
    StarParty,
    StoryCollider,
    Tinker,
    TinkerFaire,
    Training,
    Volunteering,
    Workshop,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Topic {
    Agriculture,
    Alcohol,
    Animals,
    ArchaeologyAndCultural,
    Art,
    AstronomyAndSpace,
    Awards,
    Biology,
    Birds,
    Chemistry,
    ClimateAndWeather,
    ComputersAndTechnology,
    CrowdFunding,
    Design,
    DisasterResponse,
    EcologyAndEnvironment,
    Education,
    Engineering,
    Food,
    Geography,
    GeologyAndEarthScience,
    HealthAndMedicine,
    InsectsAndPollinators,
    Mathematics,
    NatureAndOutdoors,
    OceanWaterMarine,
    Paleontology,
    Physics,
    Policy,
    Psychology,
    Religion,
    Robotics,
    SocialScience,
    Sound,
    Technology,
    Transportation,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenHours {
    pub opens: String,
    pub closes: String,
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OpenDays {
    pub monday: Option<OpenHours>,
    pub tuesday: Option<OpenHours>,
    pub wednesday: Option<OpenHours>,
    pub thursday: Option<OpenHours>,
    pub friday: Option<OpenHours>,
    pub saturday: Option<OpenHours>,
    pub sunday: Option<OpenHours>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Cost {
    Free,
    Cost,
    #[serde(other)]
    Unknown,
}

impl Default for Cost {
    fn default() -> Self {
        Cost::Free
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    #[serde(alias = "ANY")]
    #[serde(alias = "Any")]
    Any,
    #[serde(alias = "AT")]
    #[serde(alias = "At")]
    At,
    #[serde(alias = "NEAR")]
    #[serde(alias = "Near")]
    Near,
    #[serde(alias = "UNKNOWN")]
    #[serde(alias = "Unknown")]
    #[serde(other)]
    Unknown,
}

impl Default for LocationType {
    fn default() -> Self {
        LocationType::Any
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OpportunityExterior {
    pub uid: Uuid,
    pub partner_name: String,
    pub partner_created: Option<OffsetDateTime>,
    pub partner_updated: Option<OffsetDateTime>,
    pub partner_opp_url: String,
    pub organization_name: String,
    pub organization_type: OrganizationType,
    pub organization_website: String,
    pub entity_type: EntityType,
    pub min_age: i16,
    pub max_age: i16,
    pub pes_domain: Domain,
    pub tags: HashSet<String>,
    pub ticket_required: bool,
    pub title: String,
    pub description: String,
    pub image_url: String,
    pub start_dates: Vec<OffsetDateTime>,
    pub has_end: bool,
    pub end_dates: Vec<OffsetDateTime>,
    pub attraction_hours: Option<OpenDays>,
    pub cost: Cost,
    pub languages: Vec<String>,
    pub is_online: bool,
    pub location_type: LocationType,
    pub location_name: String,
    pub location_point: Option<serde_json::Value>,
    pub location_polygon: Option<serde_json::Value>,
    pub address_street: String,
    pub address_city: String,
    pub address_state: String,
    pub address_country: String,
    pub address_zip: String,
    pub opp_hashtags: Vec<String>,
    pub opp_social_handles: HashMap<String, String>,
    pub partner: Uuid, // uid of the Partner entry which controls this entry
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct OpportunityInterior {
    pub withdrawn: bool,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub extra_data: serde_json::Value,
}

impl Default for OpportunityInterior {
    fn default() -> Self {
        OpportunityInterior {
            withdrawn: false,
            contact_name: Default::default(),
            contact_email: Default::default(),
            contact_phone: Default::default(),
            extra_data: serde_json::json!({}),
        }
    }
}

#[derive(Default, Debug, Serialize, Deserialize)]
#[serde(default)]
pub struct Opportunity {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: OpportunityExterior,
    #[serde(flatten)]
    pub interior: OpportunityInterior,
}

impl Opportunity {
    pub async fn load_partner<'req, DB>(&self, db: DB) -> Result<super::partner::Partner, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        Ok(super::partner::Partner::load_by_uid(db, &self.exterior.partner).await?)
    }

    pub fn validate(&mut self) -> Result<(), Error> {
        self.exterior.partner_name = self
            .exterior
            .partner_name
            .trim_matches(char::is_whitespace)
            .into();

        self.exterior.title = self.exterior.title.trim_matches(char::is_whitespace).into();

        if self.exterior.partner_name.is_empty() {
            return Err(Error::Missing("partner_name".into()));
        }

        if self.exterior.title.is_empty() {
            return Err(Error::Missing("title".into()));
        }

        if self.exterior.uid.is_nil() {
            let namespace = Uuid::new_v5(&PARTNER_NAMESPACE, self.exterior.partner_name.as_ref());
            self.exterior.uid = Uuid::new_v5(&namespace, self.exterior.title.as_ref());
        }

        Ok(())
    }

    pub async fn load_by_id<'req, DB>(db: DB, id: i32) -> Result<Opportunity, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/opportunity/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid<'req, DB>(db: DB, uid: &Uuid) -> Result<Opportunity, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!("db/opportunity/get_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn exists_by_uid<'req, DB>(db: DB, uid: &Uuid) -> Result<bool, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let rec = sqlx::query_file!(
            "db/opportunity/exists_by_uid.sql",
            serde_json::to_value(uid)?
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
                "db/opportunity/update.sql",
                id,
                serde_json::to_value(&self.exterior)?,
                serde_json::to_value(&self.interior)?,
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/opportunity/insert.sql",
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
