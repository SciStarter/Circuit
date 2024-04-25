pub mod for_slug;

//use super::partner::LoggedErrorLevel;
use super::person::PermitAction;
use super::Error;
use crate::model::involvement;
use crate::{geo, Database, ToFixedOffset};

use chrono::{DateTime, Duration, FixedOffset, Utc};
//use deunicode::deunicode;
use inflections::Inflect;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::postgres::{PgArguments, PgHasArrayType, PgTypeInfo};
use sqlx::query::Query;
use sqlx::{prelude::*, Postgres};
use std::collections::{HashMap, HashSet, VecDeque};
use std::convert::AsRef;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};
use uuid::Uuid;

use super::{Pagination, PARTNER_NAMESPACE};

// This regular expression matches any sequence of characters that
// does not consist of letters, numbers, or the dash character. The
// slugify() function replaces these sequences with a single dash.
// Status as a letter or number is defined by Unicode, which means
// that text using non-Latin characters will be retained when slugified.
pub static SLUGIFY_REPLACE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^\pL\pN-]+").expect("Unable to compile SLUGIFY_REPLACE regex"));

#[derive(Debug, Default, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStatus {
    Draft,
    Pending,
    Reject,
    Publish,
    #[default]
    NotRequired,
}

impl ReviewStatus {
    pub fn public(&self) -> bool {
        match self {
            ReviewStatus::Draft => false,
            ReviewStatus::Pending => false,
            ReviewStatus::Reject => false,
            ReviewStatus::Publish => true,
            ReviewStatus::NotRequired => true,
        }
    }

    pub fn requires_manager(&self) -> bool {
        match self {
            ReviewStatus::Draft => false,
            ReviewStatus::Pending => false,
            ReviewStatus::Reject => true,
            ReviewStatus::Publish => true,
            ReviewStatus::NotRequired => true,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Recurrence {
    #[default]
    Once,
    Daily,
    Weekly,
}

impl Recurrence {
    pub fn delta(&self) -> Duration {
        use Recurrence::*;
        match self {
            Once => Duration::days(0),
            Daily => Duration::days(1),
            Weekly => Duration::days(7),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Review {
    pub id: i32,
    pub person: Uuid,
    pub username: Option<String>,
    pub image_url: Option<String>,
    pub rating: i16,
    pub comment: String,
    pub when: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct Reviews {
    pub average: f32,
    pub reviews: Vec<Review>,
}

#[derive(
    Debug, Default, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq,
)]
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
    MakerSpace,
    #[serde(other)]
    #[default]
    Unspecified,
}

impl super::SelectOption for OrganizationType {
    fn all_options() -> Vec<(String, String, OrganizationType)> {
        OrganizationType::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, OrganizationType) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
    }
}

#[derive(
    Debug, Default, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, PartialEq, Eq, Clone,
)]
#[serde(rename_all = "snake_case")]
// Don't forget to update impl super::SelectOption for EntityType and impl OpportunityForm to add options for new layout
// How can this be centralized so we don't have to repeat?
pub enum PageLayout {
    #[default]
    JustContent,
    AddOpportunities,
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PageOptions {
    pub layout: PageLayout,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Unspecified,
    Attraction,
    Page(PageOptions),
    #[serde(other)]
    #[default]
    Opportunity,
}

#[derive(sqlx::Type)]
#[sqlx(type_name = "t_entity_type", rename_all = "snake_case")]
enum DBEntityType {
    Unspecified,
    Attraction,
    Page,
    Opportunity,
}

impl PgHasArrayType for DBEntityType {
    fn array_type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("_t_entity_type")
    }
}

impl From<EntityType> for DBEntityType {
    fn from(value: EntityType) -> Self {
        match value {
            EntityType::Unspecified => DBEntityType::Unspecified,
            EntityType::Attraction => DBEntityType::Attraction,
            EntityType::Page(_) => DBEntityType::Page,
            EntityType::Opportunity => DBEntityType::Opportunity,
        }
    }
}

impl super::SelectOption for EntityType {
    fn all_options() -> Vec<(String, String, EntityType)> {
        vec![
            EntityType::Opportunity.to_option(),
            EntityType::Attraction.to_option(),
            EntityType::Page(PageOptions {
                layout: PageLayout::JustContent,
                ..Default::default()
            })
            .to_option(),
            EntityType::Page(PageOptions {
                layout: PageLayout::AddOpportunities,
                ..Default::default()
            })
            .to_option(),
            EntityType::Unspecified.to_option(),
        ]
    }

    fn to_option(&self) -> (String, String, EntityType) {
        match self {
            EntityType::Unspecified => (
                "unspecified".to_string(),
                "Unspecified".to_string(),
                EntityType::Unspecified,
            ),
            EntityType::Attraction => (
                "attraction".to_string(),
                "Attraction".to_string(),
                EntityType::Attraction,
            ),
            EntityType::Opportunity => (
                "opportunity".to_string(),
                "Opportunity".to_string(),
                EntityType::Opportunity,
            ),
            EntityType::Page(options) => match options.layout {
                PageLayout::JustContent => (
                    "page__just_content".to_string(),
                    "Page - Just Content".to_string(),
                    EntityType::Page(options.clone()),
                ),
                PageLayout::AddOpportunities => (
                    "page__add_opportunities".to_string(),
                    "Page - 'Add Opportunities' layout".to_string(),
                    EntityType::Page(options.clone()),
                ),
            },
        }
    }
}

#[derive(
    Debug,
    Default,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    AsRefStr,
    Copy,
    Clone,
    PartialEq,
    sqlx::Type,
)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "t_venue_type", rename_all = "snake_case")]
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
    #[default]
    Unspecified,
}

impl VenueType {
    fn db_repr(&self) -> String {
        self.as_ref().to_snake_case()
    }
}

impl super::SelectOption for VenueType {
    fn all_options() -> Vec<(String, String, VenueType)> {
        VenueType::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, VenueType) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
    }
}

#[derive(
    Debug, Default, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq,
)]
#[serde(rename_all = "snake_case")]
pub enum Domain {
    CitizenScience,
    LiveScience,
    MuseumOrScienceCenter,
    Maker,
    Policy,
    OutOfSchoolTimeProgram,
    FormalEducation,
    ScienceCommunications,
    #[serde(other)]
    #[default]
    Unspecified,
}

impl super::SelectOption for Domain {
    fn all_options() -> Vec<(String, String, Domain)> {
        Domain::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Domain) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
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

impl std::fmt::Display for Descriptor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        let len = json.len();
        write!(
            f,
            "{}",
            json.chars().skip(1).take(len - 2).collect::<String>()
        )
    }
}

impl super::SelectOption for Descriptor {
    fn all_options() -> Vec<(String, String, Descriptor)> {
        Descriptor::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Descriptor) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
    }
}

impl Descriptor {
    fn db_repr(&self) -> String {
        self.as_ref().to_snake_case()
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

impl std::fmt::Display for Topic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).map_err(|_| std::fmt::Error)?;
        let len = json.len();
        write!(
            f,
            "{}",
            json.chars().skip(1).take(len - 2).collect::<String>()
        )
    }
}

impl super::SelectOption for Topic {
    fn all_options() -> Vec<(String, String, Topic)> {
        Topic::iter()
            .filter(|x| x != &Topic::Alcohol)
            .map(|x| x.to_option())
            .collect()
    }

    fn to_option(&self) -> (String, String, Topic) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
    }
}

impl Topic {
    fn db_repr(&self) -> String {
        self.as_ref().to_snake_case()
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

#[derive(
    Debug, Default, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq,
)]
#[serde(rename_all = "snake_case")]
pub enum Cost {
    #[default]
    Free,
    Cost,
    #[serde(other)]
    Unknown,
}

impl Cost {
    fn db_repr(&self) -> String {
        self.as_ref().to_snake_case()
    }
}

impl super::SelectOption for Cost {
    fn all_options() -> Vec<(String, String, Cost)> {
        Cost::iter().map(|x| x.to_option()).collect()
    }

