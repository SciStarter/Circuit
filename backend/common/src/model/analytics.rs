use std::collections::BTreeMap;

use chrono::{DateTime, Datelike, Duration, FixedOffset, TimeZone, Utc};
use uuid::Uuid;

use crate::ToFixedOffset;

use super::opportunity::{Descriptor, VenueType};

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, strum::EnumIter)]
pub enum Status {
    #[default]
    #[serde(rename = "Live and Closed")]
    LiveAndClosed,
    Live,
    Closed,
}

impl Status {
    pub const fn discriminate(self) -> i32 {
        match self {
            Status::LiveAndClosed => 0,
            Status::Live => 1,
            Status::Closed => 2,
        }
    }

    pub const fn from_discriminated(discriminated: i32) -> Status {
        match discriminated {
            0 => Status::LiveAndClosed,
            1 => Status::Live,
            2 => Status::Closed,
            _ => Status::LiveAndClosed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AbsoluteTimePeriod {
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, strum::EnumIter)]
pub enum RelativeTimePeriod {
    #[default]
    #[serde(rename = "This Month")]
    ThisMonth,
    #[serde(rename = "Last Month")]
    LastMonth,
    #[serde(rename = "This Quarter")]
    ThisQuarter,
    #[serde(rename = "Last Quarter")]
    LastQuarter,
    #[serde(rename = "This Semiannum")]
    ThisSemiannum,
    #[serde(rename = "Last Semiannum")]
    LastSemiannum,
    #[serde(rename = "This Year")]
    ThisYear,
    #[serde(rename = "Last Year")]
    LastYear,
    #[serde(rename = "All Time")]
    AllTime,
}

impl RelativeTimePeriod {
    pub const fn discriminate(self) -> i32 {
        match self {
            RelativeTimePeriod::ThisMonth => 0,
            RelativeTimePeriod::LastMonth => 1,
            RelativeTimePeriod::ThisQuarter => 2,
            RelativeTimePeriod::LastQuarter => 3,
            RelativeTimePeriod::ThisSemiannum => 4,
            RelativeTimePeriod::LastSemiannum => 5,
            RelativeTimePeriod::ThisYear => 6,
            RelativeTimePeriod::LastYear => 7,
            RelativeTimePeriod::AllTime => 8,
        }
    }

    pub const fn from_discriminated(discriminated: i32) -> RelativeTimePeriod {
        match discriminated {
            0 => RelativeTimePeriod::ThisMonth,
            1 => RelativeTimePeriod::LastMonth,
            2 => RelativeTimePeriod::ThisQuarter,
            3 => RelativeTimePeriod::LastQuarter,
            4 => RelativeTimePeriod::ThisSemiannum,
            5 => RelativeTimePeriod::LastSemiannum,
            6 => RelativeTimePeriod::ThisYear,
            7 => RelativeTimePeriod::LastYear,
            8 => RelativeTimePeriod::AllTime,
            _ => RelativeTimePeriod::ThisMonth,
        }
    }
}

fn beginning_of_month<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let tz = day.timezone();
    tz.with_ymd_and_hms(day.year(), day.month(), 1, 0, 0, 0)
        .earliest()
        .expect("Beginning of the month should not be out of bounds")
}

fn beginning_of_quarter<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let tz = day.timezone();
    tz.with_ymd_and_hms(
        day.year(),
        match day.month() {
            1 | 2 | 3 => 1,
            4 | 5 | 6 => 4,
            7 | 8 | 9 => 7,
            10 | 11 | 12 => 10,
            _ => unreachable!(),
        },
        1,
        0,
        0,
        0,
    )
    .earliest()
    .expect("Beginning of quarter should not be out of bounds")
}

fn beginning_of_semiannum<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    let tz = day.timezone();
    tz.with_ymd_and_hms(
        day.year(),
        match day.month() {
            1 | 2 | 3 | 4 | 5 | 6 => 1,
            7 | 8 | 9 | 10 | 11 | 12 => 7,
            _ => unreachable!(),
        },
        1,
        0,
        0,
        0,
    )
    .earliest()
    .expect("Beginning of siannum should not be out of bounds")
}

