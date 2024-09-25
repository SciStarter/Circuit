use bytes::Bytes;
use common::model::partner::LoggedError;
use serde_json::Value;

#[derive(Debug)]
pub struct Json;

impl super::Format for Json {
    fn decode(&self, raw: Bytes) -> Result<Value, LoggedError> {
        //let _ = std::fs::write("json.raw", &raw);
        let cleaned = String::from_utf8_lossy(&raw).replace(|c: char| c.is_control(), " ");
        //let _ = std::fs::write("json.cleaned", &cleaned);
        match serde_json::from_str(cleaned.as_str()) {
            Ok(val) => Ok(val),
            Err(err) => {
                let line = err.line();
                let column = err.column();
                let excerpt = match cleaned.lines().skip(line - 1).next() {
                    Some(s) => s.chars().skip(column).take(16).collect(),
                    None => String::new(),
                };
                Err(LoggedError::from(err).set_title(format!(
                    "Line: {}  Col: {}  Excerpt: `{}`",
                    line, column, excerpt
                )))
            }
        }
    }
}
