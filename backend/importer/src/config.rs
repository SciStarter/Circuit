use std::time::Duration;

use async_trait::async_trait;

use common::model::opportunity::{Descriptor, Domain, OpportunityImportRecord, Topic};
use common::model::partner::LoggedErrorLevel;
use common::model::Partner;
use importer::format::{self, CommaSeparated, Format};
use importer::source::embedded::Embedded;
use importer::source::{self, Source};
use importer::structure::{self, OneOrMany, PartnerAddress, PartnerFlag, PartnerInfo, Structure};
use importer::web::{Field, Page, Process};
use importer::{Error, Importer};

/// This function is the 'config file' for the importer. Each entry
/// added to the importers vector defines how to grab data from one
/// partner.
pub fn configure(importers: &mut Vec<Box<dyn Importer>>) {
    let hours = Duration::new(60 * 60, 0);

    importers.push(Box::new(Import {
        source: Embedded::new(include_bytes!("csv/sciact.csv")), // Remember to clean up the CSV headers
        format: CommaSeparated,
        structure: structure::NASASciAct,
        period: 90 * 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: Embedded::new(include_bytes!("csv/pbc_stem.csv")),
        format: CommaSeparated,
        structure: structure::PBCStemCenter,
        period: 90 * 24 * hours,
    }));

    importers.push(Box::new(Import {
        //source: source::HttpGet::new("https://nightsky.jpl.nasa.gov/js/data/events_json_api.cfm"),
        source: source::HttpGet::new("https://nightsky.jpl.nasa.gov/json/events/api/"),
        format: format::Json,
        structure: structure::NightSkyNetwork,
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::WordPressRest::new("https://networkforyouthsuccess.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50"),
        format: format::Json,
        structure: structure::EventsJson(PartnerInfo {
            partner: "d5d0282c-1fd7-5b58-a4c2-3512778217b9".parse().unwrap(),
            partner_name: "New York State Network for Youth Success".to_string(),
            partner_website: Some("https://networkforyouthsuccess.org/".to_string()),
            partner_logo_url: None,
            domain: Domain::OutOfSchoolTimeProgram,
            descriptor: vec![Descriptor::Community, Descriptor::LiveScience],
            topics: vec![Topic::GeneralScience],
            flags: vec![],
            address: None,
            timezone: Some(chrono_tz::US::Eastern),
        }),
        period: 24 * hours,
    }));

    importers.push(Box::new(
        Page::new(
            "https://www.rockvillesciencecenter.org/event-registration",
            r#"section[data-hook="event-details"]"#,
            PartnerInfo {
                partner: "67fcc1f9-56f5-5e71-853a-3997ed986e40".parse().unwrap(),
                partner_name: "Rockville Science Center".into(),
                partner_website: Some("https://www.rockvillesciencecenter.org/".into()),
                partner_logo_url: Some("https://static.wixstatic.com/media/d8f739_ac946469337743e8bfaaa0aabe1d10f3~mv2.png".into()),
                domain: Domain::MuseumOrScienceCenter,
                descriptor: vec![Descriptor::Maker],
                topics: vec![],
                flags: vec![PartnerFlag::Ticketed],
                address: Some(PartnerAddress {
                    name: "Rockville Science Center".into(),
                    street: "33-F Maryland Avenue".into(),
                    city: "Rockville".into(),
                    state: "MD".into(),
                    zip: "20850".into(),
                    country: "United States of America".into()
                }),
                timezone: Some(chrono_tz::US::Eastern),
            },
            24 * hours,
        )
        .follow(r#"a[data-hook="ev-rsvp-button"]"#, "href")
        .step(Process::Meta(Field::Url, "og:url"))
        .step(Process::Meta(Field::Title, "og:title"))
        .step(Process::Meta(Field::Image, "og:image"))
        .step(Process::select_text(Field::Start, r#"p[data-hook="event-full-date"]"#))
        .step(Process::select_text(Field::Description, r#"div[data-hook="about-section-text"]"#))
        .step(Process::select_text(Field::LocationName, r#"p[data-hook="event-full-location"]"#))
        .step(Process::SetStartAndEndFromStartDateWithBeginAndEndTimes("%b %e, %Y", ", %l:%M %p", " – %l:%M %p")) // That's an en dash
        .step(Process::SplitLocationNameIntoNameAndAddressUSA)
    ));

    importers.push(Box::new(Import {
        source: source::ModernEventsCalendar::new(
            "https://scitechinstitute.org/wp-admin/admin-ajax.php",
        ),
        format: format::Json,
        structure: structure::LdJson(
            "items".into(),
            PartnerInfo {
                partner: "12a96513-a9a5-5372-8a85-c68ce074a54b".parse().unwrap(),
                partner_name: "SciTech Institute".into(),
                partner_website: Some("https://scitechinstitute.org/".into()),
                partner_logo_url: Some(
                    "https://scitechinstitute.org/wp-content/themes/aztc-scitech/img/logo.svg"
                        .into(),
                ),
                domain: Domain::MuseumOrScienceCenter,
                descriptor: vec![Descriptor::Community],
                topics: vec![Topic::GeneralScience],
                flags: Vec::new(),
                address: Some(PartnerAddress {
                    name: "SciTech Institute".to_string(),
                    street: "1438 W. Broadway Rd., Ste 101".to_string(),
                    city: "Tempe".to_string(),
                    state: "AZ".to_string(),
                    zip: "85282".to_string(),
                    country: "United States of America".to_string(),
                }),
                timezone: Some(chrono_tz::US::Arizona),
            },
        ),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::ModernEventsCalendar::new("https://www.explora.us/wp-admin/admin-ajax.php"),
        format: format::Json,
        structure: structure::LdJson(
            "items".into(),
            PartnerInfo {
                partner: "9c3b9c9e-238b-5dad-aa12-9ef22a8c3574".parse().unwrap(),
                partner_name: "Explora".into(),
                partner_website: Some("https://explora.us/".into()),
                partner_logo_url: Some("https://www.explora.us/wp-admin/admin-ajax.php".into()),
                domain: Domain::MuseumOrScienceCenter,
                descriptor: vec![Descriptor::ExpoStyle],
                topics: vec![Topic::Engineering, Topic::Technology],
                flags: Vec::new(),
                address: Some(PartnerAddress {
                    name: "Explora".to_string(),
                    street: "1701 Mountain Road NW".to_string(),
                    city: "Albuquerque".to_string(),
                    state: "NM".to_string(),
                    zip: "87104".to_string(),
                    country: "United States of America".to_string(),
                }),
                timezone: Some(chrono_tz::US::Mountain),
            },
        ),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        //source: source::Airtable::new("appytM7ldnmIDcbRV", ["Events"]), // 2022 list
        //source: source::Airtable::new("appzz89bVacXdFSeZ", ["Events"]), // 2023 list
        source: source::Airtable::new("appwRKtBLHRTOiAMB", ["Events"]), // 2024 list
        format: format::Json,
        structure: structure::AtlantaScienceFest::<2022>, // The 2022 table structure has so far been carried forward to later years
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
            topics: vec![Topic::GeneralScience],
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
        // source: source::EventsQLWithCustom::new("https://www.wisconsinsciencefest.org/graphql"),
        source: source::WordPressRest::new("https://www.wisconsinsciencefest.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50"),
        format: format::Json,
        // structure: structure::EventsQL(PartnerInfo {
        structure: structure::EventsJson(PartnerInfo {
            partner: "f390b07f-5e9b-5d19-bdab-d5e91401b7ff".parse().unwrap(),
            partner_name: "Wisconsin Science Festival".to_string(),
            partner_website: Some("https://www.wisconsinsciencefest.org/".to_string()),
            partner_logo_url: Some(
                "https://www.wisconsinsciencefest.org/wp-content/uploads/2018/10/WSF_Beesly_Flying-01.png".to_string(),
            ),
            domain: Domain::LiveScience,
            descriptor: vec![Descriptor::Festival],
            topics: vec![Topic::GeneralScience],
            flags: vec![],
            address: None,
            timezone: Some(chrono_tz::US::Central),
        }),
        period: 24 * hours,
    }));

    importers.push(Box::new(Import {
        source: source::WordPressRest::new(
            "https://astronomyontap.org/wp-json/tribe/events/v1/events/?status=publish&per_page=50",
        ),
        format: format::Json,
        structure: structure::EventsJson(PartnerInfo {
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
            descriptor: vec![Descriptor::LiveScience],
            topics: vec![Topic::GeneralScience],
            flags: vec![PartnerFlag::Cost],
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
            topics: vec![Topic::GeneralScience],
            flags: vec![PartnerFlag::Cost],
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

struct ImportOutcome {
    partner: String,
    date: String,
    added: i32,
    updated: i32,
    failed: i32,
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

        let mut outcome = ImportOutcome {
            partner: partner.exterior.name.clone(),
            date: chrono::Local::now()
                .date_naive()
                .format("%Y-%m-%d")
                .to_string(),
            added: 0,
            updated: 0,
            failed: 0,
        };

        async fn store_outcome(outcome: ImportOutcome, db: &sqlx::Pool<sqlx::Postgres>) {
            let _ = sqlx::query!(
                r#"
                insert into c_import_outcomes (
                  "partner",
                  "date",
                  "added",
                  "updated",
                  "failed"
                )
                values ($1, $2, $3, $4, $5)
                on conflict ("partner", "date") do update
                set "added" = c_import_outcomes."added" + $3,
                    "updated" = c_import_outcomes."updated" + $4,
                    "failed" = c_import_outcomes."failed" + 5
                "#,
                outcome.partner,
                outcome.date,
                outcome.added,
                outcome.updated,
                outcome.failed,
            )
            .execute(db)
            .await;
        }

        println!("Loading...");
        let source = match self.source.load() {
            Ok(s) => s,
            Err(mut le) => {
                le.partner_id = partner
                    .id
                    .expect("The id should be set after loading from the database");
                let _ = le.store(&db).await;
                println!("[Load] Logged error: {}", &le.message);
                store_outcome(outcome, &db).await;
                return Err(le.into());
            }
        };

        println!("Parsing...");
        let format = match self.format.decode(source) {
            Ok(f) => f,
            Err(mut le) => {
                le.partner_id = partner
                    .id
                    .expect("The id should be set after loading from the database");
                let _ = le.store(&db).await;
                println!("[Parse] Logged error: {}", &le.message);
                store_outcome(outcome, &db).await;
                return Err(le.into());
            }
        };

        println!("Interpreting...");
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
                            if created {
                                outcome.added += 1;
                                "Added"
                            } else {
                                outcome.updated += 1;
                                "Updated"
                            },
                            &item.exterior.title
                        );
                    }
                    Err(mut le) => {
                        outcome.failed += 1;
                        le.partner_id = partner
                            .id
                            .expect("The id should be set after loading from the database");
                        let _ = le.store(&db).await;
                        if le.level != LoggedErrorLevel::Info {
                            println!("[Interpret] Logged error: {}", &le.message);
                        }
                    }
                }
            }
            OneOrMany::Many(vec) => {
                for result in vec {
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
                                if created {
                                    outcome.added += 1;
                                    "Added"
                                } else {
                                    outcome.updated += 1;
                                    "Updated"
                                },
                                &item.exterior.title
                            );
                        }
                        Err(mut le) => {
                            outcome.failed += 1;
                            le.partner_id = partner
                                .id
                                .expect("The id should be set after loading from the database");
                            let _ = le.store(&db).await;
                            if le.level != LoggedErrorLevel::Info {
                                println!("[Interpret] Logged error: {}", &le.message);
                            }
                        }
                    }
                }
            }
        }

        store_outcome(outcome, &db).await;

        println!("Done.");

        Ok(Some(self.period.clone()))
    }

    async fn load_partner(&self, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Partner, Error> {
        self.structure.load_partner(db).await
    }
}
