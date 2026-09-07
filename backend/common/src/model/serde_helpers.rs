use super::Error;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn serialize_enum<T: Serialize>(val: &T) -> String {
    // Unit variants serialize to a JSON string like "\"kebab-value\"", and the
    // quotes are stripped so the database holds a bare name. Variants carrying
    // data serialize to an object like {"page":{"layout":"just_content"}}, which
    // has no surrounding quotes to strip and is therefore stored as JSON.
    // deserialize_enum() accepts both forms.
    let json = serde_json::to_string(val).unwrap_or_default();
    json.trim_matches('"').to_string()
}

pub fn deserialize_enum<T: DeserializeOwned>(s: &str) -> Result<T, Error> {
    // The inverse of serialize_enum(): anything that starts a JSON container is
    // parsed as JSON, and everything else is a bare unit-variant name. Wrapping
    // a stored object in Value::String instead would ask serde to match it
    // against the variant names, which no data-carrying variant can satisfy.
    let value = if s.trim_start().starts_with(['{', '[']) {
        serde_json::from_str(s).map_err(|e| Error::Value(format!("deserializing enum: {e}")))?
    } else {
        serde_json::Value::String(s.to_string())
    };

    serde_json::from_value(value).map_err(|e| Error::Value(format!("deserializing enum: {e}")))
}

pub fn serialize_enum_vec<T: Serialize>(vals: &[T]) -> Vec<String> {
    vals.iter().map(|v| serialize_enum(v)).collect()
}

pub fn deserialize_enum_vec<T: DeserializeOwned>(vals: &[String]) -> Vec<T> {
    vals.iter()
        .filter_map(|s| deserialize_enum(s).ok())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    struct Options {
        layout: String,
    }

    #[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
    #[serde(rename_all = "snake_case")]
    enum Sample {
        Structured(Options),
        #[default]
        Unit,
    }

    #[test]
    fn unit_variant_round_trips() {
        let stored = serialize_enum(&Sample::Unit);
        assert_eq!(stored, "unit");
        assert_eq!(deserialize_enum::<Sample>(&stored).unwrap(), Sample::Unit);
    }

    #[test]
    fn structured_variant_round_trips() {
        let original = Sample::Structured(Options {
            layout: "just_content".to_string(),
        });
        let stored = serialize_enum(&original);
        assert_eq!(stored, r#"{"structured":{"layout":"just_content"}}"#);
        assert_eq!(deserialize_enum::<Sample>(&stored).unwrap(), original);
    }

    #[test]
    fn unrecognized_value_is_an_error_rather_than_the_default() {
        assert!(deserialize_enum::<Sample>("structured_just_content").is_err());
    }
}
