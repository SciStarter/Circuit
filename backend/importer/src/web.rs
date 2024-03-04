use std::{collections::BTreeMap, fmt::Debug, time::Duration};

use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use common::model::{
    opportunity::{Cost, EntityType, LocationType, OpportunityImportRecord},
    Opportunity, Partner,
};
use scraper::{ElementRef, Html, Selector};

use crate::{
    structure::{PartnerFlag, PartnerInfo},
    Error, Importer,
};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Field {
    Url,
    Title,
    Start,
    End,
    Image,
    Description,
    LocationName,
    LocationStreet,
    LocationCity,
    LocationState,
    LocationZip,
    LocationCountry,
}

impl Field {
    pub fn name(&self) -> &'static str {
        match self {
            Field::Url => "url",
            Field::Title => "title",
            Field::Start => "start",
            Field::End => "end",
            Field::Image => "image",
            Field::Description => "description",
            Field::LocationName => "location name",
            Field::LocationStreet => "street",
            Field::LocationCity => "city",
            Field::LocationState => "state",
            Field::LocationZip => "zip",
            Field::LocationCountry => "country",
        }
    }
}

#[allow(unused)]
#[derive(Debug)]
enum ProcessingControl {
    Continue,
    Repeat,
    Restart,
    Jump(usize),
    Terminate,
}

