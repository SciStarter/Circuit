use std::io::Write;

use crate::Error;
use bytes::{BufMut, Bytes, BytesMut};
use common::model::partner::LoggedError;
use http_auth_basic::Credentials;

const API_BASE: &'static str = "https://api.neoncrm.com/v2";

#[derive(Debug)]
pub struct NeonCRM {
    auth: Credentials,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardSearchField {
    #[serde(default, rename = "field_name")]
    _field_name: String,
    #[serde(default, rename = "operators")]
    _operators: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomSearchField {
    #[serde(default, rename = "display_name")]
    _display_name: String,
    #[serde(default, rename = "id")]
    _id: i32,
    #[serde(default, rename = "operators")]
    _operators: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchFields {
    #[serde(default, rename = "custom_fields")]
    _custom_fields: Vec<CustomSearchField>,
    #[serde(default, rename = "standard_fields")]
    _standard_fields: Vec<StandardSearchField>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomOutputFieldOptionValue {
    #[serde(default, rename = "id")]
    _id: String,
    #[serde(default, rename = "name")]
    _name: String,
    #[serde(default, rename = "status")]
    _status: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomOutputField {
    #[serde(default, rename = "id")]
    _id: String,
    #[serde(default, rename = "name")]
    _name: String,
    #[serde(default, rename = "option_values")]
    _option_values: Option<Vec<CustomOutputFieldOptionValue>>,
    #[serde(default, rename = "status")]
    _status: String,
    #[serde(default, rename = "value")]
    _value: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputFields {
    #[serde(default, rename = "custom_fields")]
    _custom_fields: Vec<CustomOutputField>,
    #[serde(default, rename = "standard_fields")]
    _standard_fields: Vec<String>,
}

impl NeonCRM {
    pub fn new<Si: AsRef<str>, Sk: AsRef<str>>(org_id: Si, api_key: Sk) -> Self {
        NeonCRM {
            auth: Credentials::new(org_id.as_ref(), api_key.as_ref()),
        }
    }

    pub fn search_fields(&self) -> Result<SearchFields, Error> {
        Ok(
            ureq::get(&format!("{}/events/search/searchFields", API_BASE))
                .set("Authorization", &self.auth.as_http_header())
                .call()?
                .into_json()?,
        )
    }

    pub fn output_fields(&self) -> Result<OutputFields, Error> {
        Ok(
            ureq::get(&format!("{}/events/search/outputFields", API_BASE))
                .set("Authorization", &self.auth.as_http_header())
                .call()?
                .into_json()?,
        )
    }
}

impl super::Source for NeonCRM {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut page = 0;

        let mut writer = BytesMut::new().writer();

        writer.write(b"[")?;

        loop {
            println!("Fetching page {}", page);

            let mut reader = ureq::post(&format!("{}/events/search", API_BASE))
                .set("Authorization", &self.auth.as_http_header())
                .send_json(serde_json::json!({
                    "pagination": {
                        "pageSize": 100,
                        "currentPage": page,
                        "sortColumn": "Event End Date",
                        "sortDirection": "ASC",
                    },
                    "searchFields": [
                        {
                            "field": "Event End Date",
                            "operator": "GREATER_AND_EQUAL",
                            "value": chrono::Utc::now().format("%Y-%m-%d").to_string(),
                        }
                    ],
                    "outputFields": [
                        "Event End Date",
                        "Event End Time",
                        "Event ID",
                        "Event Name",
                        "Address Line 1",
                        "Address Line 2",
                        "Address Line 3",
                        "Address Line 4",
                        "Address Type",
                        "City",
                        "Country",
                        "Declined",
                        "Deferred",
                        "Event Admission Fee",
                        "Event Archive",
                        "Event Capacity",
                        "Event Category Name",
                        "Event Created Date/Time",
                        "Event Description",
                        "Event External URL",
                        "Event Last Modified Date/Time",
                        "Event Location Name",
                        "Event Note",
                        "Event Registration Note",
                        "Event Start Date",
                        "Event Start Time",
                        "Event Summary",
                        "Event Topic",
                        "Event Web Publish",
                        "Fee Type",
                        "Full Street Address (F)",
                        "Full Zip Code (F)",
                        "Phone 1 Full Number (F)",
                        "State/Province",
                        "Zip Code",
                    ],
                }))?
                .into_reader();

            let size = std::io::copy(&mut reader, &mut writer)?;

            // 200 bytes is significantly smaller than a page
            // containing at least one event, and significantly larger
            // than an empty page. The API returns empty pages when
            // the available content is exhausted.
            if size < 200 {
                break;
            } else {
                writer.write(b",")?;
                page += 1;
            }
        }

        writer.write(b"]")?;

        Ok(writer.into_inner().into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Source;

    #[test]
    #[ignore]
    fn fetch_scienceworks() {
        NeonCRM::new(
            std::env::var("SCIENCEWORKS_ORG_ID").unwrap(),
            std::env::var("SCIENCEWORKS_API_KEY").unwrap(),
        )
        .load()
        .unwrap();
    }
}
