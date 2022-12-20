use std::collections::BTreeMap;

use anyhow::{anyhow, Error};
use chrono::{DateTime, FixedOffset, LocalResult, NaiveDate, Utc};
use common::ToFixedOffset;
use google_analyticsdata1_beta::api::{Row, RunReportResponse};
use uuid::Uuid;

#[derive(Debug)]
pub struct ReportRow(BTreeMap<String, Option<String>>);

impl ReportRow {
    pub fn get_date(&self, key: &str) -> Result<DateTime<FixedOffset>, Error> {
        let Some(Some(date)) = self.0.get(key) else { return Err(anyhow!("Field is missing or empty: {}", key)); };
        let Ok(date) = NaiveDate::parse_from_str(&*date, "%Y%m%d") else { return Err(anyhow!("Unparsable date: {}", date)); };
        let Some(date) = date.and_hms_opt(12, 0, 0) else { return Err(anyhow!("Out of bounds date: {}", date)); };
        let LocalResult::Single(date) = date.and_local_timezone(Utc) else { return Err(anyhow!("Failed setting timezone for date")); };
        Ok(date.to_fixed_offset())
    }

    pub fn get_uuid(&self, key: &str) -> Result<Uuid, Error> {
        let Some(Some(uuid)) = self.0.get(key) else { return Err(anyhow!("Field is missing or empty: {}", key)); };
        Ok(Uuid::parse_str(&uuid)?)
    }

    pub fn get_int(&self, key: &str) -> i64 {
        let Some(Some(val)) = self.0.get(key) else { return 0 };
        let Ok(val) = val.parse() else { return 0 };
        val
    }

    pub fn get_float(&self, key: &str) -> f64 {
        let Some(Some(val)) = self.0.get(key) else { return 0.0 };
        let Ok(val) = val.parse() else { return 0.0 };
        val
    }

    pub fn get_string(&self, key: &str) -> String {
        let Some(Some(val)) = self.0.get(key) else { return String::new(); };
        val.clone()
    }
}

pub struct ReportIterator {
    idx: usize,
    last: usize,
    dimensions: Vec<Option<String>>,
    metrics: Vec<Option<String>>,
    rows: Vec<Row>,
}

impl ReportIterator {
    pub fn new(report: RunReportResponse) -> ReportIterator {
        let dimensions = report
            .dimension_headers
            .clone()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|x| x.name)
            .collect();

        let metrics = report
            .metric_headers
            .clone()
            .unwrap_or_else(Vec::new)
            .into_iter()
            .map(|x| x.name)
            .collect();

        let rows = report.rows.unwrap_or_else(Vec::new);

        ReportIterator {
            idx: 0,
            last: rows.len(),
            dimensions,
            metrics,
            rows,
        }
    }
}

impl Iterator for ReportIterator {
    type Item = ReportRow;

    fn next(&mut self) -> Option<Self::Item> {
        while self.idx < self.last {
            if let Some(row) = self.rows.get(self.idx) {
                self.idx += 1;

                let mut ret = BTreeMap::new();

                if let Some(values) = &row.dimension_values {
                    for item in self.dimensions.iter().zip(values.iter()) {
                        if let Some(k) = item.0 {
                            ret.insert(k.clone(), item.1.value.clone());
                        }
                    }
                }

                if let Some(values) = &row.metric_values {
                    for item in self.metrics.iter().zip(values.iter()) {
                        if let Some(k) = item.0 {
                            ret.insert(k.clone(), item.1.value.clone());
                        }
                    }
                }

                if ret.is_empty() {
                    continue;
                }

                return Some(ReportRow(ret));
            } else {
                return None;
            }
        }

        None
    }
}
