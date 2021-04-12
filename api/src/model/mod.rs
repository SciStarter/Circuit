use sqlx::{self, postgres::types::PgTimeTz};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "OrganizationType")]
pub enum OrganizationType {
    Unspecified,
    MuseumOrScienceCenter,
    Festival,
    Library,
    CollegeUniversity,
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
}

impl Default for OrganizationType {
    fn default() -> Self {
        OrganizationType::Unspecified
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "EntityType")]
pub enum EntityType {
    Opportunity,
    Attraction,
}

impl Default for EntityType {
    fn default() -> Self {
        EntityType::Opportunity
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "varchar(32)")]
pub enum VenueType {
    Unspecified,
    MuseumOrScienceCenter,
    Library,
    PK12School,
    CommunityOrganization,
    Bar,
    Outdoors,
    CollegeUniversity,
}

impl Default for VenueType {
    fn default() -> Self {
        VenueType::Unspecified
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "OppDomain")]
pub enum Domain {
    Unspecified,
    CitizenScience,
    LiveScience,
    Maker,
    Policy,
    OutOfSchoolTimeProgram,
}

impl Default for Domain {
    fn default() -> Self {
        Domain::Unspecified
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "varchar(32)")]
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
    Createathon,
    Dance,
    Exhibition,
    ExpoStyle,
    Festival,
    Forum,
    Fundraising,
    Hackathon,
    Lecture,
    LiveScience,
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

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "varchar(32)")]
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
    Medicine,
    NatureAndOutdoors,
    OceanWaterMarine,
    Paleontology,
    Physics,
    Policy,
    Psychology,
    Religion,
    Robotics,
    SciencePolicy,
    SocialScience,
    Sound,
    Technology,
    Transportation,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "OpenHours")]
pub struct OpenHours {
    pub opens: PgTimeTz,
    pub closes: PgTimeTz,
}

#[derive(Debug, Default, sqlx::Type)]
#[sqlx(type_name = "OpenDays")]
pub struct OpenDays {
    pub monday: Option<OpenHours>,
    pub tuesday: Option<OpenHours>,
    pub wednesday: Option<OpenHours>,
    pub thursday: Option<OpenHours>,
    pub friday: Option<OpenHours>,
    pub saturday: Option<OpenHours>,
    pub sunday: Option<OpenHours>,
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "Cost")]
pub enum Cost {
    Unknown,
    Free,
    Cost,
}

impl Default for Cost {
    fn default() -> Self {
        Cost::Free
    }
}

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "LocationType")]
pub enum LocationType {
    Unknown,
    Any,
    At,
    Near,
}

impl Default for LocationType {
    fn default() -> Self {
        LocationType::Any
    }
}

// Change to #[derive(Debug, sqlx::Type)] once the workaround is no longer needed
#[derive(Debug, sqlx::Encode)]
#[sqlx(type_name = "Opportunity")]
pub struct Opportunity {
    id: Option<i32>,
    partner_uid: Uuid,
    partner_name: String,
    partner_created: Option<OffsetDateTime>,
    partner_updated: Option<OffsetDateTime>,
    partner_opp_url: String,
    organization_name: String,
    organization_type: OrganizationType,
    organization_website: String,
    entity_type: EntityType,
    min_age: i16,
    max_age: i16,
    pes_domain: Domain,
    tags: Vec<String>,
    ticket_required: bool,
    title: String,
    description: String,
    image_url: String,
    start_dates: Vec<OffsetDateTime>,
    has_end: bool,
    end_dates: Vec<OffsetDateTime>,
    attraction_hours: Option<OpenDays>,
    cost: Cost,
    languages: Vec<String>,
    is_online: bool,
    location_type: LocationType,
    location_name: String,
    location_point: Option<serde_json::Value>,
    location_polygon: Option<serde_json::Value>,
    address_street: String,
    address_city: String,
    address_state: String,
    address_country: String,
    address_zip: String,
    contact_name: String,
    contact_email: String,
    contact_phone: String,
    opp_hashtags: Vec<String>,
    opp_social_handles: serde_json::Value,
    extra_data: serde_json::Value,
}

// Fixes an unfortunate bug related to decoding
// Option<serde_json::Value> from the database
mod workaround;

// #[derive(Debug, Default, sqlx::Type)]
// #[sqlx(type_name = "Person")]
// pub struct Person {}

// #[derive(Debug, Default, sqlx::Type)]
// #[sqlx(type_name = "Participation")]
// pub struct Participation {}