fn beginning_of_year<Tz: TimeZone>(day: &DateTime<Tz>) -> DateTime<Tz> {
    day.timezone()
        .with_ymd_and_hms(day.year(), 1, 1, 0, 0, 0)
        .earliest()
        .expect("Beginning of year should not be out of bounds")
}

impl RelativeTimePeriod {
    pub fn absolute(&self) -> AbsoluteTimePeriod {
        let now = Utc::now().to_fixed_offset();
        let day = Duration::days(1);

        match self {
            RelativeTimePeriod::ThisMonth => AbsoluteTimePeriod {
                begin: beginning_of_month(&now),
                end: now,
            },
            RelativeTimePeriod::LastMonth => {
                let month_start = beginning_of_month(&now);
                let last_month_start = beginning_of_month(&(month_start - day));
                AbsoluteTimePeriod {
                    begin: last_month_start,
                    end: month_start,
                }
            }
            RelativeTimePeriod::ThisQuarter => AbsoluteTimePeriod {
                begin: beginning_of_quarter(&now),
                end: now,
            },
            RelativeTimePeriod::LastQuarter => {
                let quarter_start = beginning_of_quarter(&now);
                let last_quarter_start = beginning_of_quarter(&(quarter_start - day));
                AbsoluteTimePeriod {
                    begin: last_quarter_start,
                    end: quarter_start,
                }
            }
            RelativeTimePeriod::ThisSemiannum => AbsoluteTimePeriod {
                begin: beginning_of_semiannum(&now),
                end: now,
            },
            RelativeTimePeriod::LastSemiannum => {
                let semi_start = beginning_of_semiannum(&now);
                let last_semi_start = beginning_of_semiannum(&(semi_start - day));
                AbsoluteTimePeriod {
                    begin: last_semi_start,
                    end: semi_start,
                }
            }
            RelativeTimePeriod::ThisYear => AbsoluteTimePeriod {
                begin: beginning_of_year(&now),
                end: now,
            },
            RelativeTimePeriod::LastYear => {
                let year_start = beginning_of_year(&now);
                let last_year_start = beginning_of_year(&(year_start - day));
                AbsoluteTimePeriod {
                    begin: last_year_start,
                    end: year_start,
                }
            }
            RelativeTimePeriod::AllTime => AbsoluteTimePeriod {
                begin: now
                    .timezone()
                    .with_ymd_and_hms(1, 1, 1, 0, 0, 0)
                    .earliest()
                    .expect("Constant should not be out of bounds"),
                end: now,
            },
        }
    }
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, strum::EnumIter)]
pub enum EngagementType {
    #[default]
    Views,
    Unique,
    #[serde(rename = "Clicks to Website")]
    ClicksToWebsite,
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize, strum::EnumIter)]
pub enum DetailedEngagementType {
    #[default]
    #[serde(rename = "Unique Users")]
    UniqueUsers,
    #[serde(rename = "New Users")]
    NewUsers,
    #[serde(rename = "Returning Users")]
    ReturningUsers,
    #[serde(rename = "Total Pageviews")]
    TotalPageviews,
    #[serde(rename = "Unique Pageviews")]
    UniquePageviews,
    #[serde(rename = "Avg. Time")]
    AvgTime,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DetailedEngagementDataChart {
    pub date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "Unique Users")]
    pub unique_users: u64,
    #[serde(rename = "New Users")]
    pub new_users: u64,
    #[serde(rename = "Returning Users")]
    pub returning_users: u64,
    #[serde(rename = "Total Pageviews")]
    pub total_pageviews: u64,
    #[serde(rename = "Unique Pageviews")]
    pub unique_pageviews: u64,
    #[serde(rename = "Avg. Time")]
    pub average_time: f64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DetailedEngagementDataChartWithPoint {
    #[serde(flatten)]
    pub values: DetailedEngagementDataChart,
    pub point: Option<(f64, f64)>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementDataChart {
    pub date: DateTime<FixedOffset>,
    #[serde(rename = "Views")]
    pub views: u64,
    #[serde(rename = "Unique")]
    pub unique: u64,
    #[serde(rename = "New")]
    pub new: u64,
    #[serde(rename = "Returning")]
    pub returning: u64,
    #[serde(rename = "Clicks to Website")]
    pub clicks: u64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct EngagementDataBar {
    #[serde(rename = "Views")]
    pub views: u64,
    #[serde(rename = "Unique")]
    pub unique: u64,
    #[serde(rename = "Clicks to Website")]
    pub clicks: u64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityEngagementDataBars {
    #[serde(rename = "self")]
    pub self_: EngagementDataBar,
    pub mean: EngagementDataBar,
    pub median: EngagementDataBar,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityEngagementData {
    pub opportunity: Uuid,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub columns: Vec<EngagementType>,
    pub chart: Vec<EngagementDataChart>,
    pub bars: OpportunityEngagementDataBars,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityEngagement {
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OpportunityEngagementData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct RegionEngagement {
    pub max: DetailedEngagementDataChart,
    pub regions: BTreeMap<String, DetailedEngagementDataChartWithPoint>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct StateEngagement {
    #[serde(flatten)]
    pub values: DetailedEngagementDataChart,
    pub regional: RegionEngagement,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityStatesData {
    pub opportunity: Uuid,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub max: DetailedEngagementDataChart,
    pub states: BTreeMap<String, StateEngagement>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityStates {
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OpportunityStatesData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityTechnologyData {
    pub opportunity: Uuid,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub max: DetailedEngagementDataChart,
    pub mobile: DetailedEngagementDataChart,
    pub tablet: DetailedEngagementDataChart,
    pub desktop: DetailedEngagementDataChart,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityTechnology {
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OpportunityTechnologyData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct PieData {
    pub label: String,
    pub hover_offset: i8,
    pub background_color: Vec<String>,
    pub data: Vec<u64>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct PieChart {
    pub labels: Vec<String>,
    pub datasets: Vec<PieData>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct TrafficChart {
    pub name: String,
    pub type_: String,
    #[serde(flatten)]
    pub values: DetailedEngagementDataChart,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityTrafficData {
    pub opportunity: Uuid,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub columns: Vec<EngagementType>,
    pub chart: Vec<EngagementDataChart>,
    pub pie: PieChart,
    pub max: DetailedEngagementDataChart,
    pub table: Vec<TrafficChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityTraffic {
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OpportunityTrafficData,
}

#[derive(Default, Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum OpportunityFormat {
    #[default]
    Event,
    Project,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityOverlapChart {
    pub name: String,
    pub overlap: f64,
    pub host: String,
    pub activity_types: Vec<Descriptor>,
    pub format: String,
    pub venue_types: Vec<VenueType>,
    pub min_age: i16,
    pub max_age: i16,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityOverlapData {
    pub engagement_type: EngagementType,
    pub table: Vec<OpportunityOverlapChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityOverlap {
    pub engagement_types: Vec<EngagementType>,
    pub data: OpportunityOverlapData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Opportunity {
    pub opportunity: Uuid,
    pub updated: DateTime<FixedOffset>,
    pub engagement: OpportunityEngagement,
    pub states: OpportunityStates,
    pub technology: OpportunityTechnology,
    pub traffic: OpportunityTraffic,
    pub overlap: OpportunityOverlap,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OpportunityChart {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub values: EngagementDataChart,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationEngagementData {
    pub organization: Uuid,
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub columns: Vec<EngagementType>,
    pub totals: EngagementDataChart,
    pub max: EngagementDataChart,
    pub chart: Vec<EngagementDataChart>,
    pub table: Vec<OpportunityChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationEngagement {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OrganizationEngagementData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationStatesData {
    pub organization: Uuid,
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub max: DetailedEngagementDataChart,
    pub states: BTreeMap<String, StateEngagement>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationStates {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OrganizationStatesData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationTechnologyData {
    pub organization: Uuid,
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub max: DetailedEngagementDataChart,
    pub mobile: DetailedEngagementDataChart,
    pub tablet: DetailedEngagementDataChart,
    pub desktop: DetailedEngagementDataChart,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationTechnology {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OrganizationTechnologyData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationTrafficData {
    pub organization: Uuid,
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub columns: Vec<EngagementType>,
    pub chart: Vec<EngagementDataChart>,
    pub pie: PieChart,
    pub max: DetailedEngagementDataChart,
    pub table: Vec<TrafficChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OrganizationTraffic {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OrganizationTrafficData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Organization {
    pub organization: Uuid,
    pub name: String,
    pub updated: DateTime<FixedOffset>,
    pub total_opportunities: u32,
    pub current_opportunities: u32,
    pub engagement: OrganizationEngagement,
    pub states: OrganizationStates,
    pub technology: OrganizationTechnology,
    pub traffic: OrganizationTraffic,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HostsDataChart {
    pub name: Option<String>,
    pub total: u64,
    pub live: u64,
    pub views: u64,
    pub opportunity_exits: u64,
    pub didits: u64,
    pub saves: u64,
    pub likes: u64,
    pub shares: u64,
    pub calendar_adds: u64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct HostsData {
    pub total_hosts: u64,
    pub total_opportunities: u64,
    pub max: HostsDataChart,
    pub hosts: Vec<HostsDataChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Hosts {
    pub updated: DateTime<FixedOffset>,
    pub data: HostsData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewEngagementDataStats {
    pub unique_visitors: u64,
    pub accounts: u64,
    pub opportunity_views: u64,
    pub opportunity_unique: u64,
    pub opportunity_exits: u64,
    pub didits: u64,
    pub saves: u64,
    pub likes: u64,
    pub shares: u64,
    pub calendar_adds: u64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewEngagementDataSearch {
    pub phrase: String,
    pub searches: u64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewEngagementData {
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub search_max: u64,
    pub stats: OverviewEngagementDataStats,
    pub searches: Vec<OverviewEngagementDataSearch>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewEngagement {
    pub data: OverviewEngagementData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DemographicComparison {
    pub index: u64,
    pub proportion: f64,
    pub national: f64,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct DemographicComparisonWithSex {
    #[serde(flatten)]
    pub values: DemographicComparison,
    pub male: DemographicComparison,
    pub female: DemographicComparison,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographicsSex {
    pub male: DemographicComparison,
    pub female: DemographicComparison,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographicsAge {
    #[serde(rename = "18-20")]
    pub eighteen_twenty: DemographicComparisonWithSex,
    #[serde(rename = "21-24")]
    pub twentyone_twentyfour: DemographicComparisonWithSex,
    #[serde(rename = "25-29")]
    pub twentyfive_twentynine: DemographicComparisonWithSex,
    #[serde(rename = "30-34")]
    pub thirty_thirtyfour: DemographicComparisonWithSex,
    #[serde(rename = "35-39")]
    pub thirtyfive_thirtynine: DemographicComparisonWithSex,
    #[serde(rename = "40-44")]
    pub forty_fortyfour: DemographicComparisonWithSex,
    #[serde(rename = "45-49")]
    pub fortyfive_fortynine: DemographicComparisonWithSex,
    #[serde(rename = "50-54")]
    pub fifty_fiftyfour: DemographicComparisonWithSex,
    #[serde(rename = "55-59")]
    pub fiftyfive_fiftynine: DemographicComparisonWithSex,
    #[serde(rename = "60-64")]
    pub sixty_sixtyfour: DemographicComparisonWithSex,
    #[serde(rename = "65+")]
    pub sixtyfive_plus: DemographicComparisonWithSex,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographicsEducation {
    #[serde(rename = "No College")]
    pub no_college: DemographicComparison,
    #[serde(rename = "College")]
    pub college: DemographicComparison,
    #[serde(rename = "Grad. Sch.")]
    pub grad_school: DemographicComparison,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographicsIncome {
    #[serde(rename = "$0-50k")]
    pub zero_fifty: DemographicComparison,
    #[serde(rename = "$50-100k")]
    pub fifty_hundred: DemographicComparison,
    #[serde(rename = "$100-150k")]
    pub hundred_hundredfifty: DemographicComparison,
    #[serde(rename = "$150k+")]
    pub hundredfifty_plus: DemographicComparison,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographicsChildren {
    #[serde(rename = "No Children under 17")]
    pub none: DemographicComparison,
    #[serde(rename = "Some Children under 17")]
    pub some: DemographicComparison,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographicsEthnicity {
    #[serde(rename = "Cauc.")]
    pub caucasian: DemographicComparison,
    #[serde(rename = "Afr. Am.")]
    pub african_american: DemographicComparison,
    #[serde(rename = "Asian")]
    pub asian: DemographicComparison,
    #[serde(rename = "Hisp")]
    pub hispanic: DemographicComparison,
    #[serde(rename = "Other")]
    pub other: DemographicComparison,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewDemographics {
    pub sex: OverviewDemographicsSex,
    pub age: OverviewDemographicsAge,
    pub education: OverviewDemographicsEducation,
    pub income: OverviewDemographicsIncome,
    pub children: OverviewDemographicsChildren,
    pub ethnicity: OverviewDemographicsEthnicity,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStatesData {
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub max: DetailedEngagementDataChart,
    pub states: BTreeMap<String, StateEngagement>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewStates {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OverviewStatesData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewTechnologyData {
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub max: DetailedEngagementDataChart,
    pub mobile: DetailedEngagementDataChart,
    pub tablet: DetailedEngagementDataChart,
    pub desktop: DetailedEngagementDataChart,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewTechnology {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OverviewTechnologyData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewTrafficData {
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub begin: DateTime<FixedOffset>,
    pub end: DateTime<FixedOffset>,
    pub columns: Vec<EngagementType>,
    pub chart: Vec<EngagementDataChart>,
    pub pie: PieChart,
    pub max: DetailedEngagementDataChart,
    pub table: Vec<TrafficChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewTraffic {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub data: OverviewTrafficData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewCrossoverDataChartSegment {
    pub proportion: f64,
    pub citizen_science: Option<EngagementDataChart>,
    pub live_science: Option<EngagementDataChart>,
    pub museum_or_science_center: Option<EngagementDataChart>,
    pub maker: Option<EngagementDataChart>,
    pub policy: Option<EngagementDataChart>,
    pub out_of_school_time_program: Option<EngagementDataChart>,
    pub formal_education: Option<EngagementDataChart>,
    pub science_communications: Option<EngagementDataChart>,
    pub unspecified: Option<EngagementDataChart>,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewCrossoverDataChart {
    pub citizen_science: OverviewCrossoverDataChartSegment,
    pub live_science: OverviewCrossoverDataChartSegment,
    pub museum_or_science_center: OverviewCrossoverDataChartSegment,
    pub maker: OverviewCrossoverDataChartSegment,
    pub policy: OverviewCrossoverDataChartSegment,
    pub out_of_school_time_program: OverviewCrossoverDataChartSegment,
    pub formal_education: OverviewCrossoverDataChartSegment,
    pub science_communications: OverviewCrossoverDataChartSegment,
    pub unspecified: OverviewCrossoverDataChartSegment,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewCrossoverData {
    pub opportunity_status: Status,
    pub time_period: RelativeTimePeriod,
    pub engagement_type: EngagementType,
    pub chart: OverviewCrossoverDataChart,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct OverviewCrossover {
    pub opportunity_statuses: Vec<Status>,
    pub time_periods: Vec<RelativeTimePeriod>,
    pub engagement_types: Vec<EngagementType>,
    pub data: OverviewCrossoverData,
}

#[derive(Default, Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Overview {
    pub updated: DateTime<FixedOffset>,
    pub engagement: OverviewEngagement,
    pub demographics: OverviewDemographics,
    pub states: OverviewStates,
    pub technology: OverviewTechnology,
    pub traffic: OverviewTraffic,
    pub crossover: OverviewCrossover,
}
