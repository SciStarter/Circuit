use crate::Error;
use bytes::Bytes;
use common::model::partner::LoggedError;
use serde_json::Value;

#[derive(Debug)]
pub struct Airtable {
    base: String,
    tables: Vec<String>,
}

impl Airtable {
    pub fn new<S: AsRef<str>, VS: IntoIterator>(base: S, tables: VS) -> Self
    where
        <VS as IntoIterator>::Item: AsRef<str>,
    {
        Airtable {
            base: base.as_ref().to_string(),
            tables: tables.into_iter().map(|x| x.as_ref().to_string()).collect(),
        }
    }
}

fn get_table(base: &str, table: &str) -> Result<Vec<Value>, Error> {
    let mut records = Vec::new();

    let token = std::env::var("AIRTABLE_TOKEN").map_err(|err| Error::Misc(err.to_string()))?;
    let mut offset = None;

    loop {
        let url = if let Some(offset) = offset {
            format!("https://api.airtable.com/v0/{base}/{table}?offset={offset}")
        } else {
            format!("https://api.airtable.com/v0/{base}/{table}")
        };

        let mut result: Value = ureq::get(&url)
            .set("Authorization", &format!("Bearer {token}"))
            .call()?
            .into_json()?;

        offset = result["offset"].as_str().map(|x| x.to_string());

        if let Some(values) = result["records"].as_array_mut() {
            records.append(values);
        }

        if let None = offset {
            return Ok(records);
        }
    }
}

impl super::Source for Airtable {
    fn load(&self) -> Result<Bytes, LoggedError> {
        let mut data = serde_json::json!({});

        for table in self.tables.iter() {
            data[table] = get_table(self.base.as_ref(), table.as_ref())?.into();
        }

        Ok(serde_json::to_string(&data)?.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::Source;

    #[test]
    fn fetch_atlanta_science_festival_events() {
        assert_eq!(
            Airtable::new("appytM7ldnmIDcbRV", ["Events"])
                .load()
                .unwrap()[..11],
            b"{\"Events\":["[..]
        );
    }
}
