use std::{fmt::Debug, io::Read, marker::PhantomData, str::FromStr};

use async_trait::async_trait;
use bytes::{Buf, BufMut, BytesMut};
use common::model::{partner::LoggedError, Partner};
use multipart::client::lazy::Multipart;
use once_cell::sync::Lazy;
use serde::{
    de::{self, MapAccess, Visitor},
    Deserialize, Deserializer,
};
use void::Void;

pub mod format;
pub mod source;
pub mod structure;

static UPLOADER_SECRET: Lazy<String> = Lazy::new(|| {
    std::env::var("UPLOADER_AUTH_SECRET").expect("UPLOADER_AUTH_SECRET env var should be set")
});

const HOSTED_FILE_MAX_SIZE: usize = 15 * 1024 * 1024;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("I/O error")]
    IO(#[from] std::io::Error),
    #[error("Incorrectly encoded data")]
    Unicode(#[from] std::str::Utf8Error),
    #[error("HTTP error")]
    Http(#[from] ureq::Error),
    #[error("JSON error")]
    Json(#[from] serde_json::Error),
    #[error("CSV error")]
    Csv(#[from] csv::Error),
    #[error("ICal parse error")]
    IcalParse(#[from] ical::parser::ParserError),
    #[error("{0}")]
    Logged(LoggedError),
    #[error("Model error")]
    Model(#[from] common::model::Error),
    #[error("Data error: {0}")]
    Data(String),
    #[error("Unable to comprehend data")]
    Comprehension,
    #[error("{0}")]
    Misc(String),
}

impl From<LoggedError> for Error {
    fn from(value: LoggedError) -> Self {
        Error::Logged(value)
    }
}

#[async_trait]
pub trait Importer: Debug {
    async fn import(
        &self,
        db: sqlx::Pool<sqlx::Postgres>,
    ) -> Result<Option<std::time::Duration>, Error>;

    async fn load_partner(&self, db: &sqlx::Pool<sqlx::Postgres>) -> Result<Partner, Error>;
}

// https://serde.rs/string-or-struct.html
fn _string_or_struct<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: Deserialize<'de> + FromStr<Err = Void>,
    D: Deserializer<'de>,
{
    // This is a Visitor that forwards string types to T's `FromStr` impl and
    // forwards map types to T's `Deserialize` impl. The `PhantomData` is to
    // keep the compiler from complaining about T being an unused generic type
    // parameter. We need T in order to know the Value type for the Visitor
    // impl.
    struct StringOrStruct<T>(PhantomData<fn() -> T>);

    impl<'de, T> Visitor<'de> for StringOrStruct<T>
    where
        T: Deserialize<'de> + FromStr<Err = Void>,
    {
        type Value = T;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("string or map")
        }

        fn visit_str<E>(self, value: &str) -> Result<T, E>
        where
            E: de::Error,
        {
            Ok(FromStr::from_str(value).unwrap())
        }

        fn visit_map<M>(self, map: M) -> Result<T, M::Error>
        where
            M: MapAccess<'de>,
        {
            // `MapAccessDeserializer` is a wrapper that turns a `MapAccess`
            // into a `Deserializer`, allowing it to be used as the input to T's
            // `Deserialize` implementation. T then deserializes itself using
            // the entries from the map visitor.
            Deserialize::deserialize(de::value::MapAccessDeserializer::new(map))
        }
    }

    deserializer.deserialize_any(StringOrStruct(PhantomData))
}

pub fn cache_file(url: impl AsRef<str>) -> String {
    let url = url.as_ref().to_string();

    if url.is_empty() {
        return String::new();
    }

    let filename = match url.split_once('?') {
        Some((before, _after)) => before,
        None => &url,
    };

    let filename = match filename.rsplit_once('/') {
        Some((_before, after)) => after,
        None => filename,
    };

    let mut writer = BytesMut::new().limit(HOSTED_FILE_MAX_SIZE).writer();
    let (_mime, mut reader) = match ureq::get(&url).call() {
        Ok(resp) => (resp.content_type().to_string(), resp.into_reader()),
        Err(e) => {
            dbg!(e);
            return url;
        }
    };

    if let Err(e) = std::io::copy(&mut reader, &mut writer) {
        dbg!(e);
        return url;
    }

    let mut mp = Multipart::new();
    mp.add_stream(
        "upload",
        writer.into_inner().into_inner().reader(),
        Some(filename),
        None,
    );

    let prepared = match mp.prepare() {
        Ok(x) => x,
        Err(e) => {
            dbg!(e);
            return url;
        }
    };

    let boundary = prepared.boundary().to_string();

    let encoded: Vec<u8> = match prepared.bytes().collect::<Result<_, _>>() {
        Ok(v) => v,
        Err(e) => {
            dbg!(e);
            return url;
        }
    };

    match ureq::post("https://sciencenearme.org/api/upload")
        .set("Authorization", &UPLOADER_SECRET)
        .set(
            "Content-Type",
            &format!("multipart/form-data; boundary={boundary}"),
        )
        .send_bytes(&encoded)
    {
        Ok(resp) => {
            if resp.status() == 200 {
                let data: Vec<String> = match resp.into_json() {
                    Ok(v) => v,
                    Err(e) => {
                        dbg!(e);
                        return url;
                    }
                };

                if let Some(u) = data.into_iter().next() {
                    u
                } else {
                    return url;
                }
            } else {
                dbg!(resp);
                return url;
            }
        }
        Err(e) => {
            dbg!(e);
            return url;
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn cache_file_simple() {
        assert_ne!(
            super::cache_file("https://docs.rs/rust-logo-20210528-1.54.0-nightly-f58631b45.png"),
            "https://docs.rs/rust-logo-20210528-1.54.0-nightly-f58631b45.png"
        );
    }
}
