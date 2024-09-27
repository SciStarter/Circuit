use chrono::{NaiveDateTime, TimeDelta};
use common::model::{
    opportunity::{Cost, Domain, EntityType, LocationType},
    partner::{LoggedError, LoggedErrorLevel},
    Opportunity, Partner,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::Error;

use super::{OneOrMany, Structure};

const PBC_STEM_CENTER: Uuid = match Uuid::try_parse("ea0ae38f-d4b7-5ea6-b5c5-5d7817ea9791") {
    Ok(uid) => uid,
    Err(_) => panic!("Expected to be able to parse the UUID constant"),
};

#[derive(Debug)]
pub struct PBCStemCenter;

impl PBCStemCenter {
    fn import_one(&self, row: &serde_json::Value) -> Result<Opportunity, LoggedError> {
        let name = row["Name of Opportunity"].as_str();
        let description = row["Description"].as_str();
        let hosted_by = row["Host Organization"].as_str();
        let contact_name = row["Contact Name"].as_str();
        let contact_email = row["Contact Email"].as_str();
        let online_url = row["Online Only (provide link to participate)"].as_str();
        let location_name = row["Location Name"].as_str();
        let location_address = row["Location Address"].as_str();
        // let location_info = row["Online & Physical Location Only (Name, Address, Link)"].as_str();
        let date = row["Date"].as_str();
        let time = row["Time"].as_str();
        let domain = row
            ["Select the public engagement of science domain that fits your opportunity best."]
            .as_str();
        let cost = row["What is the cost $"].as_str();
        let ticket = row["Ticket or registration required"].as_str();
        // let info_reg = row["More information/registration"].as_str();

        let mut opp = Opportunity::default();
        opp.exterior.uid = Uuid::new_v5(
            &PBC_STEM_CENTER,
            format!(
                "{}/{}/{}",
                name.unwrap_or("[MISSING]"),
                date.unwrap_or("{MISSING}"),
                time.unwrap_or("<MISSING>")
            )
            .as_bytes(),
        );

        opp.exterior.title = name.unwrap_or("").to_owned();
        opp.exterior.description = description.unwrap_or("").to_owned();
        opp.exterior.organization_name = hosted_by.unwrap_or("").to_owned();
        opp.interior.contact_name = contact_name.unwrap_or("").to_owned();
        opp.interior.contact_email = contact_email.unwrap_or("").to_owned();
        opp.exterior.location_name = location_name.unwrap_or("").to_owned();
        opp.exterior.ticket_required = ticket.unwrap_or("Yes") == "Yes";
        opp.exterior.partner_opp_url = online_url.map(str::to_string);

        if let Some(addr) = location_address {
            if addr == "Online" {
                opp.exterior.location_type = LocationType::Any;
                opp.exterior.is_online = true;
            } else {
                let query = common::geo::Query::new(addr.to_owned(), true);
                if let Some(loc) = async_std::task::block_on(query.lookup_one()) {
                    opp.exterior.location_type = LocationType::At;
                    opp.exterior.location_point = Some(
                        serde_json::json!({"type": "Point", "coordinates": [loc.geometry.longitude, loc.geometry.latitude]}),
                    );
                } else {
                    return Err(LoggedError::new(
                        LoggedErrorLevel::Error,
                        format!("Unable to locate address: {}", addr),
                    ));
                }
            }
        }

        opp.exterior.cost = cost
            .map(|x| {
                if x.is_empty() || x == "0" || x == "$0" || x == "0.00" || x == "$0.00" {
                    Cost::Free
                } else {
                    Cost::Cost
                }
            })
            .unwrap_or(Cost::Unknown);

        opp.exterior.pes_domain = match domain {
            Some("Citizen Science") => Domain::CitizenScience,
            Some("Live Science") => Domain::LiveScience,
            Some("Museum Or Science Center") => Domain::MuseumOrScienceCenter,
            Some("Museum or Science Center") => Domain::MuseumOrScienceCenter,
            Some("Maker") => Domain::Maker,
            Some("Policy") => Domain::Policy,
            Some("Out Of School Time Program") => Domain::OutOfSchoolTimeProgram,
            Some("Out of School Time Program") => Domain::OutOfSchoolTimeProgram,
            Some("Formal Education") => Domain::FormalEducation,
            Some("Science Communications") => Domain::ScienceCommunications,
            _ => Domain::Unspecified,
        };

        let start = NaiveDateTime::parse_from_str(
            &format!("{} {}", date.unwrap_or(""), time.unwrap_or("")),
            "%m/%d/%Y %I:%M:%S %p",
        )?
        .and_local_timezone(chrono_tz::US::Eastern)
        .earliest()
        .ok_or_else(|| {
            LoggedError::new(LoggedErrorLevel::Warning, "Time specified is not possible")
        })?
        .fixed_offset();

        opp.exterior.start_datetimes = vec![start];
        opp.exterior.end_datetimes = vec![start + TimeDelta::hours(2)];

        opp.exterior.image_url = "https://www.coxsciencecenter.org/sites/all/themes/sciencemuseum/templates/pbcstem/9e5723ac6436d6efc8b97459d4fe5430.jpg".to_owned();
        opp.exterior.partner_name = "PBC STEM Center".to_owned();
        opp.exterior.partner_website = Some("https://www.coxsciencecenter.org/pbc-stem".to_owned());
        opp.exterior.partner_logo_url = Some("https://www.coxsciencecenter.org/sites/all/themes/sciencemuseum/templates/pbcstem/9e5723ac6436d6efc8b97459d4fe5430.jpg".to_owned());
        opp.exterior.entity_type = EntityType::Opportunity;
        opp.exterior.min_age = 0;
        opp.exterior.max_age = 999;
        opp.exterior.tags.insert("PBC STEM Center".to_owned());

        Ok(opp)
    }
}

#[async_trait::async_trait]
impl Structure for PBCStemCenter {
    type Data = Opportunity;

    fn interpret(&self, parsed: serde_json::Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        let opps = if let Some(rows) = parsed.as_array() {
            rows.iter().map(|row| self.import_one(row)).collect()
        } else {
            Vec::new()
        };

        OneOrMany::Many(opps)
    }

    async fn load_partner(&self, db: &Pool<Postgres>) -> Result<Partner, Error> {
        Ok(Partner::load_by_uid(db, &PBC_STEM_CENTER).await?)
    }
}