    fn to_option(&self) -> (String, String, Cost) {
        let code = self.as_ref();
        let name = super::separate_camel_case(code);

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
    }
}

#[derive(
    Debug, Default, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, Copy, Clone, PartialEq,
)]
#[serde(rename_all = "snake_case")]
pub enum LocationType {
    #[serde(alias = "ANY")]
    #[serde(alias = "Any")]
    #[default]
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

        (
            serde_json::to_string(&self)
                .expect("Serializing simple enum variants should never fail")
                .trim_matches('"')
                .to_owned(),
            name,
            *self,
        )
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
    pub slug: String,
    pub partner_name: String,
    pub partner_website: Option<String>,
    pub partner_logo_url: Option<String>,
    pub partner_created: Option<DateTime<FixedOffset>>,
    pub partner_updated: Option<DateTime<FixedOffset>>,
    pub partner_opp_url: Option<String>,
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
    pub image_credit: String,
    #[serde(alias = "start_dates")]
    pub start_datetimes: Vec<DateTime<FixedOffset>>,
    pub has_end: bool,
    #[serde(alias = "end_dates")]
    pub end_datetimes: Vec<DateTime<FixedOffset>>,
    pub recurrence: Recurrence,
    pub end_recurrence: Option<DateTime<FixedOffset>>,
    pub timezone: Option<String>,
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

impl OpportunityExterior {
    pub fn into_reference(self) -> OpportunityReference {
        OpportunityReference {
            uid: self.uid,
            slug: self.slug,
            title: self.title,
            image_url: self.image_url,
            short_desc: self.short_desc,
        }
    }
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

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
pub struct AnnotatedOpportunityExterior {
    #[serde(flatten)]
    pub exterior: OpportunityExterior,
    pub accepted: bool,
    pub withdrawn: bool,
    pub current: bool,
    pub authorized: PermitAction,
    pub review_status: ReviewStatus,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct OpportunityInterior {
    pub accepted: Option<bool>,
    pub withdrawn: bool,
    pub submitted_by: Option<Uuid>,
    pub review_status: ReviewStatus,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub extra_data: serde_json::Value,
}

impl Default for OpportunityInterior {
    fn default() -> Self {
        OpportunityInterior {
            accepted: Some(false), // editors have accepted it for publication
            withdrawn: false,      // partner has withdrawn it from publication
            submitted_by: None,
            review_status: ReviewStatus::NotRequired,
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

#[derive(Serialize, Debug)]
pub struct OpportunityForCsv {
    pub uid: Uuid,
    pub slug: String,
    pub partner_name: String,
    pub partner_website: Option<String>,
    pub partner_logo_url: Option<String>,
    pub partner_created: Option<DateTime<FixedOffset>>,
    pub partner_updated: Option<DateTime<FixedOffset>>,
    pub partner_opp_url: Option<String>,
    pub organization_name: String,
    pub organization_type: OrganizationType,
    pub organization_website: Option<String>,
    pub organization_logo_url: Option<String>,
    pub entity_type: EntityType,
    pub opp_venue: String,
    pub opp_descriptor: String,
    pub min_age: i16,
    pub max_age: i16,
    pub pes_domain: Domain,
    pub tags: String,
    pub opp_topics: String,
    pub ticket_required: bool,
    pub title: String,
    pub description: String,
    pub short_desc: String,
    pub image_url: String,
    pub image_credit: String,
    pub start_datetimes: String,
    pub has_end: bool,
    pub end_datetimes: String,
    pub recurrence: Recurrence,
    pub end_recurrence: Option<String>,
    pub timezone: Option<String>,
    pub cost: Cost,
    pub languages: String,
    pub is_online: bool,
    pub location_type: LocationType,
    pub location_name: String,
    pub location_point: Option<String>,
    pub location_polygon: Option<String>,
    pub address_street: String,
    pub address_city: String,
    pub address_state: String,
    pub address_country: String,
    pub address_zip: String,
    pub opp_hashtags: String,
    pub partner: Uuid,
    pub accepted: Option<bool>,
    pub withdrawn: bool,
    pub contact_name: String,
    pub contact_email: String,
    pub contact_phone: String,
    pub extra_data: String,
}

impl From<Opportunity> for OpportunityForCsv {
    fn from(opp: Opportunity) -> Self {
        OpportunityForCsv {
            uid: opp.exterior.uid,
            slug: opp.exterior.slug,
            partner_name: opp.exterior.partner_name,
            partner_website: opp.exterior.partner_website,
            partner_logo_url: opp.exterior.partner_logo_url,
            partner_created: opp.exterior.partner_created,
            partner_updated: opp.exterior.partner_updated,
            partner_opp_url: opp.exterior.partner_opp_url,
            organization_name: opp.exterior.organization_name,
            organization_type: opp.exterior.organization_type,
            organization_website: opp.exterior.organization_website,
            organization_logo_url: opp.exterior.organization_logo_url,
            entity_type: opp.exterior.entity_type,
            opp_venue: opp
                .exterior
                .opp_venue
                .into_iter()
                .fold(String::new(), |mut accum, add| {
                    if !accum.is_empty() {
                        accum.push_str(", ");
                    }
                    accum.push_str(add.as_ref());
                    accum
                }),
            opp_descriptor: opp.exterior.opp_descriptor.into_iter().fold(
                String::new(),
                |mut accum, add| {
                    if !accum.is_empty() {
                        accum.push_str(", ");
                    }
                    accum.push_str(add.as_ref());
                    accum
                },
            ),
            min_age: opp.exterior.min_age,
            max_age: opp.exterior.max_age,
            pes_domain: opp.exterior.pes_domain,
            tags: opp
                .exterior
                .tags
                .into_iter()
                .fold(String::new(), |mut accum, add| {
                    if !accum.is_empty() {
                        accum.push_str(", ");
                    }
                    accum.push_str(&add);
                    accum
                }),
            opp_topics: opp.exterior.opp_topics.into_iter().fold(
                String::new(),
                |mut accum, add| {
                    if !accum.is_empty() {
                        accum.push_str(", ");
                    }
                    accum.push_str(add.as_ref());
                    accum
                },
            ),
            ticket_required: opp.exterior.ticket_required,
            title: opp.exterior.title,
            description: opp.exterior.description,
            short_desc: opp.exterior.short_desc,
            image_url: opp.exterior.image_url,
            image_credit: opp.exterior.image_credit,
            start_datetimes: opp.exterior.start_datetimes.into_iter().fold(
                String::new(),
                |mut accum, add| {
                    if !accum.is_empty() {
                        accum.push_str(", ");
                    }
                    accum.push_str(&add.to_rfc3339());
                    accum
                },
            ),
            has_end: opp.exterior.has_end,
            end_datetimes: opp.exterior.end_datetimes.into_iter().fold(
                String::new(),
                |mut accum, add| {
                    if !accum.is_empty() {
                        accum.push_str(", ");
                    }
                    accum.push_str(&add.to_rfc3339());
                    accum
                },
            ),
            recurrence: opp.exterior.recurrence,
            end_recurrence: opp.exterior.end_recurrence.map(|dt| dt.to_rfc3339()),
            timezone: opp.exterior.timezone,
            cost: opp.exterior.cost,
            languages: opp.exterior.languages.join(", "),
            is_online: opp.exterior.is_online,
            location_type: opp.exterior.location_type,
            location_name: opp.exterior.location_name,
            location_point: opp
                .exterior
                .location_point
                .map(|point| serde_json::to_string(&point).unwrap_or_else(|_| "ERROR".to_string())),
            location_polygon: opp
                .exterior
                .location_polygon
                .map(|poly| serde_json::to_string(&poly).unwrap_or_else(|_| "ERROR".to_string())),
            address_street: opp.exterior.address_street,
            address_city: opp.exterior.address_city,
            address_state: opp.exterior.address_state,
            address_country: opp.exterior.address_country,
            address_zip: opp.exterior.address_zip,
            opp_hashtags: opp.exterior.opp_hashtags.join(", "),
            partner: opp.exterior.partner,
            accepted: opp.interior.accepted,
            withdrawn: opp.interior.withdrawn,
            contact_name: opp.interior.contact_name,
            contact_email: opp.interior.contact_email,
            contact_phone: opp.interior.contact_phone,
            extra_data: opp.interior.extra_data.to_string(),
        }
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

#[derive(Serialize, Deserialize, Default, FromRow)]
#[serde(default)]
pub struct OpportunityReference {
    pub uid: Uuid,
    pub slug: String,
    pub title: String,
    pub image_url: String,
    pub short_desc: String,
}

impl std::fmt::Display for OpportunityReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.title)
    }
}

#[derive(Serialize, Default, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OpportunityQueryPhysical {
    #[default]
    InPersonOrOnline,
    InPerson,
    Online,
}

#[derive(Serialize, Default, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OpportunityQueryTemporal {
    #[default]
    OnDemandOrScheduled,
    Scheduled,
    OnDemand,
}

#[derive(Serialize, Default, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OpportunityQueryOrdering {
    #[default]
    Alphabetical,
    Closest,
    Soonest,
    Any,
    Native,
    Unique,
    PartnerName,
}

/// Each field represents one of the database fields by which
/// Opportunity queries can be narrowed. The default value does not
/// narrow the query at all, so to find all of the opportunities with
/// a particular string in the name, we could do something like:
/// ```
/// Opportunity::load_matching(db.acquire().await?, OpportunityQuery { title_contains: "hello".to_string(), ..Default::default() })
/// ```
#[derive(Default, Serialize, Deserialize, Debug)]
pub struct OpportunityQuery {
    pub uid: Option<Uuid>,
    pub slug: Option<String>,
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub entity_type: Option<Vec<EntityType>>,
    pub title_contains: Option<String>,
    pub tags: Option<Vec<String>>,
    pub topics: Option<Vec<Topic>>,
    pub partner: Option<Uuid>,
    pub partner_member: Option<Uuid>,
    pub prefer_partner: Option<Uuid>,
    pub near: Option<(f32, f32, f32)>,
    pub physical: Option<OpportunityQueryPhysical>,
    pub temporal: Option<OpportunityQueryTemporal>,
    pub text: Option<String>,
    pub beginning: Option<DateTime<FixedOffset>>,
    pub ending: Option<DateTime<FixedOffset>>,
    pub min_age: Option<i16>,
    pub max_age: Option<i16>,
    pub kids_only: Option<bool>,
    pub adults_only: Option<bool>,
    pub descriptors: Option<Vec<Descriptor>>,
    pub cost: Option<Cost>,
    pub venue_type: Option<VenueType>,
    pub host: Option<String>,
    pub sort: Option<OpportunityQueryOrdering>,
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub involved: Option<Uuid>,
    pub saved: Option<Uuid>,
    pub participated: Option<Uuid>,
    /// probability of retaining any given result in the match set, in the range (0-1).
    pub sample: Option<f32>,
    pub exclude: Option<Vec<Uuid>>,
    pub current: Option<bool>,
    pub calendar: Option<(u32, u8)>,
    pub region: Option<String>,
}

#[derive(Debug)]
enum ParamValue {
    // Raw here means it's not converted to JSON before sending it to
    // the database.
    RawString(String),
    RawFloat(f32),
    RawInt(i32),
    RawBool(bool),
    RawUuid(Uuid),
    RawVecString(Vec<String>),
    RawVecEntityType(Vec<EntityType>),
    RawVenueType(VenueType),
    Bool(bool),
    Uuid(Uuid),
    VecString(Vec<String>),
    //VecTopic(Vec<Topic>),
    VecEntityType(Vec<EntityType>),
    //VecDescriptor(Vec<Descriptor>),
    VecVenueType(Vec<VenueType>),
    VecUuid(Vec<Uuid>),
}

impl ParamValue {
    fn append(self, params: &mut Vec<ParamValue>) -> usize {
        params.push(self);
        params.len()
    }

