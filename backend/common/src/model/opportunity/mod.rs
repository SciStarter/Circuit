pub mod for_slug;

use super::person::PermitAction;
use super::serde_helpers::{deserialize_enum, deserialize_enum_vec, serialize_enum, serialize_enum_vec};
use super::Error;
use crate::model::involvement;
use crate::{geo, Database, ToFixedOffset};

use chrono::{DateTime, Duration, FixedOffset, Utc};
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, AsRefStr, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Unspecified,
    Attraction,
    Page(PageOptions),
    #[serde(other)]
    #[default]
    Opportunity,
}

impl EntityType {
    fn _db_repr(&self) -> String {
        self.as_ref().to_snake_case()
    }
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
    fn _db_repr(&self) -> String {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenHours {
    pub opens: String,
    pub closes: String,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
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

#[derive(Default, Clone, Serialize, Deserialize)]
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

#[derive(Clone, Serialize, Deserialize)]
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

#[allow(clippy::too_many_arguments)]
pub fn opportunity_from_row(
    id: i32,
    uid: Uuid,
    slug: String,
    partner_name: String,
    partner_website: Option<String>,
    partner_logo_url: Option<String>,
    partner_created: Option<DateTime<Utc>>,
    partner_updated: Option<DateTime<Utc>>,
    partner_opp_url: Option<String>,
    organization_name: String,
    organization_type: String,
    organization_website: Option<String>,
    organization_logo_url: Option<String>,
    entity_type: String,
    opp_venue: Vec<String>,
    opp_descriptor: Vec<String>,
    min_age: i16,
    max_age: i16,
    pes_domain: String,
    tags: Vec<String>,
    opp_topics: Vec<String>,
    ticket_required: bool,
    title: String,
    description: String,
    short_desc: String,
    image_url: String,
    image_credit: String,
    start_datetimes: Vec<DateTime<Utc>>,
    has_end: bool,
    end_datetimes: Vec<DateTime<Utc>>,
    recurrence: String,
    end_recurrence: Option<DateTime<Utc>>,
    timezone: Option<String>,
    attraction_hours: Option<serde_json::Value>,
    cost: String,
    languages: Vec<String>,
    is_online: bool,
    location_type: String,
    location_name: String,
    location_point_geojson: Option<serde_json::Value>,
    location_polygon_geojson: Option<serde_json::Value>,
    address_street: String,
    address_city: String,
    address_state: String,
    address_country: String,
    address_zip: String,
    opp_hashtags: Vec<String>,
    opp_social_handles: serde_json::Value,
    opp_partner: Uuid,
    accepted: Option<bool>,
    withdrawn: bool,
    submitted_by: Option<Uuid>,
    review_status: String,
    contact_name: String,
    contact_email: String,
    contact_phone: String,
    extra_data: serde_json::Value,
) -> Result<Opportunity, Error> {
    Ok(Opportunity {
        id: Some(id),
        exterior: OpportunityExterior {
            uid,
            slug,
            partner_name,
            partner_website,
            partner_logo_url,
            partner_created: partner_created.map(|dt| dt.to_fixed_offset()),
            partner_updated: partner_updated.map(|dt| dt.to_fixed_offset()),
            partner_opp_url,
            organization_name,
            organization_type: deserialize_enum(&organization_type).unwrap_or_default(),
            organization_website,
            organization_logo_url,
            entity_type: deserialize_enum(&entity_type).unwrap_or_default(),
            opp_venue: deserialize_enum_vec(&opp_venue),
            opp_descriptor: deserialize_enum_vec(&opp_descriptor),
            min_age,
            max_age,
            pes_domain: deserialize_enum(&pes_domain).unwrap_or_default(),
            tags: tags.into_iter().collect(),
            opp_topics: deserialize_enum_vec(&opp_topics),
            ticket_required,
            title,
            description,
            short_desc,
            image_url,
            image_credit,
            start_datetimes: start_datetimes.into_iter().map(|dt| dt.to_fixed_offset()).collect(),
            has_end,
            end_datetimes: end_datetimes.into_iter().map(|dt| dt.to_fixed_offset()).collect(),
            recurrence: deserialize_enum(&recurrence).unwrap_or_default(),
            end_recurrence: end_recurrence.map(|dt| dt.to_fixed_offset()),
            timezone,
            attraction_hours: attraction_hours
                .map(|v| serde_json::from_value(v).unwrap_or_default()),
            cost: deserialize_enum(&cost).unwrap_or_default(),
            languages,
            is_online,
            location_type: deserialize_enum(&location_type).unwrap_or_default(),
            location_name,
            location_point: location_point_geojson,
            location_polygon: location_polygon_geojson,
            address_street,
            address_city,
            address_state,
            address_country,
            address_zip,
            opp_hashtags,
            opp_social_handles: serde_json::from_value(opp_social_handles).unwrap_or_default(),
            partner: opp_partner,
        },
        interior: OpportunityInterior {
            accepted,
            withdrawn,
            submitted_by,
            review_status: deserialize_enum(&review_status).unwrap_or_default(),
            contact_name,
            contact_email,
            contact_phone,
            extra_data,
        },
    })
}

fn apply_overlay(opp: &mut Opportunity, overlay_exterior: Option<serde_json::Value>, overlay_interior: Option<serde_json::Value>) {
    if let Some(ext_overlay) = overlay_exterior {
        if let Ok(mut base) = serde_json::to_value(&opp.exterior) {
            if let serde_json::Value::Object(ref mut base_map) = base {
                if let serde_json::Value::Object(overlay_map) = ext_overlay {
                    for (k, v) in overlay_map {
                        base_map.insert(k, v);
                    }
                }
            }
            if let Ok(merged) = serde_json::from_value(base) {
                opp.exterior = merged;
            }
        }
    }
    if let Some(int_overlay) = overlay_interior {
        if let Ok(mut base) = serde_json::to_value(&opp.interior) {
            if let serde_json::Value::Object(ref mut base_map) = base {
                if let serde_json::Value::Object(overlay_map) = int_overlay {
                    for (k, v) in overlay_map {
                        base_map.insert(k, v);
                    }
                }
            }
            if let Ok(merged) = serde_json::from_value(base) {
                opp.interior = merged;
            }
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
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
    pub include_tags: Option<Vec<String>>,
    pub topics: Option<Vec<Topic>>,
    pub partner: Option<Uuid>,
    pub include_partners: Option<Vec<Uuid>>,
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
    RawVecUuid(Vec<Uuid>),
    RawVenueType(VenueType),
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
            ParamValue::RawVecUuid(val) => query.bind(val),
            ParamValue::RawVecEntityType(val) => query.bind(
                val.into_iter()
                    .map(|x| x.into())
                    .collect::<Vec<DBEntityType>>(),
            ),
            ParamValue::RawVenueType(val) => query.bind(val),
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
        clauses.push(format!(
            "${} = search.uid",
            ParamValue::RawUuid(uid).append(&mut params)
        ));
    }

    if let Some(slug) = &query.slug {
        clauses.push(format!(
            "${} = search.slug",
            ParamValue::RawString(slug.to_string()).append(&mut params)
        ));
    }

    if let Some(val) = query.accepted {
        clauses.push(format!(
            "${} = search.accepted",
            ParamValue::RawBool(val).append(&mut params)
        ));
    }

    if let Some(val) = query.withdrawn {
        if val {
            clauses.push(String::from("(true = coalesce(search.withdrawn, false) OR coalesce(search.review_status, 'not_required') in ('draft', 'pending'))"));
        } else {
            clauses.push(String::from("(false = coalesce(search.withdrawn, false) AND coalesce(search.review_status, 'not_required') not in ('draft', 'pending'))"));
        }
    }

    if let Some(region) = &query.region {
        clauses.push(format!(
            r#"
(SELECT
 COALESCE(
  NULLIF(ST_Intersects(c_region.geometry, search.location_point), false),
  NULLIF(ST_Intersects(c_region.geometry, search.location_polygon), false),
  false
 )
FROM c_region WHERE "name" = ${})
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

        clauses.push(format!(
            r#"(
                (
                 exists (select value from unnest(search.start_datetimes) t(value) where value > ${}::timestamptz and value < ${}::timestamptz)
                 and
                 exists (select value from unnest(search.end_datetimes) t(value) where value > ${}::timestamptz and value < ${}::timestamptz)
                )
                or
                (
                 coalesce(search.end_recurrence, '0001-01-01')::timestamptz > ${}::timestamptz
                )
               )"#,
            begin_param, end_param, begin_param, end_param, begin_param
        ));
    } else {
        if let Some(val) = query.current {
            clauses.push(format!(
                r#"
                ${} = coalesce(
                  coalesce(search.review_status, 'not_required') in ('publish', 'not_required')
                  and
                  search.accepted = true
                  and
                  search.withdrawn = false
                  and
                  (
                    (
                      coalesce(array_length(search.start_datetimes, 1), 0) <= 1
                      and
                      coalesce(array_length(search.end_datetimes, 1), 0) = 0
                    )
                    or
                    exists (select value from unnest(search.start_datetimes) t(value) where value > now())
                    or
                    exists (select value from unnest(search.end_datetimes) t(value) where value > now())
                    or
                    (
                      (search.recurrence = 'daily' or search.recurrence = 'weekly')
                      and
                      (search.end_recurrence is null or search.end_recurrence > now())
                    )
                  )
                , false)"#,
                ParamValue::RawBool(val).append(&mut params)
            ));
        }
    }

    if let Some(person) = query.involved {
        clauses.push(format!(
            r#"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE inv.opportunity = search.uid
              AND inv.participant = ${}
              AND inv.mode >= ${})"#,
            ParamValue::RawUuid(person).append(&mut params),
            ParamValue::RawInt(involvement::Mode::Interest as i32).append(&mut params),
        ));
    }

    if let Some(person) = query.saved {
        clauses.push(format!(
            r#"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE inv.opportunity = search.uid
              AND inv.participant = ${}
              AND inv.mode = ${})"#,
            ParamValue::RawUuid(person).append(&mut params),
            ParamValue::RawInt(involvement::Mode::Saved as i32).append(&mut params),
        ));
    }

    if let Some(person) = query.participated {
        clauses.push(format!(
            r#"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE inv.opportunity = search.uid
              AND inv.participant = ${}
              AND inv.mode >= ${})"#,
            ParamValue::RawUuid(person).append(&mut params),
            ParamValue::RawInt(involvement::Mode::Logged as i32).append(&mut params),
        ));
    }

    if let Some(val) = &query.entity_type {
        clauses.push(format!(
            "search.entity_type = any(${})",
            ParamValue::RawVecEntityType(val.clone()).append(&mut params)
        ));
    }

    if let Some(val) = &query.title_contains {
        clauses.push(format!(
            "search.title ilike ${}",
            ParamValue::RawString(format!("%{}%", val)).append(&mut params)
        ));
    }

    if let Some(val) = &query.tags {
        clauses.push(format!(
            "search.tags && ${}", // In Postgresql when comparing arrays, && means 'is the intersection not empty?'
            ParamValue::RawVecString(val.clone()).append(&mut params)
        ));
    }

    if let Some(val) = &query.topics {
        clauses.push(format!(
            "search.topics && ${}",
            ParamValue::RawVecString(val.iter().map(|v| v.db_repr()).collect()).append(&mut params)
        ));
    }

    if let Some(val) = &query.descriptors {
        clauses.push(format!(
            "search.descriptors && ${}",
            ParamValue::RawVecString(val.iter().map(|v| v.db_repr()).collect()).append(&mut params)
        ));
    }

    if let Some(val) = &query.partner {
        match (&query.include_tags, &query.include_partners) {
            (None, None) => clauses.push(format!(
                "search.partner = ${}",
                ParamValue::RawUuid(*val).append(&mut params)
            )),
            (None, Some(partners)) => clauses.push(format!(
                "(search.partner = ${} or search.partner = any(${}))",
                ParamValue::RawUuid(*val).append(&mut params),
                ParamValue::RawVecUuid(partners.clone()).append(&mut params),
            )),
            (Some(tags), None) => clauses.push(format!(
                "(search.partner = ${} or search.tags && ${})",
                ParamValue::RawUuid(*val).append(&mut params),
                ParamValue::RawVecString(tags.clone()).append(&mut params),
            )),
            (Some(tags), Some(partners)) => clauses.push(format!(
                "(search.partner = ${} or search.partner = any(${}) or search.tags && ${})",
                ParamValue::RawUuid(*val).append(&mut params),
                ParamValue::RawVecUuid(partners.clone()).append(&mut params),
                ParamValue::RawVecString(tags.clone()).append(&mut params),
            )),
        };
    }

    if let Some(val) = &query.partner_member {
        let uuid_param = ParamValue::RawUuid(val.clone()).append(&mut params);
        clauses.push(format!(
            r#"
            (
              (primary_table.submitted_by = ${})
              or
              EXISTS (
                SELECT 1
                FROM c_partner
                WHERE c_partner.uid = search."partner"
                  AND (${} = ANY(c_partner.authorized))
              )
            )"#,
            uuid_param, uuid_param
        ));
    }

    if let Some(text) = &query.text {
        clauses.push(format!(
            "search.fulltext_english @@ websearch_to_tsquery(${})",
            ParamValue::RawString(text.to_string()).append(&mut params)
        ));
    }

    if let Some(beginning) = &query.beginning {
        let time_param = ParamValue::RawString(beginning.to_rfc3339()).append(&mut params);

        clauses.push(format!(
            r#"
            (
              exists(select value from unnest(search.start_datetimes) t(value) where value > ${}::timestamptz)
              or
              exists(select value from unnest(search.end_datetimes) t(value) where value > ${}::timestamptz)
              or (
                (search.recurrence = 'daily' or search.recurrence = 'weekly')
                and
                (search.end_recurrence is null or search.end_recurrence > ${}::timestamptz)
              )
              or (
               coalesce(array_length(search.start_datetimes, 1), 0) <= 1
               and
               coalesce(array_length(search.end_datetimes, 1), 0) = 0
              )
            )"#,
            time_param, time_param, time_param
        ));
    }

    if let Some(ending) = &query.ending {
        let time_param = ParamValue::RawString(ending.to_rfc3339()).append(&mut params);

        clauses.push(format!(
            r#"
            (
              not exists (select value from unnest(search.start_datetimes) t(value) where value > ${}::timestamptz)
              and
              not exists (select value from unnest(search.end_datetimes) t(value) where value > ${}::timestamptz)
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
        clauses.push(format!(
            "search.max_age >= ${}",
            ParamValue::RawInt(*min_age as i32).append(&mut params)
        ));
    }

    if let Some(max_age) = &query.max_age {
        clauses.push(format!(
            "search.min_age <= ${}",
            ParamValue::RawInt(*max_age as i32).append(&mut params)
        ));
    }

    if query.kids_only.unwrap_or(false) {
        clauses.push("search.max_age <= 18".to_string())
    }

    if query.adults_only.unwrap_or(false) {
        clauses.push("search.min_age >= 21".to_string())
    }

    if let Some(cost) = &query.cost {
        clauses.push(format!(
            "search.cost = ${}",
            ParamValue::RawString(cost.db_repr()).append(&mut params)
        ));
    }

    if let Some(venue_type) = &query.venue_type {
        clauses.push(format!(
            "any(search.venue_type) = ${}",
            ParamValue::RawVenueType(venue_type.clone()).append(&mut params)
        ));
    }

    if let Some(host) = &query.host {
        clauses.push(format!(
            "search.organization_name ilike ${}",
            ParamValue::RawString(host.to_string()).append(&mut params)
        ));
    }

    if let Some(physical) = &query.physical {
        match physical {
            OpportunityQueryPhysical::InPersonOrOnline => {}
            OpportunityQueryPhysical::InPerson => {
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
                clauses.push(String::from(
                    r#"
                    (coalesce(array_length(search.start_datetimes, 1), 0) > 1
                     or
                     coalesce(array_length(search.end_datetimes, 1), 0) > 1
                     or (
                       coalesce(array_length(search.start_datetimes, 1), 0) = 1
                       and
                       coalesce(array_length(search.end_datetimes, 1), 0) = 1
                       and
                       age(search.end_datetimes[1], search.start_datetimes[1]) <= interval '7 days'
                     )
                    )"#,
                ));
            }
            OpportunityQueryTemporal::OnDemand => {
                clauses.push(String::from(
                    r#"
                    coalesce(array_length(search.start_datetimes, 1), 0) <= 1
                    and
                    coalesce(array_length(search.end_datetimes, 1), 0) <= 1
                    and (
                      coalesce(array_length(search.start_datetimes, 1), 0) != 1
                      or
                      coalesce(array_length(search.end_datetimes, 1), 0) != 1
                      or
                      age(search.end_datetimes[1], search.start_datetimes[1]) > interval '7 days'
                    )"#,
                ));
            }
        }
    }

    if let Some(probability) = query.sample {
        clauses.push(format!(
            "random() < ${}",
            ParamValue::RawFloat(probability).append(&mut params)
        ));
    }

    if let Some(exclusions) = &query.exclude {
        clauses.push(format!(
            "NOT (search.uid = any(${}))",
            ParamValue::RawVecUuid(exclusions.clone()).append(&mut params)
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

    query_string.push_str(
        r#" FROM c_opportunity_search search JOIN (
                    SELECT
                        c_opportunity.*,
                        c_opportunity_overlay.exterior AS overlay_exterior,
                        c_opportunity_overlay.interior AS overlay_interior
                    "#,
    );

    if let Some(uid) = &query.prefer_partner {
        query_string.push_str(&format!(
            ", (c_opportunity.opp_partner = ${})::int AS _sort_preferential",
            ParamValue::RawUuid(uid.clone()).append(&mut params)
        ));
    } else {
        query_string.push_str(", 0 as _sort_preferential");
    }

    if let Some((longitude, latitude, proximity)) = &query.near {
        let lon_param = ParamValue::RawFloat(*longitude).append(&mut params);
        let lat_param = ParamValue::RawFloat(*latitude).append(&mut params);
        let prox_param = ParamValue::RawFloat(*proximity).append(&mut params);

        query_string.push_str(", CASE WHEN c_opportunity.location_type = 'any' OR c_opportunity.is_online = true THEN 2 ELSE 1 END AS _sort_location_priority");

        query_string.push_str(", CASE");

        query_string.push_str(
            &format!(" WHEN c_opportunity.location_polygon IS NOT NULL THEN ST_Distance(c_opportunity.location_polygon, ST_SetSRID(ST_Point(${lon_param}, ${lat_param}), 4326)::geography, false)")
        );

        query_string
            .push_str(&format!(" WHEN c_opportunity.location_point IS NOT NULL THEN ST_Distance(c_opportunity.location_point, ST_SetSRID(ST_Point(${lon_param}, ${lat_param}), 4326)::geography, false)"));

        query_string.push_str(&format!(" WHEN c_opportunity.location_type = 'any' OR c_opportunity.is_online = true THEN ${prox_param}"));

        // This constant number is roughly the square root of the surface area of the earth, in meters, i.e. about as far away as you can get
        query_string.push_str(" ELSE 22585394 END AS _sort_distance");

        if *proximity > 0.0 {
            clauses.push(format!("(_sort_distance < 1.1 * ${prox_param})"));
        }
    } else {
        query_string.push_str(", CASE WHEN c_opportunity.location_type = 'any' OR c_opportunity.is_online = true THEN 0 ELSE 1 END AS _sort_location_priority");
        query_string.push_str(", 1 AS _sort_distance");
    }

    query_string.push_str(", CASE WHEN c_opportunity.location_polygon IS NOT NULL THEN ST_Area(c_opportunity.location_polygon, false) ELSE 0 END AS _sort_area");

    // We bump ongoing opportunities so that they sort as a week in the future, to give actual timely opportunities priority
    query_string.push_str(r#",
            CASE
              WHEN coalesce(array_length(c_opportunity.start_datetimes, 1), 0) = 0 AND coalesce(array_length(c_opportunity.end_datetimes, 1), 0) = 0
              THEN CURRENT_TIMESTAMP + INTERVAL '7 days'
              WHEN EXISTS (SELECT 1 FROM unnest(c_opportunity.start_datetimes) t(value) WHERE value > CURRENT_TIMESTAMP)
              THEN (SELECT MIN(value) FROM unnest(c_opportunity.start_datetimes) t(value) WHERE value > CURRENT_TIMESTAMP LIMIT 1)
              WHEN EXISTS (SELECT 1 FROM unnest(c_opportunity.end_datetimes) t(value) WHERE value > CURRENT_TIMESTAMP)
              THEN CURRENT_TIMESTAMP + INTERVAL '7 days'
              ELSE '100000-01-01T00:00:00.0+00:00'::timestamptz
            END AS _sort_time
        "#);

    query_string.push_str(" FROM c_opportunity LEFT JOIN c_opportunity_overlay ON c_opportunity.id = c_opportunity_overlay.opportunity_id) AS primary_table ON search.opp_id = primary_table.id");

    if ordering == OpportunityQueryOrdering::PartnerName {
        query_string.push_str(
            " LEFT JOIN c_partner ON search.partner = c_partner.uid",
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
            query_string.push_str(r#" ORDER BY c_partner."name" ASC, search.title ASC"#)
        }
    }

    match pagination {
        Pagination::All => query_string.push_str(";"),
        Pagination::One => query_string.push_str(" LIMIT 1;"),
        Pagination::Page { index, size } => {
            query_string.push_str(format!(" LIMIT {} OFFSET {};", size, index * size).as_ref())
        }
    };

    println!("Constructed query: {}", &query_string);
    println!("Constructed params: {:?}", &params);

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
            uids: sqlx::query!(r#"SELECT uid AS "uid!" FROM c_opportunity"#)
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
                "primary_table.uid",
                "primary_table.slug",
                "primary_table.title",
                "primary_table.image_url",
                "primary_table.short_desc",
            ],
            query,
            ordering,
            pagination,
        )?;

        let query_obj = ParamValue::add_all_to_query(query_params, sqlx::query(&query_string))?;

        query_obj
            .map(|rec| {
                Ok(OpportunityReference {
                    uid: rec.get("uid"),
                    slug: rec.try_get("slug").unwrap_or_default(),
                    title: rec.get("title"),
                    image_url: rec.get("image_url"),
                    short_desc: rec.get("short_desc"),
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
                let mut opp = opportunity_from_row(
                    rec.get("id"),
                    rec.get("uid"),
                    rec.get("slug"),
                    rec.get("partner_name"),
                    rec.get("partner_website"),
                    rec.get("partner_logo_url"),
                    rec.get("partner_created"),
                    rec.get("partner_updated"),
                    rec.get("partner_opp_url"),
                    rec.get("organization_name"),
                    rec.get("organization_type"),
                    rec.get("organization_website"),
                    rec.get("organization_logo_url"),
                    rec.get("entity_type"),
                    rec.get("opp_venue"),
                    rec.get("opp_descriptor"),
                    rec.get("min_age"),
                    rec.get("max_age"),
                    rec.get("pes_domain"),
                    rec.get("tags"),
                    rec.get("opp_topics"),
                    rec.get("ticket_required"),
                    rec.get("title"),
                    rec.get("description"),
                    rec.get("short_desc"),
                    rec.get("image_url"),
                    rec.get("image_credit"),
                    rec.get("start_datetimes"),
                    rec.get("has_end"),
                    rec.get("end_datetimes"),
                    rec.get("recurrence"),
                    rec.get("end_recurrence"),
                    rec.get("timezone"),
                    rec.get("attraction_hours"),
                    rec.get("cost"),
                    rec.get("languages"),
                    rec.get("is_online"),
                    rec.get("location_type"),
                    rec.get("location_name"),
                    rec.get("location_point_geojson"),
                    rec.get("location_polygon_geojson"),
                    rec.get("address_street"),
                    rec.get("address_city"),
                    rec.get("address_state"),
                    rec.get("address_country"),
                    rec.get("address_zip"),
                    rec.get("opp_hashtags"),
                    rec.get("opp_social_handles"),
                    rec.get("opp_partner"),
                    rec.get("accepted"),
                    rec.get("withdrawn"),
                    rec.get("submitted_by"),
                    rec.get("review_status"),
                    rec.get("contact_name"),
                    rec.get("contact_email"),
                    rec.get("contact_phone"),
                    rec.get("extra_data"),
                )?;
                let overlay_ext: Option<serde_json::Value> = rec.get("overlay_exterior");
                let overlay_int: Option<serde_json::Value> = rec.get("overlay_interior");
                apply_overlay(&mut opp, overlay_ext, overlay_int);
                Ok(opp)
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

        opportunity_from_row(
            rec.id, rec.uid, rec.slug, rec.partner_name, rec.partner_website, rec.partner_logo_url,
            rec.partner_created, rec.partner_updated, rec.partner_opp_url,
            rec.organization_name, rec.organization_type, rec.organization_website, rec.organization_logo_url,
            rec.entity_type, rec.opp_venue, rec.opp_descriptor, rec.min_age, rec.max_age, rec.pes_domain,
            rec.tags, rec.opp_topics, rec.ticket_required,
            rec.title, rec.description, rec.short_desc, rec.image_url, rec.image_credit,
            rec.start_datetimes, rec.has_end, rec.end_datetimes, rec.recurrence, rec.end_recurrence, rec.timezone,
            rec.attraction_hours, rec.cost, rec.languages, rec.is_online,
            rec.location_type, rec.location_name, rec.location_point_geojson, rec.location_polygon_geojson,
            rec.address_street, rec.address_city, rec.address_state, rec.address_country, rec.address_zip,
            rec.opp_hashtags, rec.opp_social_handles, rec.opp_partner,
            rec.accepted, rec.withdrawn, rec.submitted_by, rec.review_status,
            rec.contact_name, rec.contact_email, rec.contact_phone, rec.extra_data,
        )
    }

    pub async fn load_by_id_with_overlay(db: &Database, id: i32) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_id_with_overlay.sql", id)
            .fetch_one(db)
            .await?;

        let mut opp = opportunity_from_row(
            rec.id, rec.uid, rec.slug, rec.partner_name, rec.partner_website, rec.partner_logo_url,
            rec.partner_created, rec.partner_updated, rec.partner_opp_url,
            rec.organization_name, rec.organization_type, rec.organization_website, rec.organization_logo_url,
            rec.entity_type, rec.opp_venue, rec.opp_descriptor, rec.min_age, rec.max_age, rec.pes_domain,
            rec.tags, rec.opp_topics, rec.ticket_required,
            rec.title, rec.description, rec.short_desc, rec.image_url, rec.image_credit,
            rec.start_datetimes, rec.has_end, rec.end_datetimes, rec.recurrence, rec.end_recurrence, rec.timezone,
            rec.attraction_hours, rec.cost, rec.languages, rec.is_online,
            rec.location_type, rec.location_name, rec.location_point_geojson, rec.location_polygon_geojson,
            rec.address_street, rec.address_city, rec.address_state, rec.address_country, rec.address_zip,
            rec.opp_hashtags, rec.opp_social_handles, rec.opp_partner,
            rec.accepted, rec.withdrawn, rec.submitted_by, rec.review_status,
            rec.contact_name, rec.contact_email, rec.contact_phone, rec.extra_data,
        )?;
        apply_overlay(&mut opp, rec.overlay_exterior, rec.overlay_interior);
        Ok(opp)
    }

    pub async fn load_by_uid(db: &Database, uid: &Uuid) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_uid.sql", uid)
            .fetch_one(db)
            .await?;

        opportunity_from_row(
            rec.id, rec.uid, rec.slug, rec.partner_name, rec.partner_website, rec.partner_logo_url,
            rec.partner_created, rec.partner_updated, rec.partner_opp_url,
            rec.organization_name, rec.organization_type, rec.organization_website, rec.organization_logo_url,
            rec.entity_type, rec.opp_venue, rec.opp_descriptor, rec.min_age, rec.max_age, rec.pes_domain,
            rec.tags, rec.opp_topics, rec.ticket_required,
            rec.title, rec.description, rec.short_desc, rec.image_url, rec.image_credit,
            rec.start_datetimes, rec.has_end, rec.end_datetimes, rec.recurrence, rec.end_recurrence, rec.timezone,
            rec.attraction_hours, rec.cost, rec.languages, rec.is_online,
            rec.location_type, rec.location_name, rec.location_point_geojson, rec.location_polygon_geojson,
            rec.address_street, rec.address_city, rec.address_state, rec.address_country, rec.address_zip,
            rec.opp_hashtags, rec.opp_social_handles, rec.opp_partner,
            rec.accepted, rec.withdrawn, rec.submitted_by, rec.review_status,
            rec.contact_name, rec.contact_email, rec.contact_phone, rec.extra_data,
        )
    }

    pub async fn load_by_uid_with_overlay(db: &Database, uid: &Uuid) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!(
            "db/opportunity/get_by_uid_with_overlay.sql",
            uid
        )
        .fetch_one(db)
        .await?;

        let mut opp = opportunity_from_row(
            rec.id, rec.uid, rec.slug, rec.partner_name, rec.partner_website, rec.partner_logo_url,
            rec.partner_created, rec.partner_updated, rec.partner_opp_url,
            rec.organization_name, rec.organization_type, rec.organization_website, rec.organization_logo_url,
            rec.entity_type, rec.opp_venue, rec.opp_descriptor, rec.min_age, rec.max_age, rec.pes_domain,
            rec.tags, rec.opp_topics, rec.ticket_required,
            rec.title, rec.description, rec.short_desc, rec.image_url, rec.image_credit,
            rec.start_datetimes, rec.has_end, rec.end_datetimes, rec.recurrence, rec.end_recurrence, rec.timezone,
            rec.attraction_hours, rec.cost, rec.languages, rec.is_online,
            rec.location_type, rec.location_name, rec.location_point_geojson, rec.location_polygon_geojson,
            rec.address_street, rec.address_city, rec.address_state, rec.address_country, rec.address_zip,
            rec.opp_hashtags, rec.opp_social_handles, rec.opp_partner,
            rec.accepted, rec.withdrawn, rec.submitted_by, rec.review_status,
            rec.contact_name, rec.contact_email, rec.contact_phone, rec.extra_data,
        )?;
        apply_overlay(&mut opp, rec.overlay_exterior, rec.overlay_interior);
        Ok(opp)
    }

    pub async fn id_by_uid(db: &Database, uid: &Uuid) -> Result<Option<i32>, Error> {
        let rec = sqlx::query_file!("db/opportunity/id_by_uid.sql", uid)
            .fetch_optional(db)
            .await?;

        Ok(rec.map(|row| row.id))
    }

    pub async fn exists_by_uid(db: &Database, uid: &Uuid) -> Result<bool, Error> {
        let rec = sqlx::query_file!(
            "db/opportunity/exists_by_uid.sql",
            uid
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

        opportunity_from_row(
            rec.id, rec.uid, rec.slug, rec.partner_name, rec.partner_website, rec.partner_logo_url,
            rec.partner_created, rec.partner_updated, rec.partner_opp_url,
            rec.organization_name, rec.organization_type, rec.organization_website, rec.organization_logo_url,
            rec.entity_type, rec.opp_venue, rec.opp_descriptor, rec.min_age, rec.max_age, rec.pes_domain,
            rec.tags, rec.opp_topics, rec.ticket_required,
            rec.title, rec.description, rec.short_desc, rec.image_url, rec.image_credit,
            rec.start_datetimes, rec.has_end, rec.end_datetimes, rec.recurrence, rec.end_recurrence, rec.timezone,
            rec.attraction_hours, rec.cost, rec.languages, rec.is_online,
            rec.location_type, rec.location_name, rec.location_point_geojson, rec.location_polygon_geojson,
            rec.address_street, rec.address_city, rec.address_state, rec.address_country, rec.address_zip,
            rec.opp_hashtags, rec.opp_social_handles, rec.opp_partner,
            rec.accepted, rec.withdrawn, rec.submitted_by, rec.review_status,
            rec.contact_name, rec.contact_email, rec.contact_phone, rec.extra_data,
        )
    }

    pub async fn load_by_slug_with_overlay(
        db: &Database,
        slug: &str,
    ) -> Result<Opportunity, Error> {
        let rec = sqlx::query_file!("db/opportunity/get_by_slug_with_overlay.sql", slug)
            .fetch_one(db)
            .await?;

        let mut opp = opportunity_from_row(
            rec.id, rec.uid, rec.slug, rec.partner_name, rec.partner_website, rec.partner_logo_url,
            rec.partner_created, rec.partner_updated, rec.partner_opp_url,
            rec.organization_name, rec.organization_type, rec.organization_website, rec.organization_logo_url,
            rec.entity_type, rec.opp_venue, rec.opp_descriptor, rec.min_age, rec.max_age, rec.pes_domain,
            rec.tags, rec.opp_topics, rec.ticket_required,
            rec.title, rec.description, rec.short_desc, rec.image_url, rec.image_credit,
            rec.start_datetimes, rec.has_end, rec.end_datetimes, rec.recurrence, rec.end_recurrence, rec.timezone,
            rec.attraction_hours, rec.cost, rec.languages, rec.is_online,
            rec.location_type, rec.location_name, rec.location_point_geojson, rec.location_polygon_geojson,
            rec.address_street, rec.address_city, rec.address_state, rec.address_country, rec.address_zip,
            rec.opp_hashtags, rec.opp_social_handles, rec.opp_partner,
            rec.accepted, rec.withdrawn, rec.submitted_by, rec.review_status,
            rec.contact_name, rec.contact_email, rec.contact_phone, rec.extra_data,
        )?;
        apply_overlay(&mut opp, rec.overlay_exterior, rec.overlay_interior);
        Ok(opp)
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

        Ok(rec.map(|row| row.uid))
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

        let entity_type = serialize_enum(&self.exterior.entity_type);
        let opp_venue: Vec<String> = serialize_enum_vec(&self.exterior.opp_venue);
        let opp_descriptor: Vec<String> = serialize_enum_vec(&self.exterior.opp_descriptor);
        let pes_domain = serialize_enum(&self.exterior.pes_domain);
        let tags: Vec<String> = self.exterior.tags.iter().cloned().collect();
        let opp_topics: Vec<String> = serialize_enum_vec(&self.exterior.opp_topics);
        let start_datetimes: Vec<DateTime<FixedOffset>> = self.exterior.start_datetimes.clone();
        let end_datetimes: Vec<DateTime<FixedOffset>> = self.exterior.end_datetimes.clone();
        let recurrence = serialize_enum(&self.exterior.recurrence);
        let attraction_hours = self.exterior.attraction_hours.as_ref()
            .map(|h| serde_json::to_value(h).unwrap_or_default());
        let cost = serialize_enum(&self.exterior.cost);
        let organization_type = serialize_enum(&self.exterior.organization_type);
        let location_type = serialize_enum(&self.exterior.location_type);
        let opp_social_handles = serde_json::to_value(&self.exterior.opp_social_handles)?;
        let review_status = serialize_enum(&self.interior.review_status);
        let extra_data = &self.interior.extra_data;

        if let Some(id) = self.id {
            sqlx::query_file!(
                "db/opportunity/update.sql",
                id,
                self.exterior.uid,
                self.exterior.slug,
                self.exterior.partner_name,
                self.exterior.partner_website,
                self.exterior.partner_logo_url,
                self.exterior.partner_created,
                self.exterior.partner_updated,
                self.exterior.partner_opp_url,
                self.exterior.organization_name,
                organization_type,
                self.exterior.organization_website,
                self.exterior.organization_logo_url,
                entity_type,
                &opp_venue as &[String],
                &opp_descriptor as &[String],
                self.exterior.min_age,
                self.exterior.max_age,
                pes_domain,
                &tags as &[String],
                &opp_topics as &[String],
                self.exterior.ticket_required,
                self.exterior.title,
                self.exterior.description,
                self.exterior.short_desc,
                self.exterior.image_url,
                self.exterior.image_credit,
                &start_datetimes as &[DateTime<FixedOffset>],
                self.exterior.has_end,
                &end_datetimes as &[DateTime<FixedOffset>],
                recurrence,
                self.exterior.end_recurrence,
                self.exterior.timezone,
                attraction_hours as Option<serde_json::Value>,
                cost,
                &self.exterior.languages as &[String],
                self.exterior.is_online,
                location_type,
                self.exterior.location_name,
                self.exterior.location_point.clone() as Option<serde_json::Value>,
                self.exterior.location_polygon.clone() as Option<serde_json::Value>,
                self.exterior.address_street,
                self.exterior.address_city,
                self.exterior.address_state,
                self.exterior.address_country,
                self.exterior.address_zip,
                &self.exterior.opp_hashtags as &[String],
                opp_social_handles,
                self.exterior.partner,
                self.interior.accepted,
                self.interior.withdrawn,
                self.interior.submitted_by,
                review_status,
                self.interior.contact_name,
                self.interior.contact_email,
                self.interior.contact_phone,
                extra_data,
            )
            .execute(db)
            .await?;
        } else {
            let rec = sqlx::query_file!(
                "db/opportunity/insert.sql",
                self.exterior.uid,
                self.exterior.slug,
                self.exterior.partner_name,
                self.exterior.partner_website,
                self.exterior.partner_logo_url,
                self.exterior.partner_created,
                self.exterior.partner_updated,
                self.exterior.partner_opp_url,
                self.exterior.organization_name,
                organization_type,
                self.exterior.organization_website,
                self.exterior.organization_logo_url,
                entity_type,
                &opp_venue as &[String],
                &opp_descriptor as &[String],
                self.exterior.min_age,
                self.exterior.max_age,
                pes_domain,
                &tags as &[String],
                &opp_topics as &[String],
                self.exterior.ticket_required,
                self.exterior.title,
                self.exterior.description,
                self.exterior.short_desc,
                self.exterior.image_url,
                self.exterior.image_credit,
                &start_datetimes as &[DateTime<FixedOffset>],
                self.exterior.has_end,
                &end_datetimes as &[DateTime<FixedOffset>],
                recurrence,
                self.exterior.end_recurrence,
                self.exterior.timezone,
                attraction_hours as Option<serde_json::Value>,
                cost,
                &self.exterior.languages as &[String],
                self.exterior.is_online,
                location_type,
                self.exterior.location_name,
                self.exterior.location_point.clone() as Option<serde_json::Value>,
                self.exterior.location_polygon.clone() as Option<serde_json::Value>,
                self.exterior.address_street,
                self.exterior.address_city,
                self.exterior.address_state,
                self.exterior.address_country,
                self.exterior.address_zip,
                &self.exterior.opp_hashtags as &[String],
                opp_social_handles,
                self.exterior.partner,
                self.interior.accepted,
                self.interior.withdrawn,
                self.interior.submitted_by,
                review_status,
                self.interior.contact_name,
                self.interior.contact_email,
                self.interior.contact_phone,
                extra_data,
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
  o.title = $1 AND
  o.opp_partner = $2
"#,
                &self.exterior.title,
                self.exterior.partner,
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
