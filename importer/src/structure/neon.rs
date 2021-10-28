use super::{
    Error,
    OneOrMany::{self, Many},
    PartnerInfo, Structure,
};
use common::model::Opportunity;
use serde::de::{Deserialize, Unexpected};
use serde_json::{from_value, Value};

fn bool_from_intstr<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "0" => Ok(false),
        "1" => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Str(other),
            &"zero or one",
        )),
    }
}

fn bool_from_yesno<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    match String::deserialize(deserializer)?.as_ref() {
        "No" => Ok(false),
        "Yes" => Ok(true),
        other => Err(serde::de::Error::invalid_value(
            Unexpected::Str(other),
            &"zero or one",
        )),
    }
}

#[derive(serde::Deserialize, Debug, Default)]
#[serde(default)]
struct NeonStandardFields {
    #[serde(rename = "Address Line 1")]
    address_line_1: String,
    #[serde(rename = "Address Line 2")]
    address_line_2: String,
    #[serde(rename = "Address Line 3")]
    address_line_3: String,
    #[serde(rename = "Address Line 4")]
    address_line_4: String,
    #[serde(rename = "Address Type")]
    address_type: String, // "Other"
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "Country")]
    country: String,
    #[serde(rename = "Declined")]
    #[serde(deserialize_with = "bool_from_intstr")]
    declined: bool, // "0"
    #[serde(rename = "Deferred")]
    #[serde(deserialize_with = "bool_from_intstr")]
    deferred: bool, // "0"
    #[serde(rename = "Event Admission Fee")]
    event_admission_fee: Option<String>, // null
    #[serde(rename = "Event Archive")]
    #[serde(deserialize_with = "bool_from_yesno")]
    event_archive: bool, // "No"
    #[serde(rename = "Event Capacity")]
    event_capacity: String, // "12"
    #[serde(rename = "Event Category Name")]
    event_category_name: String, // "Summer Camps"
    #[serde(rename = "Event Created Date/Time")]
    event_created: String, // "2021-03-01 21:48:05.0"
    #[serde(rename = "Event Description")]
    event_description: String, // HTML
    #[serde(rename = "Event End Date")]
    event_end_date: String, // "2021-08-13"
    #[serde(rename = "Event End Time")]
    event_end_time: String, // "15:00:01"
    #[serde(rename = "Event External URL")]
    event_external_url: Option<String>, // null
    #[serde(rename = "Event ID")]
    event_id: String, // "913"
    #[serde(rename = "Event Last Modified Date/Time")]
    event_updated: String, // "2021-06-24 22:29:42.0"
    #[serde(rename = "Event Location Name")]
    event_location_name: String, // "ScienceWorks Hands-On Museum"
    #[serde(rename = "Event Name")]
    event_name: String, // "STEM Story Telling: Storybook Engineers MM"
    #[serde(rename = "Event Note")]
    event_note: Option<String>, // null
    #[serde(rename = "Event Registration Note")]
    event_registration_node: Option<String>, // null
    #[serde(rename = "Event Start Date")]
    event_start_date: String, // "2021-08-16"
    #[serde(rename = "Event Start Time")]
    event_start_time: String, // "09:00:01"
    #[serde(rename = "Event Summary")]
    event_summary: String, // text
    #[serde(rename = "Event Topic")]
    event_topic: String, // "Summer Camp"
    #[serde(rename = "Event Web Publish")]
    #[serde(deserialize_with = "bool_from_yesno")]
    event_web_publish: bool, // "Yes"
    #[serde(rename = "Fee Type")]
    fee_type: String, // "Multiple ticket prices, one attendee per ticket"
    #[serde(rename = "Full Street Address (F)")]
    full_street_address: String, // "1500 E. Main Street"
    #[serde(rename = "Full Zip Code (F)")]
    full_zip_code: String, // "97520"
    #[serde(rename = "Phone 1 Full Number (F)")]
    full_phone_number: String, // ""
    #[serde(rename = "State/Province")]
    state_province: String, // "OR"
    #[serde(rename = "Zip Code")]
    zip_code: String, // "97520"
}

#[derive(Debug)]
struct NeonGeneric<Tz: chrono::TimeZone + std::fmt::Debug>(PartnerInfo<Tz>, Tz);

fn interpret_one<Tz: chrono::TimeZone + std::fmt::Debug>(
    partner: &PartnerInfo<Tz>,
    _tz: &Tz,
    entry: Value,
) -> Option<Opportunity> {
    let data: NeonStandardFields = match from_value(entry) {
        Ok(d) => d,
        Err(err) => {
            println!("Warning: Entry could not be parsed: {:?}", err);
            return None;
        }
    };

    let mut opp = Opportunity::default();

    opp.exterior.partner = partner.partner.clone();

    opp.exterior.partner_name = partner.partner_name.clone();

    opp.exterior.partner_website = partner.partner_website.clone();

    opp.exterior.partner_logo_url = partner.partner_logo_url.clone();

    opp.exterior.uid = uuid::Uuid::new_v5(&partner.partner, data.event_id.as_bytes());

    // !!! TODO

    Some(opp)
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct NeonPage {
    search_results: Vec<serde_json::Value>,
}

fn interpret_page<Tz: chrono::TimeZone + std::fmt::Debug>(
    partner: &PartnerInfo<Tz>,
    tz: &Tz,
    entries: Value,
) -> Vec<Opportunity> {
    if let Ok(page) = from_value::<NeonPage>(entries) {
        page.search_results
            .into_iter()
            .flat_map(|x| interpret_one(partner, tz, x))
            .collect()
    } else {
        println!("Page did not contain a searchResults key");
        vec![]
    }
}

impl<TZ: chrono::TimeZone + std::fmt::Debug> Structure for NeonGeneric<TZ> {
    type Data = Opportunity;

    fn interpret(&self, parsed: Value) -> Result<OneOrMany<Self::Data>, Error> {
        if let Value::Array(pages) = parsed {
            Ok(Many(
                pages
                    .into_iter()
                    .flat_map(|x| interpret_page(&self.0, &self.1, x))
                    .collect(),
            ))
        } else {
            Err(Error::Structure("Top level of JSON is not an array".into()))
        }
    }
}
