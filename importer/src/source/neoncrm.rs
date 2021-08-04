use crate::Error;
use bytes::{BufMut, Bytes, BytesMut};
use http_auth_basic::Credentials;

const API_BASE: &'static str = "https://api.neoncrm.com/v2";

#[derive(Debug)]
pub struct NeonCRM {
    auth: Credentials,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StandardSearchField {
    field_name: String,
    operators: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomSearchField {
    display_name: String,
    id: i32,
    operators: Vec<String>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SearchFields {
    custom_fields: Vec<CustomSearchField>,
    standard_fields: Vec<StandardSearchField>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomOutputFieldOptionValue {
    id: String,
    name: String,
    status: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CustomOutputField {
    id: String,
    name: String,
    option_values: Option<Vec<CustomOutputFieldOptionValue>>,
    status: String,
    value: String,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OutputFields {
    custom_fields: Vec<CustomOutputField>,
    standard_fields: Vec<String>,
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
    fn load(&self) -> Result<Bytes, Error> {
        todo!()
    }
}
