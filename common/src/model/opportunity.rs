use super::Error;
use crate::Database;

use chrono::{DateTime, FixedOffset, Utc};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgArguments;
use sqlx::query::Query;
use sqlx::{prelude::*, Postgres};
use std::collections::{HashMap, HashSet};
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

impl Default for OrganizationType {
    fn default() -> Self {
        OrganizationType::Unspecified
    }
}

#[derive(Debug, Serialize, Deserialize, EnumIter, EnumString, AsRefStr, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum PageLayout {
    JustContent,
}

impl Default for PageLayout {
    fn default() -> Self {
        PageLayout::JustContent
    }
}

#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct PageOptions {
    pub layout: PageLayout,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(rename_all = "snake_case")]
pub enum EntityType {
    Unspecified,
    Attraction,
    Page(PageOptions),
    #[serde(other)]
    Opportunity,
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
            },
        }
    }
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
    FormalEducation,
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
    pub slug: String,
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
    pub image_credit: String,
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

#[derive(Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
#[serde(rename_all = "kebab-case")]
pub enum OpportunityQueryPhysical {
    InPersonOrOnline,
    InPerson,
    Online,
}

impl Default for OpportunityQueryPhysical {
    fn default() -> Self {
        OpportunityQueryPhysical::InPersonOrOnline
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum OpportunityQueryOrdering {
    Alphabetical,
    Closest,
    Soonest,
    Any,
    Native,
}

impl Default for OpportunityQueryOrdering {
    fn default() -> Self {
        OpportunityQueryOrdering::Alphabetical
    }
}

/// Each field represents one of the database fields by which
/// Opportunity queries can be narrowed. The default value does not
/// narrow the query at all, so to find all of the opportunities with
/// a particular string in the name, we could do something like:
/// ```
/// Opportunity::load_matching(db.acquire().await?, OpportunityQuery { title_contains: "hello".to_string(), ..Default::default() })
/// ```
#[derive(Default, Deserialize, Debug)]
pub struct OpportunityQuery {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub entity_type: Option<Vec<EntityType>>,
    pub title_contains: Option<String>,
    pub tags: Option<Vec<String>>,
    pub topics: Option<Vec<Topic>>,
    pub partner: Option<Uuid>,
    pub partner_member: Option<Uuid>,
    pub near: Option<(f32, f32, f32)>,
    pub physical: Option<OpportunityQueryPhysical>,
    pub text: Option<String>,
    pub beginning: Option<DateTime<FixedOffset>>,
    pub ending: Option<DateTime<FixedOffset>>,
    pub min_age: Option<i16>,
    pub max_age: Option<i16>,
    pub descriptors: Option<Vec<Descriptor>>,
    pub cost: Option<Cost>,
    pub venue_type: Option<VenueType>,
    pub host: Option<String>,
    pub sort: Option<OpportunityQueryOrdering>,
    pub page: Option<u32>,
    pub per_page: Option<u8>,
    pub saved: Option<Uuid>,
    pub participated: Option<Uuid>,
}

#[derive(Debug)]
enum ParamValue {
    // Raw here means it's not converted to JSON before sending it to
    // the database.
    RawString(String),
    RawFloat(f32),
    RawInt(i32),
    RawUuid(Uuid),
    Bool(bool),
    Uuid(Uuid),
    VecString(Vec<String>),
    VecTopic(Vec<Topic>),
    VecEntityType(Vec<EntityType>),
    VecDescriptor(Vec<Descriptor>),
    VecVenueType(Vec<VenueType>),
}

impl ParamValue {
    fn add_to_query(
        self,
        query: Query<Postgres, PgArguments>,
    ) -> Result<Query<Postgres, PgArguments>, Error> {
        Ok(match self {
            ParamValue::RawString(val) => query.bind(val),
            ParamValue::RawFloat(val) => query.bind(val),
            ParamValue::RawInt(val) => query.bind(val),
            ParamValue::RawUuid(val) => query.bind(val),
            ParamValue::Bool(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::Uuid(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecString(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecTopic(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecEntityType(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecDescriptor(val) => query.bind(serde_json::to_value(val)?),
            ParamValue::VecVenueType(val) => query.bind(serde_json::to_value(val)?),
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
    ordering: OpportunityQueryOrdering,
    pagination: Pagination,
) -> Result<(String, Vec<ParamValue>), Error> {
    let mut clauses = Vec::new();
    let mut params = Vec::new();

    // Use *geography* versions of the functions
    // https://postgis.net/docs/ST_DWithin.html
    // https://postgis.net/docs/ST_Distance.html
    // https://postgis.net/docs/ST_Intersects.html

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

    if let Some(person) = query.saved {
        params.push(ParamValue::RawUuid(person));
        clauses.push(format!(
            r"EXISTS (SELECT 1 FROM c_person_bookmark
              WHERE person = ${} AND opportunity = (exterior ->> 'uid')::uuid)",
            params.len()
        ));
    }

    if let Some(person) = query.participated {
        params.push(ParamValue::Uuid(person));
        clauses.push(format!(
            r"EXISTS (SELECT 1 FROM c_involvement AS inv
              WHERE (inv.exterior -> 'opportunity') @> (primary_table.exterior -> 'uid')
              AND (inv.interior -> 'participant') @> ${}::jsonb
              AND (inv.exterior -> 'mode') @> '3'::jsonb)",
            params.len()
        ));
    }

    if let Some(val) = &query.entity_type {
        params.push(ParamValue::VecEntityType(val.clone()));
        clauses.push(format!("(exterior -> 'entity_type') <@ ${}", params.len()));
    }

    if let Some(val) = &query.title_contains {
        params.push(ParamValue::RawString(format!("%{}%", val)));
        clauses.push(format!("(exterior ->> 'title') ILIKE ${}", params.len()));
    }

    if let Some(val) = &query.tags {
        params.push(ParamValue::VecString(val.clone()));
        clauses.push(format!("(exterior -> 'tags') @> ${}", params.len()));
    }

    if let Some(val) = &query.topics {
        params.push(ParamValue::VecTopic(val.clone()));
        clauses.push(format!("(exterior -> 'topics') @> ${}", params.len()));
    }

    if let Some(val) = &query.partner {
        params.push(ParamValue::Uuid(val.clone()));
        clauses.push(format!(
            "(${}::jsonb) @> (exterior -> 'partner')",
            params.len()
        ));
    }

    if let Some(val) = &query.partner_member {
        params.push(ParamValue::Uuid(val.clone()));
        let uuid_param = params.len();
        clauses.push(format!(
            r"jsonb_agg(
                SELECT (exterior -> 'uid') FROM c_partner
                  WHERE (interior -> 'authorized') @> (${}::jsonb)
                  OR (interior -> 'prime') @> (${}::jsonb)
            ) @> (exterior -> 'partner')",
            uuid_param, uuid_param,
        ));
    }

    if let Some(text) = &query.text {
        params.push(ParamValue::RawString(text.to_string()));
        clauses.push(format!(
            "fulltext_english @@ websearch_to_tsquery(${})",
            params.len()
        ));
    }

    if let Some(beginning) = &query.beginning {
        params.push(ParamValue::RawString(beginning.to_rfc3339()));
        let time_param = params.len();
        clauses.push(format!(
            r"(EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > ${}::timestamptz)
              OR
              EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > ${}::timestamptz))",
        time_param, time_param));
    }

    if let Some(ending) = &query.ending {
        params.push(ParamValue::RawString(ending.to_rfc3339()));
        let time_param = params.len();
        clauses.push(format!(
            r"(NOT EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > ${}::timestamptz)
              AND
              NOT EXISTS (SELECT value FROM jsonb_array_elements_text(exterior -> 'end_datetimes') WHERE value::timestamptz > ${}::timestamptz))",
        time_param, time_param));
    }

    if let Some(min_age) = &query.min_age {
        params.push(ParamValue::RawInt(*min_age as i32));
        clauses.push(format!(
            "(exterior -> 'min_age')::integer > ${}",
            params.len()
        ))
    }

    if let Some(max_age) = &query.max_age {
        params.push(ParamValue::RawInt(*max_age as i32));
        clauses.push(format!(
            "(exterior -> 'max_age')::integer < ${}",
            params.len()
        ))
    }

    if let Some(descriptors) = &query.descriptors {
        params.push(ParamValue::VecDescriptor(descriptors.clone()));
        clauses.push(format!("(exterior -> 'descriptors') @> ${}", params.len()))
    }

    if let Some(cost) = &query.cost {
        params.push(ParamValue::RawString(cost.as_ref().to_owned()));
        clauses.push(format!("(exterior ->> 'cost') = ${}", params.len()))
    }

    if let Some(venue_type) = &query.venue_type {
        params.push(ParamValue::VecVenueType(vec![venue_type.clone()]));
        clauses.push(format!("(exterior -> 'opp_venue') @> ${}", params.len()))
    }

    if let Some(host) = &query.host {
        params.push(ParamValue::RawString(format!("%{}%", host)));
        clauses.push(format!(
            "(exterior ->> 'organization_name') ILIKE ${}",
            params.len()
        ))
    }

    if let Some(physical) = &query.physical {
        match physical {
            OpportunityQueryPhysical::InPersonOrOnline => {}
            OpportunityQueryPhysical::InPerson => {
                params.push(ParamValue::Bool(false));
                clauses.push(format!(
                    "(${}::jsonb) @> (exterior -> 'is_online')",
                    params.len()
                ));
            }
            OpportunityQueryPhysical::Online => {
                params.push(ParamValue::Bool(true));
                clauses.push(format!(
                    "(${}::jsonb) @> (exterior -> 'is_online')",
                    params.len()
                ));
            }
        }
    }

    let point = if let Some((longitude, latitude, proximity)) = &query.near {
        params.push(ParamValue::RawFloat(*longitude));
        let lon_param = params.len();
        params.push(ParamValue::RawFloat(*latitude));
        let lat_param = params.len();
        params.push(ParamValue::RawFloat(*proximity));
        let prox_param = params.len();

        clauses.push(format!(
            r#"(
  (exterior ->> 'location_type') = 'any'
  OR
  CASE WHEN location_polygon IS NOT NULL
    THEN ST_Intersects(ST_Buffer(ST_SetSRID(ST_Point(${}, ${}), 4326)::geography, ${}), location_polygon)
    ELSE false END
  OR
  CASE WHEN location_point IS NOT NULL
    THEN ST_Distance(ST_SetSRID(ST_Point(${}, ${}), 4326)::geography, location_point, false) < ${}
    ELSE false END
)"#,
            lon_param, lat_param, prox_param,
            lon_param, lat_param, prox_param
        ));

        Some((lon_param, lat_param))
    } else {
        None
    };

    let mut query_string = "SELECT ".to_string();

    match fields.len() {
        0 => query_string.push_str("*"),
        1 => query_string.push_str(fields[0]),
        _ => query_string.push_str(&fields.join(", ")),
    }

    query_string.push_str(" FROM c_opportunity AS primary_table");

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

    match ordering {
        OpportunityQueryOrdering::Alphabetical => {
            query_string.push_str(" ORDER BY (exterior ->> 'title')")
        }
        OpportunityQueryOrdering::Closest => {
            query_string.push_str(" ORDER BY CASE WHEN location_polygon IS NOT NULL THEN sqrt(ST_Area(location_polygon, false)) / 2");

            if let Some((lon_param, lat_param)) = point {
                query_string
                    .push_str(&format!(" WHEN location_point IS NOT NULL THEN ST_Distance(location_point, ST_SetSRID(ST_Point(${}, ${}), 4326)::geography, false)",
                              lon_param, lat_param));
            }

            // This constant number is roughly the square root of the surface area of the earth, in meters
            query_string.push_str(" ELSE 22585394 END ASC")
        }
        OpportunityQueryOrdering::Soonest => {
            // look for the nearest future start, and fall back to
            // sorting as if it started now if there are none.
            //
            // Where there is no start time, sort it far into the
            // future.
            query_string.push_str(
                r#" ORDER BY CASE
                   WHEN jsonb_array_length(exterior -> 'start_datetimes') > 0
                   THEN COALESCE((SELECT value::timestamptz FROM jsonb_array_elements_text(exterior -> 'start_datetimes') WHERE value::timestamptz > NOW() LIMIT 1), NOW())
                   ELSE '100000-01-01T00:00:00.0+00:00'::timestamptz
                   END ASC"#)
        }
        OpportunityQueryOrdering::Native => query_string.push_str(" ORDER BY id"),
        OpportunityQueryOrdering::Any => {}
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
        .replace_all(source.trim(), "-")
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

impl Opportunity {
    pub async fn count_matching(db: &Database, query: &OpportunityQuery) -> Result<u32, Error> {
        let (query_string, query_params) = build_matching_query(
            &["count(*) as matches"],
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
                "(exterior -> 'uid') as uid",
                "(exterior -> 'slug') as slug",
                "(exterior -> 'title') as title",
                "(exterior -> 'image_url') as image_url",
                "(exterior -> 'short_desc') as short_desc",
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

        self.exterior.description = ammonia::clean(&self.exterior.description);

        if self.exterior.partner_name.is_empty() {
            return Err(Error::Missing("partner_name".into()));
        }

        if let (None, Some(dt)) = (self.exterior.partner_created, self.exterior.partner_updated) {
            self.exterior.partner_created = Some(dt.clone());
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

    pub async fn id_by_slug(db: &Database, slug: &str) -> Result<Option<i32>, Error> {
        let rec = sqlx::query_file!("db/opportunity/id_by_slug.sql", slug)
            .fetch_optional(db)
            .await?;

        Ok(rec.map(|row| row.id))
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
        self.validate()?;

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

        Ok(())
    }
}
