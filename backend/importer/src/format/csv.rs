use crate::Error;
use bytes::Bytes;
use common::model::partner::LoggedError;
use csv::{Reader, ReaderBuilder};
use serde_json::Value;

fn _decode<R: std::io::Read>(mut reader: Reader<R>) -> Result<Value, LoggedError> {
    let fields: Vec<String> = reader.headers()?.iter().map(|s| s.to_string()).collect();

    let mut rows = Vec::new();

    for rec in reader.into_records() {
        let mut map = serde_json::Map::new();

        for (k, v) in fields.iter().zip(rec?.iter()) {
            map.insert(k.to_string(), Value::String(v.to_string()));
        }

        rows.push(Value::Object(map));
    }

    Ok(Value::Array(rows))
}

/// CSV: fields separated by "," and records separated by "\r", "\n",
/// or "\r\n". Quotes are escaped by doubling them, so "" represents
/// ". The first row is assumed to contain field names.
#[derive(Debug)]
pub struct CommaSeparated;

impl super::Format for CommaSeparated {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        let reader = ReaderBuilder::new()
            .delimiter(b',')
            .terminator(csv::Terminator::CRLF)
            .has_headers(true)
            .flexible(false)
            .trim(csv::Trim::All)
            .quoting(true)
            .quote(b'"')
            .escape(None)
            .double_quote(true)
            .comment(None)
            .from_reader(raw.as_ref());

        _decode(reader)
    }
}

/// TSV: fields separated by tab characters and records separated by
/// "\r", "\n", or "\r\n". The first row is assumed to contain field
/// names.
#[derive(Debug)]
pub struct TabSeparated;

impl super::Format for TabSeparated {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        let reader = ReaderBuilder::new()
            .delimiter(b'\t')
            .terminator(csv::Terminator::CRLF)
            .has_headers(true)
            .flexible(false)
            .trim(csv::Trim::All)
            .quoting(true)
            .quote(b'"')
            .escape(None)
            .double_quote(true)
            .comment(None)
            .from_reader(raw.as_ref());

        _decode(reader)
    }
}

/// SSV: fields separated by ";" and records separated by "\r", "\n",
/// or "\r\n". The first row is assumed to contain field names.
#[derive(Debug)]
pub struct SemicolonSeparated;

impl super::Format for SemicolonSeparated {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        let reader = ReaderBuilder::new()
            .delimiter(b';')
            .terminator(csv::Terminator::CRLF)
            .has_headers(true)
            .flexible(false)
            .trim(csv::Trim::All)
            .quoting(true)
            .quote(b'"')
            .escape(None)
            .double_quote(true)
            .comment(None)
            .from_reader(raw.as_ref());

        _decode(reader)
    }
}