#[derive(Eq, PartialEq, Debug)]
pub enum Process {
    SelectText(Field, Selector),
    SelectAttr(Field, Selector, String),
    Meta(Field, &'static str),

    /// See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    SetStartAndEndFromStartDateWithBeginAndEndTimes(&'static str, &'static str, &'static str),

    /// See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    NormalizeStart(&'static str),

    /// See https://docs.rs/chrono/latest/chrono/format/strftime/index.html
    NormalizeEnd(&'static str),
    SplitLocationNameIntoNameAndAddressUSA,
    SplitLocationNameIntoAddressUSA,
}

#[derive(Debug)]
pub struct Page<Tz: TimeZone> {
    period: Duration,
    info: PartnerInfo<Tz>,
    url: String,
    follows: Vec<(Selector, String)>,
    opportunity: Selector,
    steps: Vec<Process>,
}

#[derive(Default, Debug)]
struct OpportunityScratchpad {
    url: String,
    title: String,
    start: String,
    end: String,
    description: String,
    image: String,
    location_name: String,
    location_street: String,
    location_city: String,
    location_state: String,
    location_zip: String,
    location_country: String,
}

impl OpportunityScratchpad {
    fn set_field(&mut self, field: Field, value: String) {
        match field {
            Field::Url => self.url = value,
            Field::Title => self.title = value,
            Field::Start => self.start = value,
            Field::End => self.end = value,
            Field::Image => self.image = value,
            Field::Description => self.description = value,
            Field::LocationName => self.location_name = value,
            Field::LocationStreet => self.location_street = value,
            Field::LocationCity => self.location_city = value,
            Field::LocationState => self.location_state = value,
            Field::LocationZip => self.location_zip = value,
            Field::LocationCountry => self.location_country = value,
        }
    }

    fn apply_steps<Tz: TimeZone>(
        &mut self,
        page: &Page<Tz>,
        meta: &BTreeMap<&str, &str>,
        container: &ElementRef,
        steps: &[Process],
    ) {
        let mut idx = 0;
        let len = steps.len();

        while idx < len {
            let control = steps[idx].apply(page, meta, container, self);

            match control {
                ProcessingControl::Continue => idx = idx + 1,
                ProcessingControl::Repeat => idx = idx,
                ProcessingControl::Restart => idx = 0,
                ProcessingControl::Jump(target) => idx = target,
                ProcessingControl::Terminate => idx = len,
            }
        }
    }
}

impl Process {
    pub fn select_text(field: Field, sel: impl AsRef<str>) -> Process {
        Process::SelectText(field, Selector::parse(sel.as_ref()).expect(field.name()))
    }

    pub fn select_attr(field: Field, sel: impl AsRef<str>, attr: impl AsRef<str>) -> Process {
        Process::SelectAttr(
            field,
            Selector::parse(sel.as_ref()).expect(field.name()),
            attr.as_ref().to_string(),
        )
    }

    fn apply<Tz: TimeZone>(
        &self,
        _page: &Page<Tz>,
        meta: &BTreeMap<&str, &str>,
        container: &ElementRef,
        scratchpad: &mut OpportunityScratchpad,
    ) -> ProcessingControl {
        match self {
            Process::SelectText(target, sel) => {
                if let Some(el) = container.select(sel).next() {
                    scratchpad.set_field(*target, el.text().collect());
                }
            }
            Process::SelectAttr(target, sel, attr) => {
                if let Some(el) = container.select(sel).next() {
                    scratchpad.set_field(*target, el.attr(attr).expect(attr).to_string());
                }
            }
            Process::Meta(field, key) => {
                if let Some(val) = meta.get(key) {
                    scratchpad.set_field(*field, val.to_string());
                }
            }
            Process::SetStartAndEndFromStartDateWithBeginAndEndTimes(
                date_fmt,
                start_fmt,
                end_fmt,
            ) => {
                Process::do_set_start_and_end_from_start_date_with_begin_and_end_times(
                    scratchpad, date_fmt, start_fmt, end_fmt,
                );
            }
            Process::NormalizeStart(fmt) => {
                if let Ok(dt) = NaiveDateTime::parse_from_str(&scratchpad.start, fmt) {
                    scratchpad.start = dt.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
                }
            }
            Process::NormalizeEnd(fmt) => {
                if let Ok(dt) = NaiveDateTime::parse_from_str(&scratchpad.end, fmt) {
                    scratchpad.end = dt.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
                }
            }
            Process::SplitLocationNameIntoNameAndAddressUSA => {
                Process::do_split_location_name_into_name_and_address_usa(scratchpad);
            }
            Process::SplitLocationNameIntoAddressUSA => {
                Process::do_split_location_name_into_address_usa(scratchpad);
            }
        }

        ProcessingControl::Continue
    }

    fn do_split_location_name_into_name_and_address_usa(scratchpad: &mut OpportunityScratchpad) {
        let mut parts: Vec<_> = scratchpad
            .location_name
            .split(',')
            .map(|s| s.trim().to_string())
            .rev()
            .collect();
        scratchpad.location_name = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_street = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_city = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_state = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_country = parts.pop().unwrap_or_else(String::new);
        if scratchpad.location_state.ends_with(char::is_numeric) {
            if let Some((state, zip)) = scratchpad.location_state.rsplit_once(' ') {
                scratchpad.location_zip = zip.trim().to_string();
                scratchpad.location_state = state.trim().to_string();
            }
        }
    }

    fn do_split_location_name_into_address_usa(scratchpad: &mut OpportunityScratchpad) {
        let mut parts: Vec<_> = scratchpad
            .location_name
            .split(',')
            .map(|s| s.trim().to_string())
            .rev()
            .collect();
        scratchpad.location_name = String::new();
        scratchpad.location_street = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_city = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_state = parts.pop().unwrap_or_else(String::new);
        scratchpad.location_country = parts.pop().unwrap_or_else(String::new);
        if scratchpad.location_state.ends_with(char::is_numeric) {
            if let Some((state, zip)) = scratchpad.location_state.rsplit_once(' ') {
                scratchpad.location_zip = zip.trim().to_string();
                scratchpad.location_state = state.trim().to_string();
            }
        }
    }

    fn do_set_start_and_end_from_start_date_with_begin_and_end_times(
        scratchpad: &mut OpportunityScratchpad,
        date_fmt: &'static str,
        start_fmt: &'static str,
        end_fmt: &'static str,
    ) {
        let Ok((date, rest)) = NaiveDate::parse_and_remainder(&scratchpad.start, date_fmt) else {
            println!("Unable to parse date in \"{}\"", &scratchpad.start);
            return;
        };

        let Ok((start_time, rest)) = NaiveTime::parse_and_remainder(rest, start_fmt) else {
            println!("Unable to parse start time in \"{}\"", rest);
            return;
        };

        let Ok((end_time, _)) = NaiveTime::parse_and_remainder(rest, end_fmt) else {
            println!("Unable to parse end time in \"{}\"", rest);
            return;
        };

        let start = NaiveDateTime::new(date.clone(), start_time);
        let end = NaiveDateTime::new(date, end_time);

        scratchpad.start = start.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
        scratchpad.end = end.format("%Y-%m-%dT%H:%M:%S%.f").to_string();
    }
}

impl<Tz: TimeZone + Debug + Sync> Page<Tz> {
    pub fn new(
        url: impl AsRef<str>,
        opportunity_sel: impl AsRef<str>,
        info: PartnerInfo<Tz>,
        period: Duration,
    ) -> Self {
        Page {
            url: url.as_ref().to_string(),
            opportunity: Selector::parse(opportunity_sel.as_ref())
                .expect("Opportunity wrapper selector"),
            follows: Vec::new(),
            steps: Vec::new(),
            info,
            period,
        }
    }

    pub fn follow(mut self, sel: impl AsRef<str>, url_attr: impl AsRef<str>) -> Self {
        self.follows.push((
            Selector::parse(sel.as_ref()).expect("follow CSS selector"),
            url_attr.as_ref().to_string(),
        ));

        self
    }

    pub fn step(mut self, step: Process) -> Self {
        self.steps.push(step);

        self
    }
}

#[async_trait]
impl<Tz: TimeZone + Debug + Sync> Importer for Page<Tz> {
    async fn import(
        &self,
        db: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<std::time::Duration>, Error> {
        let mut pads = Vec::new();

        // This block is to convince the compiler that we're really
        // not going to use the html5ever data once we've built the
        // scratchpads
        {
            println!("Loading base page...");

            let raw = ureq::get(&self.url)
                .set("User-Agent", "ScienceNearMe/1.0")
                .call()?
                .into_string()?;

            println!("Loaded HTML...");

            let toplevel = Html::parse_document(&raw);

            println!("Parsed DOM...");

            let mut doms = Vec::new();

            for (sel, attr) in self.follows.iter() {
                for el in toplevel.select(sel) {
                    if let Some(url) = el.attr(attr) {
                        println!("Following link {} ...", url);

                        let raw = ureq::get(url)
                            .set("User-Agent", "ScienceNearMe/1.0")
                            .call()?
                            .into_string()?;

                        doms.push(Html::parse_document(&raw));
                    }
                }
            }

            doms.push(toplevel);

            println!("Done loading.");

            for dom in doms.into_iter() {
                let mut meta = BTreeMap::new();

                for m in dom.select(&Selector::parse("meta[name],meta[property]").unwrap()) {
                    if let (Some(k), Some(v)) =
                        (m.attr("name").or(m.attr("property")), m.attr("content"))
                    {
                        meta.insert(k, v);
                    }
                }

                for entry in dom.select(&self.opportunity) {
                    let mut scratchpad = OpportunityScratchpad::default();
                    scratchpad.apply_steps(self, &meta, &entry, &self.steps);
                    pads.push(scratchpad);
                }
            }
        }

        for pad in pads.into_iter() {
            let mut opp = Opportunity::default();

            opp.exterior.uid = uuid::Uuid::new_v5(&self.info.partner, pad.title.as_bytes());
            opp.exterior.partner = self.info.partner;
            opp.exterior.partner_name = self.info.partner_name.clone();
            opp.exterior.partner_website = self.info.partner_website.clone();
            opp.exterior.partner_logo_url = self.info.partner_logo_url.clone();
            opp.exterior.partner_opp_url = Some(pad.url);
            opp.exterior.organization_name = self.info.partner_name.clone();
            opp.exterior.organization_website = self.info.partner_website.clone();
            opp.exterior.entity_type = EntityType::Opportunity;
            opp.exterior.pes_domain = self.info.domain.clone();
            opp.exterior.opp_descriptor = self.info.descriptor.clone();
            opp.exterior.title = pad.title;
            opp.exterior.description = pad.description;
            opp.exterior.image_url = pad.image;
            opp.exterior.cost = Cost::Unknown;
            opp.exterior.organization_type = Default::default();
            opp.exterior.languages = vec!["en".to_string()];
            opp.exterior.is_online = false;

            if let Some(tz) = &self.info.timezone {
                opp.exterior.start_datetimes =
                    vec![match DateTime::parse_from_rfc3339(&pad.start) {
                        Ok(dt) => dt,
                        Err(_) => {
                            NaiveDateTime::parse_from_str(&pad.start, "%Y-%m-%dT%H:%M:%S%.f")?
                                .and_local_timezone(tz.clone())
                                .earliest()
                                .ok_or_else(|| {
                                    Error::Data(String::from("DateTime is out of bounds"))
                                })?
                                .fixed_offset()
                        }
                    }];

                opp.exterior.end_datetimes = vec![match DateTime::parse_from_rfc3339(&pad.end) {
                    Ok(dt) => dt,
                    Err(_) => NaiveDateTime::parse_from_str(&pad.end, "%Y-%m-%dT%H:%M:%S%.f")?
                        .and_local_timezone(tz.clone())
                        .earliest()
                        .ok_or_else(|| Error::Data(String::from("DateTime is out of bounds")))?
                        .fixed_offset(),
                }];
            } else {
                opp.exterior.start_datetimes = vec![DateTime::parse_from_rfc3339(&pad.start)?];
                opp.exterior.end_datetimes = vec![DateTime::parse_from_rfc3339(&pad.end)?];
            };

            if pad.location_street.is_empty() {
                if let Some(addr) = &self.info.address {
                    opp.exterior.location_type = LocationType::At;
                    opp.exterior.location_name = addr.name.clone();
                    opp.exterior.address_street = addr.street.clone();
                    opp.exterior.address_city = addr.city.clone();
                    opp.exterior.address_state = addr.state.clone();
                    opp.exterior.address_zip = addr.zip.clone();
                    opp.exterior.address_country = addr.country.clone();
                } else {
                    opp.exterior.location_type = LocationType::Unknown;
                }
            } else {
                opp.exterior.location_type = LocationType::At;
                opp.exterior.location_name = pad.location_name;
                opp.exterior.address_street = pad.location_street;
                opp.exterior.address_city = pad.location_city;
                opp.exterior.address_state = pad.location_state;
                opp.exterior.address_zip = pad.location_zip;
                opp.exterior.address_country = pad.location_country;
            }

            if self.info.flags.contains(&PartnerFlag::OnlineOpportunities) {
                opp.exterior.location_type = LocationType::Any;
                opp.exterior.is_online = true;
            }

            opp.set_id_if_necessary(&db).await?;
            let created = opp.id.is_none();
            opp.interior.accepted = if created { Some(true) } else { None };
            opp.store(&db).await?;
            OpportunityImportRecord::store(
                &db,
                &opp.exterior.partner,
                &opp.exterior.uid,
                created,
                false, // Ignored currently, authoritative flag handling
            )
            .await?;
            println!(
                "{} {}",
                if created { "Added" } else { "Updated" },
                &opp.exterior.title
            );
        }

        Ok(Some(self.period.clone()))
    }

    async fn load_partner(&self, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Partner, Error> {
        Ok(Partner::load_by_uid(db, &self.info.partner).await?)
    }
}
