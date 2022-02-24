use once_cell::sync::Lazy;
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::collections::BTreeMap;
use thiserror::Error;
use uuid::Uuid;

pub mod emails;
pub mod geo;
pub mod jwt;
pub mod model;
pub mod time;

pub use time::ToFixedOffset;

pub type Database = Pool<Postgres>;

pub type BoxedError = Box<dyn std::error::Error + Sync + Send + 'static>;

static LOG_ENDPOINT: Lazy<String> = Lazy::new(|| {
    format!(
        "http://{}:{}/internal/log",
        std::env::var("CIRCUIT_LOGGER_SERVICE_SERVICE_HOST").unwrap_or_else(|_| std::env::var(
            "CIRCUIT_LOGGER_SERVICE_BETA_SERVICE_HOST"
        )
        .unwrap_or_else(|_| "localhost".to_string())),
        std::env::var("CIRCUIT_LOGGER_SERVICE_SERVICE_PORT").unwrap_or_else(|_| std::env::var(
            "CIRCUIT_LOGGER_SERVICE_BETA_SERVICE_PORT"
        )
        .unwrap_or_else(|_| "9000".to_string())),
    )
});

pub static INTERNAL_UID: Lazy<Uuid> = Lazy::new(|| {
    Uuid::parse_str(
        &std::env::var("INTERNAL_UID").expect("INTERNAL_UID is not set in the environmnet"),
    )
    .expect("INTERNAL_UID environment variable did not contain a UUID")
});

pub static LANGUAGES: Lazy<BTreeMap<String, String>> = Lazy::new(|| {
    [
        ("en", "English"),
        ("es", "Español"),
        ("af", "Afrikaans"),
        ("sq", "shqip"),
        ("am", "ኣማርኛ"),
        ("ar", "العربية"),
        ("hy", "Հայերէն"),
        ("az", "Azərbaycan dili"),
        ("eu", "Euskara"),
        ("be", "Беларуская мова"),
        ("bn", "বাংলা"),
        ("bs", "bosanski"),
        ("bg", "Български"),
        ("ca", "català"),
        ("ceb", "Binisaya"),
        ("zh-CN", "简化字"),
        ("zh-TW", "正體字"),
        ("co", "corsu"),
        ("hr", "hrvatski"),
        ("cs", "čeština"),
        ("da", "dansk"),
        ("nl", "Nederlands"),
        ("eo", "Esperanto"),
        ("et", "eesti keel"),
        ("fi", "suomi"),
        ("fr", "français"),
        ("fy", "Frasch"),
        ("gl", "Galego"),
        ("ka", "ქართული ენა"),
        ("de", "Deutsch"),
        ("el", "ελληνικά"),
        ("gu", "ગુજરાતી"),
        ("ht", "Kreyòl ayisyen"),
        ("ha", " هَرْشَن هَوْسَ"),
        ("haw", "ʻŌlelo Hawaiʻi"),
        ("he", "עברית"),
        ("hi", "हिन्दी"),
        ("hmn", "lus Hmoob"),
        ("hu", "magyar"),
        ("is", "Íslenska"),
        ("ig", "Ásụ̀sụ̀ Ìgbò"),
        ("id", "Bahasa Indonesia"),
        ("ga", "Gaeilge"),
        ("it", "italiano"),
        ("ja", "日本語"),
        ("jv", "baṣa Jawa"),
        ("kn", "ಕನ್ನಡ"),
        ("kk", "Қазақ тілі"),
        ("km", "ភាសាខ្មែរ"),
        ("rw", "Ikinyarwanda"),
        ("ko", "한국어"),
        ("ku", "Kurdî"),
        ("ky", "Kyrgyz tili"),
        ("lo", "ພາສາລາວ"),
        ("la", "Lingua Latina"),
        ("lv", "latviešu valoda"),
        ("lt", "lietuvių kalba"),
        ("lb", "Lëtzebuergesch"),
        ("mk", "македонски"),
        ("mg", "Malagasy"),
        ("ms", "Bahasa Melayu"),
        ("ml", "മലയാളം"),
        ("mt", "Malti"),
        ("mi", "Te Reo Māori"),
        ("mr", "मराठी"),
        ("mn", "ᠮᠣᠩᠭᠣᠯ"),
        ("my", "မြန်မာစကား"),
        ("ne", "नेपाली"),
        ("no", "norsk"),
        ("ny", "Nyanja (Chichewa)"),
        ("or", "ଓଡ଼ିଆ"),
        ("ps", "Pax̌tó"),
        ("fa", "فارسی"),
        ("pl", "polski"),
        ("pt", "Português"),
        ("pa", "ਪੰਜਾਬੀ"),
        ("ro", "limba română"),
        ("ru", "Русский язык"),
        ("sm", "Gagana fa‘a Sāmoa"),
        ("gd", "Gàidhlig"),
        ("sr", "српски"),
        ("st", "Sesotho"),
        ("sn", "Shona"),
        ("sd", "سنڌي‎"),
        ("si", "සිංහල"),
        ("sk", "slovenčina"),
        ("sl", "slovenščina"),
        ("so", "af Soomaali"),
        ("su", "Sundanese"),
        ("sw", "Kiswahili"),
        ("sv", "svenska"),
        ("tl", "Tagalog"),
        ("tg", "тоҷикӣ"),
        ("ta", "தமிழ்"),
        ("tt", "tatarça"),
        ("te", "తెలుగు"),
        ("th", "ภาษาไทย"),
        ("tr", "Türkçe"),
        ("tk", "Türkmen dili"),
        ("uk", "українська мова"),
        ("ur", "اُردُو"),
        ("ug", "Уйғур тил"),
        ("uz", "Ўзбек тили"),
        ("vi", "㗂越"),
        ("cy", "Cymraeg"),
        ("xh", "Xhosa"),
        ("yi", "ײִדיש"),
        ("yo", "Èdè Yorùbá"),
        ("zu", "Zulu"),
    ]
    .iter()
    .map(|(c, n)| (c.to_string(), n.to_string()))
    .collect()
});

#[derive(Debug, Error)]
pub enum Error {
    #[error("Email error: {0}")]
    Email(String),
    #[error("Authorization error: {0}")]
    Auth(String),
    #[error("JWT operation failed")]
    JWT(#[from] ::jwt::Error),
    #[error("UUID operation failed")]
    Uuid(#[from] uuid::Error),
    #[error("Migration failed")]
    Migrate(#[from] sqlx::migrate::MigrateError),
}

pub async fn migrate(db: &Database) -> Result<(), Error> {
    sqlx::migrate!().run(db).await?;
    Ok(())
}

pub fn log<M, T>(tag: &T, msg: &M)
where
    M: Serialize + ?Sized,
    T: AsRef<str> + ?Sized,
{
    async_std::task::spawn(
        surf::post(&*LOG_ENDPOINT)
            .body(serde_json::json!({"at": chrono::Local::now(), "tag": tag.as_ref(), "msg": msg}))
            .send(),
    );
}