    fn add_to_query(
        self,
        query: Query<Postgres, PgArguments>,
    ) -> Result<Query<Postgres, PgArguments>, Error> {
        Ok(match self {
            ParamValue::RawString(val) => query.bind(val),
            ParamValue::RawFloat(val) => query.bind(val),
            ParamValue::RawInt(val) => query.bind(val),
            ParamValue::RawBool(val) => query.bind(val),
            ParamValue::RawUuid(val) => query.bind(val),
            ParamValue::RawVecString(val) => query.bind(val),
            ParamValue::RawVecEntityType(val) => query.bind(
                val.into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<DBEntityType>>(),
            ),
            ParamValue::RawVenueType(val) => query.bind(val),

            ParamValue::Bool(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::Uuid(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecString(val) => query.bind(serde_json::to_value(val)?),
            //ParamValue::VecTopic(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecEntityType(val) => query.bind(serde_json::to_value(val)?),
            //ParamValue::VecDescriptor(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecVenueType(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecUuid(val) => query.bind(serde_json::to_value(val)?),
        })
    }

    fn add_all_to_query<'req>(
        params: Vec<ParamValue>,
        mut query: Query<Postgres, PgArguments>,
    ) -> Result<Query<Postgres, PgArguments>, Error> {
        for value in params.into_iter() {
            query = value.add_to_query(query)?;
        }

        Ok(query)
    }
}

fn build_matching_query(
    fields: &[&str],
    query: &OpportunityQuery,
    mut ordering: OpportunityQueryOrdering,
    pagination: Pagination,
) -> Result<(String, Vec<ParamValue>), Error> {
    let mut clauses = Vec::new();
    let mut params = Vec::new();

    // Use *geography* versions of the functions
    // https://postgis.net/docs/ST_DWithin.html
    // https://postgis.net/docs/ST_Distance.html
    // https://postgis.net/docs/ST_Intersects.html

    if let Some(uid) = query.uid {
        // clauses.push(format!(
        //     "(${}::jsonb) @> (primary_table.exterior -> 'uid')",
        //     ParamValue::Uuid(uid).append(&mut params)
        // ));

        clauses.push(format!(
            "${} = search.uid",
            ParamValue::RawUuid(uid).append(&mut params)
        ));
    }

    if let Some(slug) = &query.slug {
        // clauses.push(format!(
        //     "${} = (primary_table.exterior ->> 'slug')",
        //     ParamValue::RawString(slug.to_string()).append(&mut params)
        // ));

        clauses.push(format!(
            "${} = search.slug",
            ParamValue::RawString(slug.to_string()).append(&mut params)
        ));
    }

    if let Some(val) = query.accepted {
        // clauses.push(format!(
        //     "(${}::jsonb) @> (primary_table.interior -> 'accepted')",
        //     ParamValue::Bool(val).append(&mut params)
        // ));

        clauses.push(format!(
            "${} = search.accepted",
            ParamValue::RawBool(val).append(&mut params)
        ));
    }

    if let Some(val) = query.withdrawn {
        if val {
            // clauses.push(
            //     "(('true'::jsonb) @> (primary_table.interior -> 'withdrawn') OR coalesce(nullif(primary_table.interior ->> 'review_status', ''), 'not_required') IN ('draft', 'pending'))".to_string(),
            // );
            clauses.push(String::from("true = search.withdrawn OR coalesce(search.review_status, 'not_required') IN ('draft', 'pending'))"));
        } else {
            // clauses.push(
            //     "(('false'::jsonb) @> (primary_table.interior -> 'withdrawn') AND coalesce(nullif(primary_table.interior ->> 'review_status', ''), 'not_required') NOT IN ('draft', 'pending'))".to_string(),
            // );
            clauses.push(String::from("false = search.withdrawn AND coalesce(search.review_status, 'not_required') NOT IN ('draft', 'pending'))"));
        }
    }

    if let Some(region) = &query.region {
        //         clauses.push(format!(
        //             r#"(
        // SELECT
        //  COALESCE(
        //   NULLIF(ST_Intersects(c_region.geometry, primary_table.location_point), false),
        //   NULLIF(ST_Intersects(c_region.geometry, primary_table.location_polygon), false),
        //   false
        //  )
        // FROM c_region WHERE "name" = ${}
        // )"#,
        //             ParamValue::RawString(region.to_owned()).append(&mut params)
        //         ));

        clauses.push(format!(
            r#"
SELECT
 COALESCE(
  NULLIF(ST_Intersects(c_region.geometry, search.location_point), false),
  NULLIF(ST_Intersects(c_region.geometry, search.location_polygon), false),
  false
 )
FROM c_region WHERE "name" = ${}
"#,
            ParamValue::RawString(region.to_owned()).append(&mut params)
        ));
    }

    if let Some((year, month)) = query.calendar {
        let (next_year, next_month) = if month > 11 {
            (year + 1, 1)
        } else {
            (year, month + 1)
        };

        let begin = format!("{year:04}-{month:02}-01T00:00:00Z");
        let end = format!("{next_year:04}-{next_month:02}-01T00:00:00Z");

        let begin_param = ParamValue::RawString(begin).append(&mut params);
        let end_param = ParamValue::RawString(end).append(&mut params);

        // clauses.push(format!(
        //     r#"(
        //         (
        //          EXISTS (SELECT value FROM jsonb_array_elements_text(primary_table.exterior -> 'start_datetimes') WHERE value::timestamptz > ${}::timestamptz AND value::timestamptz < ${}::timestamptz)
        //          AND
        //          EXISTS (SELECT value FROM jsonb_array_elements_text(primary_table.exterior -> 'end_datetimes') WHERE value::timestamptz > ${}::timestamptz AND value::timestamptz < ${}::timestamptz)
        //         )
        //         OR
        //         (
        //          coalesce(nullif(primary_table.exterior ->> 'end_recurrence', ''), '0001-01-01')::timestamptz > ${}::timestamptz
        //         )
        //        )"#,
        //     begin_param, end_param, begin_param, end_param, begin_param));

        clauses.push(format!(
            r#"(
                (
                 EXISTS (SELECT value FROM search.start_datetimes WHERE value > ${} AND value < ${})
                 AND
                 EXISTS (SELECT value FROM search.end_datetimes WHERE value > ${} AND value < ${})
                )
                OR
                (
                 coalesce(search.end_recurrence, '0001-01-01')::timestamptz > ${}::timestamptz
                )
               )"#,
            begin_param, end_param, begin_param, end_param, begin_param
        ));
    } else {
        if let Some(val) = query.current {
            // clauses.push(format!(
            //     r#"c_opportunity_is_current(primary_table.interior, primary_table.exterior) = ${}"#,
            //     ParamValue::RawBool(val).append(&mut params)
            // ));
            clauses.push(format!(
                r#"
${} = (
  search.review_status in ('publish', 'not_required')
  and
  search.accepted = true
  and
  search.withdrawn = false
  and
  (
    (
      array_length(search.start_datetimes, 1) <= 1
      and
      array_length(search.end_datetimes, 1) = 0
    )
    or
    exists (select value from search.start_datetimes where value > now())
    or
    exists (select value from search.end_datetimes where value > now())
    or
    (
      (search.recurrence = 'daily' OR search.recurrence = 'weekly')
      and
      (search.end_recurrence' is null or (search.end_recurrence > now())
    )
  )
"#,
                ParamValue::RawBool(val).append(&mut params)
            ));
        }
    }

    if let Some(person) = query.involved {
        clauses.push(format!(
            r#"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE (inv.exterior -> 'opportunity') @> ('"' || search.uid::text || '"')::jsonb
              AND (inv.interior -> 'participant') @> ${}::jsonb
              AND (inv.exterior ->> 'mode')::integer >= ${})"#,
            ParamValue::Uuid(person).append(&mut params),
            ParamValue::RawInt(involvement::Mode::Interest as i32).append(&mut params),
        ));
    }

    if let Some(person) = query.saved {
        clauses.push(format!(
            r#"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE (inv.exterior -> 'opportunity') @> ('"' || search.uid::text || '"')::jsonb
              AND (inv.interior -> 'participant') @> ${}::jsonb
              AND (inv.exterior ->> 'mode')::integer = ${})"#,
            ParamValue::Uuid(person).append(&mut params),
            ParamValue::RawInt(involvement::Mode::Saved as i32).append(&mut params),
        ));
    }

