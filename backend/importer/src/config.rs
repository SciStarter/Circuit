use std::time::Duration;

use async_trait::async_trait;

use common::model::opportunity::{Descriptor, Domain, OpportunityImportRecord, Topic};
use common::model::Partner;
use importer::format::{self, Format};
use importer::source::{self, Source};
use importer::structure::{self, OneOrMany, PartnerAddress, PartnerFlag, PartnerInfo, Structure};
use importer::{Error, Importer};

/// This function is the 'config file' for the importer. Each entry
/// added to the importers vector defines how to grab data from one
/// partner.
pub fn configure(importers: &mut Vec<Box<dyn Importer>>) {
    let hours = Duration::new(60 * 60, 0);

    importers.push(Box::new(Import {
        source: source::HttpGet::new(
            "https://scitechinstitute.org/wp-json/mecexternal/v1/calendar/1",
        ),
        format: format::Json,
        structure: structure::ModernEventsCalendar(PartnerInfo {
            partner: "12a96513-a9a5-5372-8a85-c68ce074a54b".parse().unwrap(),
            partner_name: "SciTech Institute".to_string(),
            partner_website: Some("https://scitechinstitute.org/".to_string()),
            partner_logo_url: Some(
                "https://scitechinstitute.org/wp-content/themes/aztc-scitech/img/logo.svg"
                    .to_string(),
            ),
            domain: Domain::MuseumOrScienceCenter,
            descriptor: vec![Descriptor::ExpoStyle],
            topics: vec![Topic::Engineering, Topic::Technology],
            flags: vec![],
            address: Some(PartnerAddress {
                name: "SciTech Institute".to_string(),
                street: "1438 W. Broadway Rd., Ste 101".to_string(),
                city: "Tempe".to_string(),
                state: "AZ".to_string(),
                zip: "85282".to_string(),
                country: "United States of America".to_string(),
            }),
            timezone: Some(chrono_tz::US::Mountain),
        }),
        period: 4 * hours,
    }));

    importers.push(Box::new(Import {
        //source: source::Airtable::new("appytM7ldnmIDcbRV", ["Events"]), // 2022 list
        source: source::Airtable::new("appzz89bVacXdFSeZ", ["Events"]), // 2023 list
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
            partner_name: "Terry Lee Wells Nevada Discovery Museum".to_string(),
            partner_website: Some("https://nvdm.org/".to_string()),
            partner_logo_url: Some(
                "https://nvdm.org/wp-content/themes/discoverypress-site/assets/svgs/discovery-logo.svg".to_string(),
            ),
            domain: Domain::MuseumOrScienceCenter,
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
        source: source::WordPressRest::new(
            "https://mods.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50",
        ),
        format: format::Json,
        structure: structure::EventsJson(PartnerInfo {
            partner: "b1f10e01-ad75-5a84-8efc-04003af9e202".parse().unwrap(),
            partner_name: "Museum of Discovery and Science".to_string(),
            partner_website: Some("https://mods.org/".to_string()),
            partner_logo_url: Some(
                "https://mods.org/wp-content/uploads/2014/11/main_logo.png".to_string(),
            ),
            domain: Domain::MuseumOrScienceCenter,
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

    importers.push(Box::new(Import {
        source: source::EventsQLWithCustom::new("https://ecastonline.org/graphql"),
        format: format::Json,
        structure: structure::EventsQL(PartnerInfo {
            partner: "74ba0027-887d-5dc4-928a-3a9beda27e4e".parse().unwrap(),
            partner_name: "ECast".to_string(),
            partner_website: Some("https://ecastonline.org/".to_string()),
            partner_logo_url: Some(
                "https://ecastonline.org/wp-content/uploads/2014/08/final-ecast_outline.png"
                    .to_string(),
            ),
            domain: Domain::Policy,
            descriptor: vec![Descriptor::Forum],
            topics: vec![],
            flags: vec![],
            address: None,
            timezone: Some(chrono_tz::US::Eastern),
        }),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::WordPressRest::new("https://www.mi-sci.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50"),
        format: format::Json,
        structure: structure::EventsJson(PartnerInfo {
            partner: "020aa009-8225-5fd3-95e9-42c29067f4c8".parse().unwrap(),
            partner_name: "Michigan Science Center".to_string(),
            partner_website: Some("https://mi-sci.org/".to_string()),
            partner_logo_url: Some(
                "https://www.mi-sci.org/wp-content/uploads/2020/06/website-logo-color-with-shadow.png"
                    .to_string(),
            ),
            domain: Domain::MuseumOrScienceCenter,
            descriptor: vec![Descriptor::LiveScience],
            topics: vec![],
            flags: vec![],
            address: Some(PartnerAddress {
                name: "Michigan Science Center".into(),
                street: "5020 John R. Street".into(),
                city: "Detroit".into(),
                state: "MI".into(),
                zip: "48202".into(),
                country: "USA".into()
            }),
            timezone: Some(chrono_tz::US::Eastern),
        }),
        period: 24 * hours,
    }));

    // Disabled because they're using an exchange instead
    // importers.push(Box::new(Import {
    //     source: source::EventsQLWithCustom::new("https://stemcouncil.alabama.gov/graphql"),
    //     format: format::Json,
    //     structure: structure::EventsQL(PartnerInfo {
    //         partner: "b9224b48-dcc3-5153-9c31-7b53ff24a380".parse().unwrap(),
    //         partner_name: "Alabama STEM Council".to_string(),
    //         partner_website: Some("https://stemcouncil.alabama.gov/".to_string()),
    //         partner_logo_url: Some("".to_string()),
    //         domain: Domain::OutOfSchoolTimeProgram,
    //         descriptor: vec![],
    //         topics: vec![],
    //         flags: vec![],
    //         address: None,
    //         timezone: Some(chrono_tz::US::Central),
    //     }),
    //     period: 24 * hours,
    // }));
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
        let partner = if let Some("1") = option_env!("DEV_MODE") {
            let mut p = Partner::default();
            p.exterior.name = "Fake dev partner".into();
            p.id = Some(0);
            p
        } else {
            self.load_partner(&db).await?
        };

        let source = match self.source.load() {
            Ok(s) => s,
            Err(mut le) => {
                le.partner_id = partner
                    .id
                    .expect("The id should be set after loading from the database");
                let _ = le.store(&db).await;
                println!("Logged error: {}", &le.message);
                return Err(le.into());
            }
        };

        let format = match self.format.decode(source) {
            Ok(f) => f,
            Err(mut le) => {
                le.partner_id = partner
                    .id
                    .expect("The id should be set after loading from the database");
                let _ = le.store(&db).await;
                println!("Logged error: {}", &le.message);
                return Err(le.into());
            }
        };

        match self.structure.interpret(format) {
            OneOrMany::One(result) => {
                match result {
                    Ok(mut item) => {
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
                        println!(
                            "{} {}",
                            if created { "Added" } else { "Updated" },
                            &item.exterior.title
                        );
                    }
                    Err(mut le) => {
                        le.partner_id = partner
                            .id
                            .expect("The id should be set after loading from the database");
                        let _ = le.store(&db).await;
                        println!("Logged error: {}", &le.message);
                    }
                }
            }
            OneOrMany::Many(vec) => {
                for mut result in vec {
                    match result {
                        Ok(mut item) => {
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
                            println!(
                                "{} {}",
                                if created { "Added" } else { "Updated" },
                                &item.exterior.title
                            );
                        }
                        Err(mut le) => {
                            le.partner_id = partner
                                .id
                                .expect("The id should be set after loading from the database");
                            let _ = le.store(&db).await;
                            println!("Logged error: {}", &le.message);
                        }
                    }
                }
            }
        }

        Ok(Some(self.period.clone()))
    }

    async fn load_partner(&self, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Partner, Error> {
        self.structure.load_partner(db).await
    }
}
