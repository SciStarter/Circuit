use super::Error;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use sqlx::prelude::*;
use std::collections::{HashMap, HashSet};
use std::convert::AsRef;
use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};
use uuid::Uuid;

use super::PARTNER_NAMESPACE;

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
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

impl super::SelectOption for OrganizationType {
    fn all_options() -> Vec<(String, String, OrganizationType)> {
        OrganizationType::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, OrganizationType) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
}

impl Default for OrganizationType {
    fn default() -> Self {
        OrganizationType::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Unspecified,
    Attraction,
    #[serde(other)]
    Opportunity,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Opportunity
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum VenueType {
    Indoors,
    Outdoors,
    // Following variants are deprecated
    MuseumOrScienceCenter,
    Library,
    #[serde(rename = "pk12school")]
    PK12School,
    CommunityOrganization,
    Bar,
    CollegeUniversity,
    #[serde(other)]
    Unspecified,
}

impl super::SelectOption for VenueType {
    fn all_options() -> Vec<(String, String, VenueType)> {
        VenueType::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, VenueType) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
}

impl Default for VenueType {
    fn default() -> Self {
        VenueType::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
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

impl super::SelectOption for Domain {
    fn all_options() -> Vec<(String, String, Domain)> {
        Domain::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Domain) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
}

impl Default for Domain {
    fn default() -> Self {
        Domain::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
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

impl super::SelectOption for Descriptor {
    fn all_options() -> Vec<(String, String, Descriptor)> {
        Descriptor::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Descriptor) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
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
    GeneralScience,
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

impl super::SelectOption for Topic {
    fn all_options() -> Vec<(String, String, Topic)> {
        Topic::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Topic) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
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

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Cost {
    Free,
    Cost,
    #[serde(other)]
    Unknown,
}

impl super::SelectOption for Cost {
    fn all_options() -> Vec<(String, String, Cost)> {
        Cost::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Cost) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
}

impl Default for Cost {
    fn default() -> Self {
        Cost::Free
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq)]
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

impl super::SelectOption for LocationType {
    fn all_options() -> Vec<(String, String, LocationType)> {
        LocationType::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, LocationType) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (code.to_string(), name, *self)
    }
}

impl Default for LocationType {
    fn default() -> Self {
        LocationType::Any
    }
}

fn zero() -> i16 {
    0
}

fn nineninetynine() -> i16 {
    999
}

fn en_us() -> Vec<String> {
    vec!["en-US".to_string()]
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct OpportunityExterior {
    pub uid: Uuid,
    pub partner_name: String,
    pub partner_website: Option<String>,
    pub partner_logo_url: Option<String>,
    pub partner_created: Option<DateTime<FixedOffset>>,
    pub partner_updated: Option<DateTime<FixedOffset>>,
    pub partner_opp_url: String,
    pub organization_name: String,
    pub organization_type: OrganizationType,
    pub organization_website: Option<String>,
    pub organization_logo_url: Option<String>,
    pub entity_type: EntityType,
    pub opp_venue: Vec<VenueType>,
    pub opp_descriptor: Vec<Descriptor>,
    #[serde(default = "zero")]
    pub min_age: i16,
    #[serde(default = "nineninetynine")]
    pub max_age: i16,
    pub pes_domain: Domain,
    pub tags: HashSet<String>,
    pub opp_topics: Vec<Topic>,
    pub ticket_required: bool,
    pub title: String,
    pub description: String,
    pub short_desc: String,
    pub image_url: String,
    #[serde(alias = "start_dates")]
    pub start_datetimes: Vec<DateTime<FixedOffset>>,
    pub has_end: bool,
    #[serde(alias = "end_dates")]
    pub end_datetimes: Vec<DateTime<FixedOffset>>,
    pub attraction_hours: Option<OpenDays>,
    pub cost: Cost,
    #[serde(default = "en_us")]
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

impl std::fmt::Debug for OpportunityExterior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self)
                .unwrap_or_else(|_| "## JSON serialization failed".to_string())
        )
    }
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct OpportunityInterior {
    pub accepted: bool,
    pub withdrawn: bool,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub extra_data: serde_json::Value,
}

impl Default for OpportunityInterior {
    fn default() -> Self {
        OpportunityInterior {
            accepted: false,  // editors have accepted it for publication
            withdrawn: false, // partner has withdrawn it from publication
            contact_name: Default::default(),
            contact_email: Default::default(),
            contact_phone: Default::default(),
            extra_data: serde_json::json!({}),
        }
    }
}

impl std::fmt::Debug for OpportunityInterior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self)
                .unwrap_or_else(|_| "## JSON serialization failed".to_string())
        )
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct Opportunity {
    pub id: Option<i32>,
    #[serde(flatten)]
    pub exterior: OpportunityExterior,
    #[serde(flatten)]
    pub interior: OpportunityInterior,
}

impl std::fmt::Display for Opportunity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.exterior.title)
    }
}

impl std::fmt::Debug for Opportunity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            serde_json::to_string_pretty(self)
                .unwrap_or_else(|_| "## JSON serialization failed".to_string())
        )
    }
}