    if let Some(person) = query.participated {
        clauses.push(format!(
            r#"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE (inv.exterior -> 'opportunity') @> ('"' || search.uid::text || '"')::jsonb
              AND (inv.interior -> 'participant') @> ${}::jsonb
              AND (inv.exterior ->> 'mode')::integer >= ${})"#,
            ParamValue::Uuid(person).append(&mut params),
            ParamValue::RawInt(involvement::Mode::Logged as i32).append(&mut params),
        ));
    }

    if let Some(val) = &query.entity_type {
        // clauses.push(format!(
        //     r"(primary_table.exterior -> 'entity_type') <@ ${}",
        //     ParamValue::VecEntityType(val.clone()).append(&mut params)
        // ));
        clauses.push(format!(
            "search.entity_type = any(${})",
            ParamValue::RawVecEntityType(val.clone()).append(&mut params)
        ));
    }

    if let Some(val) = &query.title_contains {
        // clauses.push(format!(
        //     "(primary_table.exterior ->> 'title') ILIKE ${}",
        //     ParamValue::RawString(format!("%{}%", val)).append(&mut params)
        // ));
        clauses.push(format!(
            "search.title ilike ${}",
            ParamValue::RawString(format!("%{}%", val)).append(&mut params)
        ));
    }

    if let Some(val) = &query.tags {
        // clauses.push(format!(
        //     "(primary_table.exterior -> 'tags') @> ${}",
        //     ParamValue::VecString(val.clone()).append(&mut params)
        // ));

        clauses.push(format!(
            "search.tags && ${}", // In Postgresql when comparing arrays, && means 'is the intersection not empty?'
            ParamValue::RawVecString(val.clone()).append(&mut params)
        ));
    }

    if let Some(val) = &query.topics {
        // clauses.push(format!(
        //     "(primary_table.exterior -> 'opp_topics') ?| ${}",
        //     ParamValue::RawVecString(val.clone().into_iter().map(|x| x.to_string()).collect())
        //         .append(&mut params)
        // ));
        clauses.push(format!(
            "search.topics && ${}",
            ParamValue::RawVecString(val.iter().map(|v| v.db_repr()).collect()).append(&mut params)
        ));
    }

    if let Some(val) = &query.descriptors {
        // clauses.push(format!(
        //     "(primary_table.exterior -> 'opp_descriptor') ?| ${}",
        //     ParamValue::RawVecString(val.clone().into_iter().map(|x| x.to_string()).collect())
        //         .append(&mut params)
        // ))

        clauses.push(format!(
            "search.descriptors && ${}",
            ParamValue::RawVecString(val.iter().map(|v| v.db_repr()).collect()).append(&mut params)
        ));
    }

    if let Some(val) = &query.partner {
        // clauses.push(format!(
        //     "(${}::jsonb) @> (primary_table.exterior -> 'partner')",
        //     ParamValue::Uuid(val.clone()).append(&mut params)
        // ));

        clauses.push(format!(
            "search.partner = ${}",
            ParamValue::RawUuid(*val).append(&mut params)
        ));
    }

