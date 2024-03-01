use std::{collections::BTreeMap, fmt::Debug, time::Duration};

use async_trait::async_trait;
use chrono::TimeZone;
use common::model::Partner;
use scraper::{ElementRef, Html, Selector};

use crate::{structure::PartnerInfo, Error, Importer};

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Field {
    Url,
    Title,
    Datetime,
    Location,
    Image,
    Description,
}

impl Field {
    pub fn name(&self) -> &'static str {
        match self {
            Field::Url => "url",
            Field::Title => "title",
            Field::Datetime => "datetime",
            Field::Location => "location",
            Field::Image => "image",
            Field::Description => "description",
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
    datetime: String,
    description: String,
    location: String,
    image: String,
}

impl OpportunityScratchpad {
    fn set_field(&mut self, field: Field, value: String) {
        match field {
            Field::Url => self.url = value,
            Field::Title => self.title = value,
            Field::Datetime => self.datetime = value,
            Field::Location => self.location = value,
            Field::Image => self.image = value,
            Field::Description => self.description = value,
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
        }

        ProcessingControl::Continue
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
        println!("Loading base page...");

        let raw = ureq::get(&self.url)
            .set("User-Agent", "ScienceNearMe/1.0")
            .call()?
            .into_string()?;

        println!("Loaded HTML...");

        let toplevel = Html::parse_document(&raw);

        println!("Parsed DOM...");

        drop(raw);

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
                dbg!(scratchpad);
            }
        }

        println!("Done.");

        Ok(Some(self.period.clone()))
    }

    async fn load_partner(&self, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Partner, Error> {
        Ok(Partner::load_by_uid(db, &self.info.partner).await?)
    }
}