pub struct OpportunityReference {
    pub uid: Uuid,
    pub title: String,
}

impl std::fmt::Display for OpportunityReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

/// Each field represents one of the database fields by which
/// Opportunity queries can be narrowed. The default value does not
/// narrow the query at all, so to find all of the opportunities with
/// a particular string in the name, we could do something like:
/// ```
/// Opportunity::load_matching(db.acquire().await?, OpportunityQuery { title_contains: "hello".to_string(), ..Default::default() })
/// ```
#[derive(Default)]
pub struct OpportunityQuery {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub title_contains: Option<String>,
    pub partner: Option<Uuid>,
}

enum ParamValue {
    Bool(bool),
    String(String),
    Uuid(Uuid),
}

impl Opportunity {
    pub async fn load_matching<'req, DB>(
        db: DB,
        query: OpportunityQuery,
    ) -> Result<Vec<Opportunity>, Error>
    where
        DB: sqlx::Executor<'req, Database = sqlx::Postgres>,
    {
        let mut clauses = Vec::new();
        let mut params = Vec::new();

        if let Some(val) = query.accepted {
            params.push(ParamValue::Bool(val));
            clauses.push(format!(
                "(${}::jsonb) @> (interior -> 'accepted')",
                params.len()
            ));
        }

        if let Some(val) = query.withdrawn {
            params.push(ParamValue::Bool(val));
            clauses.push(format!(
                "(${}::jsonb) @> (interior -> 'withdrawn')",
                params.len()
            ));
        }

        if let Some(val) = query.title_contains {
            params.push(ParamValue::String(format!("%{}%", val)));
            clauses.push(format!("(exterior ->> 'title') ILIKE ${}", params.len()));
        }

        if let Some(val) = query.partner {
            params.push(ParamValue::Uuid(val));
            clauses.push(format!(
                "(${}::jsonb) @> (exterior -> 'partner')",
                params.len()
            ));
        }

        let mut query_string = "SELECT * FROM c_opportunity".to_string();

        if !clauses.is_empty() {
            query_string.push_str(" WHERE");
        }

        for clause in clauses.into_iter() {
            query_string.push(' ');
            query_string.push_str(&clause);
        }

        query_string.push_str(" ORDER BY (exterior ->> 'title');");

        let mut query_obj = sqlx::query(&query_string);

        for value in params.into_iter() {
            query_obj = match value {
                ParamValue::Bool(val) => query_obj.bind(val),
                ParamValue::String(val) => query_obj.bind(val),
                ParamValue::Uuid(val) => query_obj.bind(serde_json::to_value(val)?),
            };
        }

        query_obj
            .map(|rec| {
                Ok(Opportunity {
                    id: Some(rec.get("id")),
                    exterior: serde_json::from_value(rec.get("exterior"))?,
                    interior: serde_json::from_value(rec.get("interior"))?,
                })
            })
            .fetch_all(db)
            .await?
            .into_iter()
            .collect()
    }

    pub fn to_reference(&self) -> OpportunityReference {
        OpportunityReference {
            uid: self.exterior.uid.clone(),
            title: self.exterior.title.clone(),
        }
    }

    pub fn into_reference(self) -> OpportunityReference {
        OpportunityReference {
            uid: self.exterior.uid,
            title: self.exterior.title,
        }
    }

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

        self.exterior.partner_opp_url = self
            .exterior
            .partner_opp_url
            .trim_matches(char::is_whitespace)
            .into();

        self.exterior.title = self.exterior.title.trim_matches(char::is_whitespace).into();

        if self.exterior.partner_name.is_empty() {
            return Err(Error::Missing("partner_name".into()));
        }

        if self.exterior.title.is_empty() {
            return Err(Error::Missing("title".into()));
        }

        if self.exterior.partner_opp_url.is_empty() {
            return Err(Error::Missing("partner_opp_url".into()));
        }

        if self.exterior.uid.is_nil() {
            let namespace = Uuid::new_v5(&PARTNER_NAMESPACE, self.exterior.partner_name.as_ref());

            let mut identifier = self.exterior.partner_opp_url.to_string();
            identifier.push_str("||");
            identifier.push_str(&self.exterior.title);

            self.exterior.uid = Uuid::new_v5(&namespace, identifier.as_ref());
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