    if let Some(val) = &query.partner_member {
        let uuid_param = ParamValue::Uuid(val.clone()).append(&mut params);
        clauses.push(format!(
            r#"
(
  (primary_table.interior -> 'submitted_by' @> ${}::jsonb)
OR
  (
    SELECT jsonb_agg("uid") FROM (
        SELECT (c_partner.exterior -> 'uid') AS "uid" FROM c_partner
        WHERE (c_partner.interior -> 'authorized') @> (${}::jsonb)
        OR (c_partner.interior -> 'prime') @> (${}::jsonb)
    ) AS "authorized_partners"
  ) @> (primary_table.exterior -> 'partner')
)"#,
            uuid_param, uuid_param, uuid_param
        ));
    }

    if let Some(text) = &query.text {
        // clauses.push(format!(
        //     "primary_table.fulltext_english @@ websearch_to_tsquery(${})",
        //     ParamValue::RawString(text.to_string()).append(&mut params)
        // ));

        clauses.push(format!(
            "search.fulltext_english @@ websearch_to_tsquery(${})",
            ParamValue::RawString(text.to_string()).append(&mut params)
        ));
    }

    if let Some(beginning) = &query.beginning {
        let time_param = ParamValue::RawString(beginning.to_rfc3339()).append(&mut params);
        // clauses.push(format!(
        //     r"(EXISTS (SELECT value FROM jsonb_array_elements_text(primary_table.exterior -> 'start_datetimes') WHERE value::timestamptz > ${}::timestamptz)
        //       OR
        //       EXISTS (SELECT value FROM jsonb_array_elements_text(primary_table.exterior -> 'end_datetimes') WHERE value::timestamptz > ${}::timestamptz)
        //       OR
        //       ((primary_table.exterior->>'recurrence' = 'daily' OR primary_table.exterior->>'recurrence' = 'weekly') AND (primary_table.exterior->>'end_recurrence' IS null OR (primary_table.exterior->>'end_recurrence')::timestamptz > ${}::timestamptz ))
        //       OR (
        //        jsonb_array_length(primary_table.exterior -> 'start_datetimes') <= 1
        //        AND
        //        jsonb_array_length(primary_table.exterior -> 'end_datetimes') = 0
        //       ))",
        // time_param, time_param, time_param));

        clauses.push(format!(
            r#"
            (
              exists(select value from search.start_datetimes where value > ${})
              or
              exists(select value from search.end_datetimes where value > ${})
              or (
                (search.recurrence = 'daily' or search.recurrence = 'weekly')
                and
                (search.end_recurrence is null or search.end_recurrence > ${})
              )
              or (
               array_length(search.start_datetimes, 1) <= 1
               and
               array_length(search.end_datetimes) = 0
              )
            )"#,
            time_param, time_param, time_param
        ));
    }

    if let Some(ending) = &query.ending {
        let time_param = ParamValue::RawString(ending.to_rfc3339()).append(&mut params);
        // clauses.push(format!(
        //     r"(NOT EXISTS (SELECT value FROM jsonb_array_elements_text(primary_table.exterior -> 'start_datetimes') WHERE value::timestamptz > ${}::timestamptz)
        //       AND
        //       NOT EXISTS (SELECT value FROM jsonb_array_elements_text(primary_table.exterior -> 'end_datetimes') WHERE value::timestamptz > ${}::timestamptz))",
        // time_param, time_param));

        clauses.push(format!(
            r#"
            (
              not exists (select value from search.start_datetimes where value > ${})
              and
              not exists (select value from search.end_datetimes where value > ${})
            )"#,
            time_param, time_param
        ));
    }

    // Minimum and maximum age in queries each define a contraint on
    // the opposite project field. A queried min age checks that the
    // opportunity max age is greater than the query min age, and a
    // queried max age checks that the opporuntity minimum is less
    // than the queried minimum
    if let Some(min_age) = &query.min_age {
        // clauses.push(format!(
        //     "(primary_table.exterior -> 'max_age')::integer >= ${}",
        //     ParamValue::RawInt(*min_age as i32).append(&mut params)
        // ))

        clauses.push(format!(
            "search.max_age >= ${}",
            ParamValue::RawInt(*min_age as i32).append(&mut params)
        ));
    }

    if let Some(max_age) = &query.max_age {
        // clauses.push(format!(
        //     "(primary_table.exterior -> 'min_age')::integer <= ${}",
        //     ParamValue::RawInt(*max_age as i32).append(&mut params)
        // ))

        clauses.push(format!(
            "search.min_age <= ${}",
            ParamValue::RawInt(*max_age as i32).append(&mut params)
        ));
    }

    if query.kids_only.unwrap_or(false) {
        // clauses.push("(primary_table.exterior -> 'max_age')::integer <= 18".to_string())
        clauses.push("search.max_age <= 18".to_string())
    }

    if query.adults_only.unwrap_or(false) {
        // clauses.push("(primary_table.exterior -> 'min_age')::integer >= 21".to_string())
        clauses.push("search.min_age >= 21".to_string())
    }

    if let Some(cost) = &query.cost {
        // clauses.push(format!(
        //     "(primary_table.exterior ->> 'cost') = ${}",
        //     ParamValue::RawString(cost.as_ref().to_lowercase()).append(&mut params)
        // ))

        clauses.push(format!(
            "search.cost = ${}",
            ParamValue::RawString(cost.db_repr()).append(&mut params)
        ));
    }

    if let Some(venue_type) = &query.venue_type {
        // clauses.push(format!(
        //     "(primary_table.exterior -> 'opp_venue') @> ${}",
        //     ParamValue::VecVenueType(vec![venue_type.clone()]).append(&mut params)
        // ))

        clauses.push(format!(
            "any(search.venue_type) = ${}",
            ParamValue::RawVenueType(venue_type.clone()).append(&mut params)
        ));
    }

    if let Some(host) = &query.host {
        // clauses.push(format!(
        //     "(primary_table.exterior ->> 'organization_name') ILIKE ${}",
        //     ParamValue::RawString(format!("%{}%", host)).append(&mut params)
        // ))

        clauses.push(format!(
            "search.organization_name ilike ${}",
            ParamValue::RawString(host.to_string()).append(&mut params)
        ));
    }

    if let Some(physical) = &query.physical {
        match physical {
            OpportunityQueryPhysical::InPersonOrOnline => {}
            OpportunityQueryPhysical::InPerson => {
                // clauses.push(format!(
                //     "(${}::jsonb) @> (exterior -> 'is_online')",
                //     ParamValue::Bool(false).append(&mut params)
                // ));

                // The area constant is ten thousand square miles in square meters

                // clauses.push(format!(
                //     "(((${}::jsonb) @> (primary_table.exterior -> 'is_online')) AND (primary_table.location_polygon IS NULL OR ST_Area(primary_table.location_polygon, false) <= 25899752356) AND (primary_table.exterior ->> 'location_type' NOT IN ('any', 'unknown')))",
                //     ParamValue::Bool(false).append(&mut params)
                // ));

                clauses.push(String::from(
                    r#"
                    (
                      (search.is_online = false)
                      and
                      (search.location_polygon is null or ST_Area(search.location_polygon, false) <= 25899752356)
                      and
                      (search.location_type not in ('any', 'unknown'))
                    )"#,
                ));
            }
            OpportunityQueryPhysical::Online => {
                // clauses.push(format!(
                //     "(${}::jsonb) @> (exterior -> 'is_online')",
                //     ParamValue::Bool(true).append(&mut params)
                // ));

                // The area constant is ten thousand square miles in square meters

                // clauses.push(format!("(((${}::jsonb) @> (primary_table.exterior -> 'is_online')) OR (primary_table.location_polygon IS NOT NULL AND ST_Area(primary_table.location_polygon, false) > 25899752356))", ParamValue::Bool(true).append(&mut params)));

                clauses.push(String::from(
                    r#"
                    (
                      (search.is_online = true)
                      or
                      (search.location_polygon is not null and ST_Area(search.location_polygon, false) > 25899752356)
                    )"#
                ));
            }
        }
    }

