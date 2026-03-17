use super::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_enum<T: Serialize>(val: &T) -> String {
    // Serialize the enum to a JSON string like "\"kebab-value\"", then strip the quotes
    let json = serde_json::to_string(val).unwrap_or_default();
    json.trim_matches('"').to_string()
}

pub fn deserialize_enum<T: DeserializeOwned>(s: &str) -> Result<T, Error> {
    serde_json::from_value(serde_json::Value::String(s.to_string()))
        .map_err(|e| Error::Value(format!("deserializing enum: {e}")))
}

pub fn serialize_enum_vec<T: Serialize>(vals: &[T]) -> Vec<String> {
    vals.iter().map(|v| serialize_enum(v)).collect()
}

pub fn deserialize_enum_vec<T: DeserializeOwned>(vals: &[String]) -> Vec<T> {
    vals.iter()
        .filter_map(|s| deserialize_enum(s).ok())
        .collect()
}
