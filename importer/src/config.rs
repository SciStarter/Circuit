use std::time::Duration;

use async_trait::async_trait;

use common::model::opportunity::OpportunityImportRecord;
use importer::format::csv::{CommaSeparated, SemicolonSeparated, TabSeparated};
use importer::format::ical::Ical;
use importer::format::json::Json;
use importer::format::Format;
use importer::source::eventsql::EventsQL;
use importer::source::http::HttpGet;
use importer::source::Source;
use importer::structure::night_sky_network::NightSkyNetwork;
use importer::structure::{OneOrMany, Structure};
use importer::Importer;

pub fn configure() -> Vec<Box<dyn Importer>> {
    let hours = Duration::new(60 * 60, 0);

    let mut importers: Vec<Box<dyn Importer>> = Vec::new();

    importers.push(Box::new(Import {
        source: HttpGet::new("https://nightsky.jpl.nasa.gov/js/data/events_json_api.cfm"),
        format: Json,
        structure: NightSkyNetwork,
        period: 24 * hours,
    }));

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
                item.store(&db).await?;
                OpportunityImportRecord::store(
                    &db,
                    &item.exterior.partner,
                    &item.exterior.uid,
                    created,
                    false, // Ignored is for a hypothetical case, where we may skip importing a record because the current version is authoritative. In that case, it should be set to true.
                )
                .await?;
            }
            OneOrMany::Many(vec) => {
                for mut item in vec {
                    item.set_id_if_necessary(&db).await?;
                    let created = item.id.is_none();
                    item.store(&db).await?;
                    OpportunityImportRecord::store(
                        &db,
                        &item.exterior.partner,
                        &item.exterior.uid,
                        created,
                        false,
                    )
                    .await?;
                }
            }
        }

        Ok(Some(self.period.clone()))
    }
}