    if let Some(temporal) = &query.temporal {
        match temporal {
            OpportunityQueryTemporal::OnDemandOrScheduled => {}
            OpportunityQueryTemporal::Scheduled => {
                // clauses.push(
                //     "c_opportunity_is_scheduled(primary_table.interior, primary_table.exterior)"
                //         .into(),
                // );

                clauses.push(String::from(
                    r#"
                    array_length(search.start_datetimes, 1) > 1
                    or
                    array_length(search.end_datetimes, 1) > 1
                    or (
                      array_length(search.start_datetimes, 1) = 1
                      and
                      array_length(search.end_datetimes, 1) = 1
                      and
                      age(search.end_datetimes[0], search.start_datetimes[0]) <= interval '7 days'
                    )"#,
                ));
            }
            OpportunityQueryTemporal::OnDemand => {
                // clauses.push(
                //     "c_opportunity_is_ondemand(primary_table.interior, primary_table.exterior)"
                //         .into(),
                // );

                clauses.push(String::from(
                    r#"
                    array_length(search.start_datetimes, 1) <= 1
                    and
                    array_length(search.end_datetimes, 1) <= 1
                    and (
                      array_length(search.start_datetimes, 1) != 1
                      or
                      array_length(search.end_datetimes, 1) != 1
                      or
                      age(search.end_datetimes[0], search.start_datetimes[0]) > interval '7 days'
                    )"#,
                ));
            }
        }
    }

    //     let point = if let Some((longitude, latitude, proximity)) = &query.near {
    //         let lon_param = ParamValue::RawFloat(*longitude).append(&mut params);
    //         let lat_param = ParamValue::RawFloat(*latitude).append(&mut params);
    //         let prox_param = ParamValue::RawFloat(*proximity).append(&mut params);

    //         clauses.push(format!(
    //             r#"(
    //   (exterior ->> 'location_type') = 'any'
    //   OR
    //   CASE WHEN location_polygon IS NOT NULL
    //     THEN ST_Intersects(ST_Buffer(ST_SetSRID(ST_Point(${}, ${}), 4326)::geography, ${}), location_polygon)
    //     ELSE false END
    //   OR
    //   CASE WHEN location_point IS NOT NULL
    //     THEN ST_Distance(ST_SetSRID(ST_Point(${}, ${}), 4326)::geography, location_point, false) < ${}
    //     ELSE false END
    // )"#,
    //             lon_param, lat_param, prox_param,
    //             lon_param, lat_param, prox_param
    //         ));

    //         Some((lon_param, lat_param))
    //     } else {
    //         None
    //     };

    if let Some(probability) = query.sample {
        clauses.push(format!(
            "random() < ${}",
            ParamValue::RawFloat(probability).append(&mut params)
        ));
    }

    if let Some(exclusions) = &query.exclude {
        clauses.push(format!(
            "NOT (search.uid = any(${}))",
            ParamValue::VecUuid(exclusions.clone()).append(&mut params)
        ));
    }

    let mut query_string = "SELECT ".to_string();

    if ordering == OpportunityQueryOrdering::Unique {
        query_string.push_str("DISTINCT ON (search.title, search.partner) ");
    }

    match fields.len() {
        0 => query_string.push_str("primary_table.*"),
        1 => query_string.push_str(fields[0]),
        _ => query_string.push_str(fields.join(", ").as_str()),
    }

    //     query_string.push_str(
    //         r#" FROM (
    // SELECT
    //     id,
    //     created,
    //     updated,
    //     location_point,
    //     location_polygon,
    //     fulltext_english,
    //     (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) AS exterior,
    //     (c_opportunity.interior || COALESCE(c_opportunity_overlay.interior, '{}'::jsonb)) AS interior
    // "#,
    //     );

    query_string.push_str(
        r#" FROM c_opportunity_search search JOIN (
SELECT
    id,
    created,
    updated,
    location_point,
    location_polygon,
    fulltext_english,
    (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) AS exterior,
    (c_opportunity.interior || COALESCE(c_opportunity_overlay.interior, '{}'::jsonb)) AS interior
"#,
    );

    if let Some(uid) = &query.prefer_partner {
        query_string.push_str(&format!(
            ", ((${}::jsonb) @> (c_opportunity.exterior -> 'partner'))::int AS _sort_preferential",
            ParamValue::Uuid(uid.clone()).append(&mut params)
        ));
    } else {
        query_string.push_str(", 0 as _sort_preferential");
    }

    if let Some((longitude, latitude, proximity)) = &query.near {
        let lon_param = ParamValue::RawFloat(*longitude).append(&mut params);
        let lat_param = ParamValue::RawFloat(*latitude).append(&mut params);
        let prox_param = ParamValue::RawFloat(*proximity).append(&mut params);

        query_string.push_str(", CASE WHEN (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) ->> 'location_type' = 'any' OR (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) ->> 'is_online' = 'true' THEN 2 ELSE 1 END AS _sort_location_priority");

        query_string.push_str(", CASE");

        query_string.push_str(
            &format!(" WHEN location_polygon IS NOT NULL THEN ST_Distance(location_polygon, ST_SetSRID(ST_Point(${lon_param}, ${lat_param}), 4326)::geography, false)")
        );

        query_string
            .push_str(&format!(" WHEN location_point IS NOT NULL THEN ST_Distance(location_point, ST_SetSRID(ST_Point(${lon_param}, ${lat_param}), 4326)::geography, false)"));

        query_string.push_str(&format!(" WHEN (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{{}}'::jsonb)) ->> 'location_type' = 'any' OR (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{{}}'::jsonb)) ->> 'is_online' = 'true' THEN ${prox_param}"));

        // This constant number is roughly the square root of the surface area of the earth, in meters, i.e. about as far away as you can get
        query_string.push_str(" ELSE 22585394 END AS _sort_distance");

        if *proximity > 0.0 {
            clauses.push(format!("(_sort_distance < 1.1 * ${prox_param})"));
        }
    } else {
        query_string.push_str(", CASE WHEN (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) ->> 'location_type' = 'any' OR (c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) ->> 'is_online' = 'true' THEN 0 ELSE 1 END AS _sort_location_priority");
        query_string.push_str(", 1 AS _sort_distance");
    }

    query_string.push_str(", CASE WHEN location_polygon IS NOT NULL THEN ST_Area(location_polygon, false) ELSE 0 END AS _sort_area");

