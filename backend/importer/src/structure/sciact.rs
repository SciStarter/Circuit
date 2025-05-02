use std::str::FromStr;

use chrono::NaiveDateTime;
use chrono_tz::Tz;
use common::model::{
    opportunity::{Cost, Descriptor, Domain, EntityType, LocationType, Topic, VenueType},
    partner::{LoggedError, LoggedErrorLevel},
    Opportunity, Partner,
};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::Error;

use super::{EmptyAsNone, OneOrMany, Structure};

const NASA_SCIACT: Uuid = match Uuid::try_parse("e07c14de-b15c-50d9-a0c1-8148b0356d2d") {
    Ok(uid) => uid,
    Err(_) => panic!("Expected to be able to parse the UUID constant"),
};

#[derive(Debug)]
pub struct NASASciAct;

impl NASASciAct {
    fn import_one(&self, row: &serde_json::Value) -> Result<Opportunity, LoggedError> {
        let mut opp = Opportunity::default();

        //dbg!(&row);

        let name = row["Name/Title of Event/Activity*"]
            .as_str()
            .empty_as_none();

        opp.exterior.title = if let Some(name) = name {
            if name == "OLD/ARCHIVED EVENTS BELOW" {
                return Err(LoggedError::new(
                    LoggedErrorLevel::TerminateProcessing,
                    "Found end marker",
                ));
            }
            name.to_owned()
        } else {
            return Err(LoggedError::new(
                LoggedErrorLevel::Info,
                "No opportunity on row",
            ));
        };

        let name = opp.exterior.title.clone();

        // dbg!(&name);

        let start_date = row["Start Date"].as_str().empty_as_none().ok_or_else(|| {
            LoggedError::new(
                LoggedErrorLevel::Warning,
                "Opportunity is missing a start date",
            )
            .set_title(&name)
        })?;

        // dbg!(&start_date);

        let end_date = row["End Date"].as_str().empty_as_none().ok_or_else(|| {
            LoggedError::new(
                LoggedErrorLevel::Warning,
                "Opportunity is missing an end date",
            )
            .set_title(&name)
        })?;

        // dbg!(&end_date);

        let start_time = row["Start Time"].as_str().empty_as_none().ok_or_else(|| {
            LoggedError::new(
                LoggedErrorLevel::Warning,
                "Opportunity is missing a start time",
            )
            .set_title(&name)
        })?;

        // dbg!(&start_time);

        let end_time = row["End Time"].as_str().empty_as_none().ok_or_else(|| {
            LoggedError::new(
                LoggedErrorLevel::Warning,
                "Opportunity is missing an end time",
            )
            .set_title(opp.exterior.title.clone())
        })?;

        // dbg!(&end_time);

        let location_zone = row["Time Zone "]
            .as_str()
            .empty_as_none()
            .unwrap_or("US/Eastern");

        // dbg!(&location_zone);

        let start_ndt = match NaiveDateTime::parse_from_str(
            &format!("{} {}", start_date, start_time),
            "%Y/%m/%d %H:%M:%S",
        ) {
            Ok(ndt) => ndt,
            Err(_) => NaiveDateTime::parse_from_str(
                &format!("{} {}", start_date, start_time),
                "%m/%d/%Y %H:%M",
            )?,
        };

        // dbg!(&start_ndt);

        let end_ndt = match NaiveDateTime::parse_from_str(
            &format!("{} {}", end_date, end_time),
            "%Y/%m/%d %H:%M:%S",
        ) {
            Ok(ndt) => ndt,
            Err(_) => NaiveDateTime::parse_from_str(
                &format!("{} {}", end_date, end_time),
                "%m/%d/%Y %H:%M",
            )?,
        };

        // dbg!(&end_ndt);

        let tz: Tz = location_zone.parse().map_err(|_| {
            LoggedError::new(LoggedErrorLevel::Warning, "Unrecognized time zone")
                .set_title(opp.exterior.title.clone())
        })?;

        // dbg!(&tz);

        let Some(start_dt) = start_ndt.and_local_timezone(tz.clone()).earliest() else {
            return Err(LoggedError::new(
                LoggedErrorLevel::Warning,
                "Error constructing the start datetime",
            ));
        };

        // dbg!(&start_dt);

        let Some(end_dt) = end_ndt.and_local_timezone(tz).earliest() else {
            return Err(LoggedError::new(
                LoggedErrorLevel::Warning,
                "Error constructing the end datetime",
            ));
        };

        // dbg!(&end_dt);

        opp.exterior.start_datetimes = vec![start_dt.fixed_offset()];
        opp.exterior.end_datetimes = vec![end_dt.fixed_offset()];

        let guid = row["GUID (internal use only)"].as_str().empty_as_none();

        // dbg!(&guid);

        opp.exterior.uid = guid
            .and_then(|s| Uuid::parse_str(s).ok())
            .unwrap_or_else(|| {
                Uuid::new_v5(
                    &NASA_SCIACT,
                    format!("{}/{}/{}", &name, start_date, start_time).as_bytes(),
                )
            });

        let host = row["Name of Host Organization*"].as_str().empty_as_none();

        // dbg!(&host);

        if let Some(host) = host {
            opp.exterior.organization_name = host.to_owned();
        }

        let contact_name = row["Contact Name*"].as_str().empty_as_none();

        // dbg!(&contact_name);

        if let Some(contact_name) = contact_name {
            opp.interior.contact_name = contact_name.to_owned();
        }

        let contact_email = row["Contact Email*"].as_str().empty_as_none();

        // dbg!(&contact_email);

        if let Some(contact_email) = contact_email {
            opp.interior.contact_email = contact_email.to_owned();
        }

        let location_type = row[r#"Where is Your Opportunity?*
Online Only,
 Physical Location,
or Physical Location & Online"#]
            .as_str()
            .empty_as_none();

        // dbg!(&location_type);

        if let Some(location_type) = location_type {
            opp.exterior.location_type = match location_type {
                "Online Only" => LocationType::Any,
                "Physical Location" => LocationType::At,
                "Physical Location & Online" => LocationType::Any,
                //"" => LocationType::Near,
                _ => LocationType::Unknown,
            }
        }

        let join_url = row["Link to Register or Join*"].as_str().empty_as_none();

        // dbg!(&join_url);

        if let Some(join_url) = join_url {
            opp.exterior.partner_opp_url = Some(join_url.to_owned());
        }

        let location_name = row["For In-Person Events: Name of Physical Location "]
            .as_str()
            .empty_as_none();

        // dbg!(&location_name);

        if let Some(location_name) = location_name {
            opp.exterior.location_name = location_name.to_owned();
        }

        let location_street = row["For In-Person Events: Street Address"]
            .as_str()
            .empty_as_none();

        // dbg!(&location_street);

        if let Some(location_street) = location_street {
            opp.exterior.address_street = location_street.to_owned();
        }

        let location_city = row["For In-Person Events: City"].as_str().empty_as_none();

        // dbg!(&location_city);

        if let Some(location_city) = location_city {
            opp.exterior.address_city = location_city.to_owned();
        }

        let location_state = row["For In-Person Events: Location State (or Province)"]
            .as_str()
            .empty_as_none();

        // dbg!(&location_state);

        if let Some(location_state) = location_state {
            opp.exterior.address_state = location_state.to_owned();
        }

        let location_country = row["For In-Person Events: Country"]
            .as_str()
            .empty_as_none();

        // dbg!(&location_country);

        if let Some(location_country) = location_country {
            opp.exterior.address_country = location_country.to_owned();
        }

        let location_zip = row["For In-Person Events: Zip/Postal Code"]
            .as_str()
            .empty_as_none();

        // dbg!(&location_zip);

        if let Some(location_zip) = location_zip {
            opp.exterior.address_zip = location_zip.to_owned();
        }

        let webpage_url = row["Webpage URL for More Information About This Event/Activity (No Links to Google Drive)"].as_str().empty_as_none();

        // dbg!(&webpage_url);

        if let Some(webpage_url) = webpage_url {
            if opp.exterior.partner_opp_url.is_none() {
                opp.exterior.partner_opp_url = Some(webpage_url.to_owned());
            }
        }

        let short_description = row[r#"Short Summary (164 Character Limit):*
Tell prospective participants what to expect from your opportunity in a short, friendly sentence. Appears in search results."#].as_str().empty_as_none();

        // dbg!(&short_description);

        if let Some(short_desc) = short_description {
            opp.exterior.short_desc = short_desc.to_owned();
        }

        let description = row[r#"Detailed Description of Opportunity:*
Write a public-friendly description for the web, written for an audience has no idea who you are or what this is.)"#].as_str().empty_as_none();

        // dbg!(&description);

        if let Some(desc) = description {
            opp.exterior.description = desc.to_owned();
        }

        let image_url = row[r#"Ideal images should reflect the activity participants will experience, rather than a logo. Acceptable "web friendly" formats: png, jpeg, webp. Recommended dimensions: 600×400 pixels."#].as_str().empty_as_none();

        // dbg!(&image_url);

        if let Some(image_url) = image_url {
            opp.exterior.image_url = image_url.to_owned();
        } else {
            opp.exterior.image_url =
                String::from("https://sciencenearme.s3.us-east-1.amazonaws.com/SciAct.png");
        }

        let image_credit = row["Image Credit + Alt Text (Both Required)"]
            .as_str()
            .empty_as_none();

        // dbg!(&image_credit);

        if let Some(image_credit) = image_credit {
            opp.exterior.image_credit = image_credit.to_owned();
        }

        let domain = row
            ["Select the engagement domain that fits your opportunity best (select ONE)*"]
            .as_str()
            .empty_as_none();

        // dbg!(&domain);

        if let Some(domain) = domain {
            opp.exterior.pes_domain = Domain::from_str(domain).unwrap_or(Domain::Unspecified);
        }

        let descriptor = row
            ["Select the activity types that fit your opportunity best (multiselect)*"]
            .as_str()
            .empty_as_none();

        // dbg!(&descriptor);

        if let Some(descriptor) = descriptor {
            for descriptor in descriptor.split(',') {
                if let Ok(descr) = Descriptor::from_str(descriptor.trim()) {
                    opp.exterior.opp_descriptor.push(descr);
                }
            }
        }

        let cost = row["Associated Cost*"].as_str().empty_as_none();

        // dbg!(&cost);

        if let Some(cost) = cost {
            opp.exterior.cost = match cost {
                "Free" => Cost::Free,
                "0" => Cost::Free,
                "0.00" => Cost::Free,
                "$0" => Cost::Free,
                "$0.00" => Cost::Free,
                _ => Cost::Cost,
            }
        }

        let min_age = row["Minimum Age"].as_str().empty_as_none();

        // dbg!(&min_age);

        if let Some(min_age) = min_age {
            opp.exterior.min_age = min_age.parse().unwrap_or(0);
        }

        let max_age = row["Maximum Age"].as_str().empty_as_none();

        // dbg!(&max_age);

        if let Some(max_age) = max_age {
            opp.exterior.max_age = max_age.parse().unwrap_or(999);
        }

        let venue = row["Select the Venue Type(s) that fit your opportunity best* (leave blank for virtual events)"].as_str().empty_as_none();

        // dbg!(&venue);

        if let Some(venue) = venue {
            opp.exterior.opp_venue = match venue {
                "Outdoors" => vec![VenueType::Outdoors],
                "Indoors" => vec![VenueType::Indoors],
                _ => vec![],
            }
        }

        let topics = row["Select the topic(s) that fit your opportunity best*"]
            .as_str()
            .empty_as_none();

        // dbg!(&topics);

        if let Some(topics) = topics {
            for topic in topics.split(',') {
                if let Ok(topic) = Topic::from_str(topic.trim()) {
                    opp.exterior.opp_topics.push(topic);
                }
            }
        }

        let ticket_required = row["Registration/Ticket Required?*"]
            .as_str()
            .empty_as_none();

        // dbg!(&ticket_required);

        if let Some(ticket_required) = ticket_required {
            opp.exterior.ticket_required = match ticket_required {
                "Yes" => true,
                "yes" => true,
                "TRUE" => true,
                "True" => true,
                "true" => true,
                _ => false,
            }
        }

        let keywords = row["Add Keywords/Phrases. Separate with a comma. (e.g. museum, astronomy, afterschool, library, kids, citizen science, nature)"].as_str().empty_as_none();

        // dbg!(&keywords);

        if let Some(keywords) = keywords {
            for keyword in keywords.split(',') {
                opp.exterior.tags.insert(keyword.to_owned());
            }
        }

        opp.exterior.partner = NASA_SCIACT;
        opp.exterior.partner_name = "NASA SciAct".to_owned();
        opp.exterior.partner_website =
            Some("https://science.nasa.gov/learn/about-science-activation/".to_owned());
        opp.exterior.partner_logo_url =
            Some("https://sciencenearme.s3.us-east-1.amazonaws.com/SciAct.png".to_owned());
        opp.exterior.entity_type = EntityType::Opportunity;
        opp.exterior.tags.insert("NASA SciAct".to_owned());

        Ok(opp)
    }
}

#[async_trait::async_trait]
impl Structure for NASASciAct {
    type Data = Opportunity;

    fn interpret(&self, parsed: serde_json::Value) -> OneOrMany<Result<Self::Data, LoggedError>> {
        let opps = if let Some(rows) = parsed.as_array() {
            let mut accumulator = Vec::with_capacity(rows.len());
            for row in rows.iter() {
                let res = self.import_one(row);
                match res {
                    Err(e) if e.level == LoggedErrorLevel::TerminateProcessing => break,
                    _ => accumulator.push(res),
                }
            }
            accumulator
        } else {
            Vec::new()
        };

        OneOrMany::Many(opps)
    }

    async fn load_partner(&self, db: &Pool<Postgres>) -> Result<Partner, Error> {
        Ok(Partner::load_by_uid(db, &NASA_SCIACT).await?)
    }
}
