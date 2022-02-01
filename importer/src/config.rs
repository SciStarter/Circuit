use std::time::Duration;

use async_trait::async_trait;

use common::model::opportunity::{Descriptor, Domain, OpportunityImportRecord, Topic};
use importer::format::{self, Format};
use importer::source::{self, Source};
use importer::structure::{self, OneOrMany, PartnerAddress, PartnerFlag, PartnerInfo, Structure};
use importer::Importer;

/// This function is the 'config file' for the importer. Each entry
/// added to th importers vector defines how to grab data from one
/// partner.
pub fn configure(importers: &mut Vec<Box<dyn Importer>>) {
    let hours = Duration::new(60 * 60, 0);

    importers.push(Box::new(Import {
        source: source::Airtable::new("appytM7ldnmIDcbRV", ["Events"]),
        format: format::Json,
        structure: structure::AtlantaScienceFest::<2022>,
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::HttpGet::new("https://nightsky.jpl.nasa.gov/js/data/events_json_api.cfm"),
        format: format::Json,
        structure: structure::NightSkyNetwork,
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::HttpGet::new(
            "https://nvdm.org/wp-json/tribe/events/v1/events/?per_page=1000&status=publish",
        ),
        format: format::Json,
        structure: structure::EventsJson(PartnerInfo {
            partner: "82b846de-dda5-5bad-a918-41a2a0648b71".parse().unwrap(),
            partner_name: "Nevada Discovery Museum".to_string(),
            partner_website: Some("https://nvdm.org/".to_string()),
            partner_logo_url: Some(
                "https://nvdm.org/wp-content/themes/discoverypress-site/assets/svgs/discovery-logo.svg".to_string(),
            ),
            domain: Domain::LiveScience,
            descriptor: vec![Descriptor::Community],
            topics: vec![],
            flags: vec![PartnerFlag::Cost],
            address: Some(PartnerAddress {
                name: "The Discovery".to_string(),
                street: "490 S. Center Street".to_string(),
                city: "Reno".to_string(),
                state: "NV".to_string(),
                zip: "89501".to_string(),
                country: "United States of America".to_string(),
            }),
            timezone: Some(chrono_tz::US::Pacific),
        }),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::EventsQLWithCustom::new("https://www.wisconsinsciencefest.org/graphql"),
        format: format::Json,
        structure: structure::EventsQL(PartnerInfo {
            partner: "f390b07f-5e9b-5d19-bdab-d5e91401b7ff".parse().unwrap(),
            partner_name: "Wisconsin Science Festival".to_string(),
            partner_website: Some("https://www.wisconsinsciencefest.org/".to_string()),
            partner_logo_url: Some(
                "https://www.wisconsinsciencefest.org/wp-content/uploads/2018/10/WSF_Beesly_Flying-01.png".to_string(),
            ),
            domain: Domain::LiveScience,
            descriptor: vec![],
            topics: vec![],
            flags: vec![],
            address: None,
            timezone: Some(chrono_tz::US::Central),
        }),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::EventsQLWithCustom::new("https://astronomyontap.org/graphql"),
        format: format::Json,
        structure: structure::EventsQL(PartnerInfo {
            partner: "784f3316-bdc0-5855-8a44-2044cbb23788".parse().unwrap(),
            partner_name: "Astronomy On Tap".to_string(),
            partner_website: Some("https://astronomyontap.org/".to_string()),
            partner_logo_url: Some("".to_string()),
            domain: Domain::LiveScience,
            topics: vec![Topic::AstronomyAndSpace],
            descriptor: vec![Descriptor::Community],
            flags: vec![],
            address: None,
            timezone: Some(chrono::Utc),
        }),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::EventsQL::new("https://mods.org/graphql"),
        format: format::Json,
        structure: structure::EventsQL(PartnerInfo {
            partner: "b1f10e01-ad75-5a84-8efc-04003af9e202".parse().unwrap(),
            partner_name: "Museum of Discovery and Science".to_string(),
            partner_website: Some("https://mods.org/".to_string()),
            partner_logo_url: Some(
                "https://mods.org/wp-content/uploads/2014/11/main_logo.png".to_string(),
            ),
            domain: Domain::LiveScience,
            descriptor: vec![],
            topics: vec![],
            flags: vec![],
            address: Some(PartnerAddress {
                name: "Museum of Discovery and Science".to_string(),
                street: "401 SW Second Street".to_string(),
                city: "Fort Lauderdale".to_string(),
                state: "FL".to_string(),
                zip: "33312".to_string(),
                country: "USA".to_string(),
            }),
            timezone: Some(chrono_tz::US::Eastern),
        }),
        period: 24 * hours,
    }));
}

pub fn setup() -> Vec<Box<dyn Importer>> {
    let mut importers: Vec<Box<dyn Importer>> = Vec::new();

    configure(&mut importers);

    importers.shrink_to_fit();

    importers
}

#[derive(Debug)]
pub struct Import<Src, Fmt, Struct> {
    source: Src,
    format: Fmt,
    structure: Struct,
    period: Duration,
}

#[async_trait]
impl<Src, Fmt, Struct> Importer for Import<Src, Fmt, Struct>
where
    Src: Source + Sync + Send,
    Fmt: Format + Sync + Send,
    Struct: Structure<Data = common::model::Opportunity> + Sync + Send,
{
    async fn import(
        &self,
        db: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<std::time::Duration>, importer::Error> {
        match self
            .structure
            .interpret(self.format.decode(self.source.load()?)?)?
        {
            OneOrMany::One(mut item) => {
                item.set_id_if_necessary(&db).await?;
                let created = item.id.is_none();
                item.interior.accepted = if created { Some(true) } else { None };
                item.store(&db).await?;
                OpportunityImportRecord::store(
                    &db,
                    &item.exterior.partner,
                    &item.exterior.uid,
                    created,
                    false, // Ignored is for a hypothetical case, where we may skip importing a record because the current version is authoritative. In that case, it should be set to true.
                )
                .await?;
                println!("Saved {}", &item.exterior.title);
            }
            OneOrMany::Many(vec) => {
                for mut item in vec {
                    item.set_id_if_necessary(&db).await?;
                    let created = item.id.is_none();
                    item.interior.accepted = if created { Some(true) } else { None };
                    item.store(&db).await?;
                    OpportunityImportRecord::store(
                        &db,
                        &item.exterior.partner,
                        &item.exterior.uid,
                        created,
                        false,
                    )
                    .await?;
                    println!("Saved {}", &item.exterior.title);
                }
            }
        }

        Ok(Some(self.period.clone()))
    }
}