    // We bump ongoing opportunities so that they sort as a week in the future, to give actual timely opportunities priority
    query_string.push_str(r#",
            CASE
              WHEN jsonb_array_length((c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) -> 'start_datetimes') = 0 AND jsonb_array_length((c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) -> 'end_datetimes') = 0
              THEN CURRENT_TIMESTAMP + INTERVAL '7 days'
              WHEN EXISTS (SELECT 1 FROM jsonb_array_elements_text((c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) -> 'start_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
              THEN (SELECT MIN(value::timestamptz) FROM jsonb_array_elements_text((c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) -> 'start_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP LIMIT 1)
              WHEN EXISTS (SELECT 1 FROM jsonb_array_elements_text((c_opportunity.exterior || COALESCE(c_opportunity_overlay.exterior, '{}'::jsonb)) -> 'end_datetimes') WHERE value::timestamptz > CURRENT_TIMESTAMP)
              THEN CURRENT_TIMESTAMP + INTERVAL '7 days'
              ELSE '100000-01-01T00:00:00.0+00:00'::timestamptz
            END AS _sort_time
        "#);

    // query_string.push_str(" FROM c_opportunity LEFT JOIN c_opportunity_overlay ON c_opportunity.id = c_opportunity_overlay.opportunity_id) AS primary_table");
    query_string.push_str(" FROM c_opportunity LEFT JOIN c_opportunity_overlay ON c_opportunity.id = c_opportunity_overlay.opportunity_id) AS primary_table ON search.opp_id = primary_table.id");

    if ordering == OpportunityQueryOrdering::PartnerName {
        // query_string.push_str(" LEFT JOIN c_partner ON primary_table.exterior->>'partner' = c_partner.exterior->>'uid'");
        query_string.push_str(
            " LEFT JOIN c_partner ON search.partner = (c_partner.exterior->>'uid')::uuid",
        );
    }

    if !clauses.is_empty() {
        query_string.push_str(" WHERE");
    }

    let mut first = true;

    for clause in clauses.into_iter() {
        if first {
            query_string.push(' ');
            first = false;
        } else {
            query_string.push_str(" AND ");
        }
        query_string.push_str(&clause);
    }

    if let (Some(_), OpportunityQueryOrdering::Closest) = (query.calendar, ordering) {
        ordering = OpportunityQueryOrdering::Soonest;
    }

    match ordering {
        OpportunityQueryOrdering::Alphabetical => {
            query_string.push_str(" ORDER BY (search.title) ASC");
        }
        OpportunityQueryOrdering::Closest => {
            query_string.push_str(
                " ORDER BY _sort_preferential DESC, _sort_location_priority ASC, _sort_distance + sqrt(_sort_area) ASC, _sort_time ASC",
            );
        }
        OpportunityQueryOrdering::Soonest => {
            query_string.push_str(
                " ORDER BY _sort_preferential DESC, _sort_location_priority ASC, _sort_time ASC, _sort_distance + sqrt(_sort_area) ASC",
            );
        }
        OpportunityQueryOrdering::Native => query_string.push_str(" ORDER BY id ASC"),
        OpportunityQueryOrdering::Any => {}
        OpportunityQueryOrdering::Unique => {
            query_string.push_str(" ORDER BY search.title, search.partner ASC")
        }
        OpportunityQueryOrdering::PartnerName => {
            query_string.push_str(" ORDER BY (c_partner.exterior->>'name') ASC, search.title ASC")
        }
    }

    match pagination {
        Pagination::All => query_string.push_str(";"),
        Pagination::One => query_string.push_str(" LIMIT 1;"),
        Pagination::Page { index, size } => {
            query_string.push_str(format!(" LIMIT {} OFFSET {};", size, index * size).as_ref())
        }
    };

    Ok((query_string, params))
}

fn slugify(source: &str) -> String {
    SLUGIFY_REPLACE
        .replace_all(&deunicode::deunicode(source.trim()), "-")
        .trim_end_matches("-") // Only trims from the trailing end of the string
        .to_lowercase()
}

#[derive(Debug)]
pub struct OpportunityImportRecord {
    pub id: i32,
    pub when: DateTime<Utc>,
    pub partner: Uuid,
    pub opportunity: Uuid,
    pub created: bool,
    pub ignored: bool,
}

impl OpportunityImportRecord {
    pub async fn store(
        db: &Database,
        partner: &Uuid,
        opportunity: &Uuid,
        created: bool,
        ignored: bool,
    ) -> Result<OpportunityImportRecord, Error> {
        Ok(sqlx::query_file_as!(
            OpportunityImportRecord,
            "db/opportunity/import_record_store.sql",
            partner,
            opportunity,
            created,
            ignored
        )
        .fetch_one(db)
        .await?)
    }
}

pub struct OpportunityPseudoIter {
    uids: VecDeque<Uuid>,
}

impl OpportunityPseudoIter {
    pub async fn get_next(&mut self, db: &Database) -> Option<Opportunity> {
        let Some(uid) = self.uids.pop_front() else {
            return None;
        };
        Opportunity::load_by_uid(db, &uid).await.ok()
    }
}

impl Opportunity {
    pub async fn catalog(db: &Database) -> Result<OpportunityPseudoIter, Error> {
        Ok(OpportunityPseudoIter {
            uids: sqlx::query!(r#"SELECT ("exterior"->>'uid')::uuid AS "uid!" FROM c_opportunity"#)
                .map(|row| row.uid)
                .fetch_all(db)
                .await?
                .into(),
        })
    }

    pub fn into_annotated_exterior(self, authorized: PermitAction) -> AnnotatedOpportunityExterior {
        let current = self.current();
        AnnotatedOpportunityExterior {
            exterior: self.exterior,
            accepted: self.interior.accepted.unwrap_or(false),
            withdrawn: self.interior.withdrawn,
            review_status: self.interior.review_status,
            current,
            authorized,
        }
    }

    pub fn current_as_of(&self, now: &DateTime<FixedOffset>) -> bool {
        let reviewed = match self.interior.review_status {
            ReviewStatus::Draft => false,
            ReviewStatus::Pending => false,
            ReviewStatus::Reject => false,
            ReviewStatus::Publish => true,
            ReviewStatus::NotRequired => true,
        };

        let publish = self.interior.accepted == Some(true) && !self.interior.withdrawn;

        let num_starts = self.exterior.start_datetimes.len();
        let num_ends = self.exterior.end_datetimes.len();
        let start_in_future = self.exterior.start_datetimes.iter().any(|dt| dt > now);
        let end_in_future = self.exterior.end_datetimes.iter().any(|dt| dt > now);
        let upcoming = (num_starts == 1 && num_ends == 0) || start_in_future || end_in_future;

        reviewed && publish && upcoming
    }

    pub fn expired_as_of(&self, now: &DateTime<FixedOffset>) -> bool {
        !self.current_as_of(now)
    }

    pub fn current(&self) -> bool {
        let now = chrono::Utc::now().to_fixed_offset();
        self.interior.accepted.unwrap_or(false)
            && !self.interior.withdrawn
            && self.current_as_of(&now)
    }

    pub fn expired(&self) -> bool {
        !self.current()
    }

    pub async fn count_matching(db: &Database, query: &OpportunityQuery) -> Result<u32, Error> {
        let (query_string, query_params) = build_matching_query(
            &["count(primary_table.*) as matches"],
            query,
            OpportunityQueryOrdering::Any,
            Pagination::One,
        )?;

        let query_obj = ParamValue::add_all_to_query(query_params, sqlx::query(&query_string))?;

        Ok(query_obj
            .fetch_one(db)
            .await?
            .try_get::<i64, _>("matches")
            .unwrap_or(0) as u32)
    }

    pub async fn load_matching_refs(
        db: &Database,
        query: &OpportunityQuery,
        ordering: OpportunityQueryOrdering,
        pagination: Pagination,
    ) -> Result<Vec<OpportunityReference>, Error> {
        let (query_string, query_params) = build_matching_query(
            &[
                "(primary_table.exterior -> 'uid') as uid",
                "(primary_table.exterior -> 'slug') as slug",
                "(primary_table.exterior -> 'title') as title",
                "(primary_table.exterior -> 'image_url') as image_url",
                "(primary_table.exterior -> 'short_desc') as short_desc",
            ],
            query,
            ordering,
            pagination,
        )?;

        let query_obj = ParamValue::add_all_to_query(query_params, sqlx::query(&query_string))?;

        query_obj
            .map(|rec| {
                Ok(OpportunityReference {
                    uid: serde_json::from_value(rec.get("uid"))?,
                    slug: serde_json::from_value(
                        rec.try_get("slug")
                            .unwrap_or_else(|_| serde_json::Value::String(String::new())),
                    )?,
                    title: serde_json::from_value(rec.get("title"))?,
                    image_url: serde_json::from_value(rec.get("image_url"))?,
                    short_desc: serde_json::from_value(rec.get("short_desc"))?,
                })
            })
            .fetch_all(db)
            .await?
            .into_iter()
            .collect()
    }

    pub async fn load_matching(
        db: &Database,
        query: &OpportunityQuery,
        ordering: OpportunityQueryOrdering,
        pagination: Pagination,
    ) -> Result<Vec<Opportunity>, Error> {
        let (query_string, query_params) = build_matching_query(&[], query, ordering, pagination)?;

        let query_obj = ParamValue::add_all_to_query(query_params, sqlx::query(&query_string))?;

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
            slug: self.exterior.slug.clone(),
            title: self.exterior.title.clone(),
            image_url: self.exterior.image_url.clone(),
            short_desc: self.exterior.short_desc.clone(),
        }
    }

    pub fn into_reference(self) -> OpportunityReference {
        OpportunityReference {
            uid: self.exterior.uid,
            slug: self.exterior.slug,
            title: self.exterior.title,
            image_url: self.exterior.image_url,
            short_desc: self.exterior.short_desc,
        }
    }

    pub async fn load_partner(&self, db: &Database) -> Result<super::partner::Partner, Error> {
        Ok(super::partner::Partner::load_by_uid(db, &self.exterior.partner).await?)
    }

    pub async fn reviews(&mut self, db: &Database) -> Result<Reviews, Error> {
        for_slug::reviews_for_slug(db, &self.exterior.slug).await
    }

    pub async fn likes(&mut self, db: &Database) -> Result<u32, Error> {
        for_slug::likes_for_slug(db, &self.exterior.slug).await
    }

    pub async fn validate(&mut self) -> Result<(), Error> {
        self.exterior.partner_name = self
            .exterior
            .partner_name
            .trim_matches(char::is_whitespace)
            .into();

        self.exterior.partner_opp_url = self
            .exterior
            .partner_opp_url
            .as_ref()
            .map(|url| url.trim_matches(char::is_whitespace).into());

        self.exterior.title = self.exterior.title.trim_matches(char::is_whitespace).into();

        self.exterior.short_desc = ammonia::clean(&self.exterior.short_desc);
        self.exterior.description = ammonia::clean(&self.exterior.description);

        if let None = &self.exterior.location_point {
            if !self.exterior.address_street.is_empty() {
                if let Some(found) = geo::Query::new(
                    format!(
                        "{} {} {} {} {}",
                        self.exterior.address_street,
                        self.exterior.address_city,
                        self.exterior.address_state,
                        self.exterior.address_zip,
                        self.exterior.address_country
                    ),
                    false,
                )
                .lookup_one()
                .await
                {
                    self.exterior.location_point = Some(serde_json::json!({
                        "type": "Point",
                        "coordinates": [found.geometry.longitude, found.geometry.latitude]
                    }));
                }
            }
        }

        if let Some(point) = &self.exterior.location_point {
            let geom = &point["geometry"];
            if geom.is_object() {
                self.exterior.location_point = Some(geom.clone());
            }
        }

        if let Some(poly) = &self.exterior.location_polygon {
            let geom = &poly["geometry"];
            if geom.is_object() {
                self.exterior.location_polygon = Some(geom.clone());
            }
        }

        if let Some(poly) = &self.exterior.location_polygon {
            if poly["type"] == Value::from("Polygon") {
                let new = json!({"type": "MultiPolygon", "coordinates": [poly["coordinates"]]});
                self.exterior.location_polygon = Some(new);
            }
        }

        if self.exterior.partner_name.is_empty() {
            return Err(Error::Missing("partner_name".into()));
        }

        if let (None, Some(dt)) = (self.exterior.partner_created, self.exterior.partner_updated) {
            self.exterior.partner_created = Some(dt.clone());
        }

        if self.exterior.title.is_empty() {
            return Err(Error::Missing("title".into()));
        }

        // if self
        //     .exterior
        //     .partner_opp_url
        //     .map(|url| url.is_empty())
        //     .unwrap_or(true)
        // {
        //     return Err(Error::Missing("partner_opp_url".into()));
        // }

        if self.exterior.uid.is_nil() {
            let namespace = Uuid::new_v5(&PARTNER_NAMESPACE, self.exterior.partner_name.as_ref());

            let mut identifier = self
                .exterior
                .partner_opp_url
                .clone()
                .unwrap_or_else(|| "sciencenearme.org".to_string());
            identifier.push_str("||");
            identifier.push_str(&self.exterior.title);

            self.exterior.uid = Uuid::new_v5(&namespace, identifier.as_ref());
        }

        Ok(())
    }

    pub async fn load_by_id(db: &Database, id: i32) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_id.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_id_with_overlay(db: &Database, id: i32) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_id_with_overlay.sql", id)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid(db: &Database, uid: &Uuid) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_uid_with_overlay(db: &Database, uid: &Uuid) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!(
            "db/opportunity/get_by_uid_with_overlay.sql",
            serde_json::to_value(uid)?
        )
        .fetch_one(db)
        .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn id_by_uid(db: &Database, uid: &Uuid) -> Result<Option<i32>, Error> {
        let rec = sqlx::query_file!("db/opportunity/id_by_uid.sql", serde_json::to_value(uid)?)
            .fetch_optional(db)
            .await?;

        Ok(rec.map(|row| row.id))
    }

    pub async fn exists_by_uid(db: &Database, uid: &Uuid) -> Result<bool, Error> {
        let rec = sqlx::query_file!(
            "db/opportunity/exists_by_uid.sql",
            serde_json::to_value(uid)?
        )
        .fetch_one(db)
        .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn set_id_if_necessary(&mut self, db: &Database) -> Result<(), Error> {
        if let None = self.id {
            self.id = Opportunity::id_by_uid(db, &self.exterior.uid).await?;
        }

        Ok(())
    }

    pub async fn load_by_slug(db: &Database, slug: &str) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_slug.sql", slug)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn load_by_slug_with_overlay(
        db: &Database,
        slug: &str,
    ) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_slug_with_overlay.sql", slug)
            .fetch_one(db)
            .await?;

        Ok(Opportunity {
            id: Some(rec.id),
            exterior: serde_json::from_value(rec.exterior)?,
            interior: serde_json::from_value(rec.interior)?,
        })
    }

    pub async fn id_by_slug(db: &Database, slug: &str) -> Result<Option<i32>, Error> {
        let rec = sqlx::query_file!("db/opportunity/id_by_slug.sql", slug)
            .fetch_optional(db)
            .await?;

        Ok(rec.map(|row| row.id))
    }

    pub async fn uid_by_slug(db: &Database, slug: &str) -> Result<Option<Uuid>, Error> {
        let rec = sqlx::query_file!("db/opportunity/uid_by_slug.sql", slug)
            .fetch_optional(db)
            .await?;

        Ok(rec.map(|row| row.uid).flatten())
    }

    pub async fn exists_by_slug(db: &Database, slug: &str) -> Result<bool, Error> {
        let rec = sqlx::query_file!("db/opportunity/exists_by_slug.sql", slug)
            .fetch_one(db)
            .await?;

        Ok(rec.exists.unwrap_or(false))
    }

    pub async fn set_slug_if_necessary(&mut self, db: &Database) -> Result<(), Error> {
        if self.exterior.slug.is_empty() {
            let base = slugify(&self.exterior.title);
            let mut slug = base.clone();
            let mut disamb = 0u32;

            while Opportunity::exists_by_slug(db, &slug).await? {
                disamb += 1;
                slug = format!("{}-{}", base, disamb);
            }

            self.exterior.slug = slug
        }

        Ok(())
    }

    pub async fn store(&mut self, db: &Database) -> Result<(), Error> {
        self.validate().await?;

        self.set_slug_if_necessary(db).await?;

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

        let overlay = sqlx::query_scalar!(
            r#"SELECT true AS "exists!" FROM c_opportunity_overlay WHERE opportunity_id = $1"#,
            self.id
        )
        .fetch_optional(db)
        .await?;

        if overlay.is_none() {
            let other = sqlx::query!(
                r#"
SELECT v.interior AS "interior!", v.exterior AS "exterior!"
FROM c_opportunity_overlay v JOIN c_opportunity o ON v.opportunity_id = o.id
WHERE
  o.exterior->>'title' = $1 AND
  o.exterior->>'partner' = $2
"#,
                &self.exterior.title,
                self.exterior.partner.to_string(),
            )
            .fetch_optional(db)
            .await?;

            if let Some(other) = other {
                sqlx::query!(
                    r#"
INSERT INTO c_opportunity_overlay (opportunity_id, interior, exterior)
VALUES ($1, $2, $3)
"#,
                    self.id,
                    other.interior,
                    other.exterior,
                )
                .execute(db)
                .await?;
            }
        }

        Ok(())
    }
}
