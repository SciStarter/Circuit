use google_analyticsdata1_beta::api::{Row, RunReportResponse};
use std::collections::BTreeMap;

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
    type Item = BTreeMap<String, Option<String>>;

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

                return Some(ret);
            } else {
                return None;
            }
        }

        None
    }
}
