use once_cell::sync::Lazy;
use regex::{Captures, Regex, RegexBuilder};
use serde::Serialize;
use sqlx::{Pool, Postgres};
use std::collections::BTreeMap;
use thiserror::Error;
use uuid::Uuid;

pub mod emails;
pub mod gis;
pub mod jwt;
pub mod model;
pub mod time;
pub mod timezones;

pub use crate::time::ToFixedOffset;

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
        &std::env::var("INTERNAL_UID").expect("INTERNAL_UID should be set in the environmnet"),
    )
    .expect("INTERNAL_UID environment variable should contain a UUID")
});

pub static ANONYMOUS_UID: Lazy<Uuid> = Lazy::new(|| {
    Uuid::parse_str("66ec1ab7-f0fb-4771-88b8-54bff8dddebb")
        .expect("const UUID string should parse correctly")
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

pub fn log<M, T>(who: Option<&Uuid>, tag: &T, msg: &M)
where
    M: Serialize + ?Sized,
    T: AsRef<str> + ?Sized,
{
    async_std::task::spawn(
        surf::post(&*LOG_ENDPOINT)
            .body(serde_json::json!({"at": chrono::Local::now(), "who": who, "tag": tag.as_ref(), "msg": msg}))
            .send(),
    );
}

pub async fn cache_json(
    db: &Database,
    key: &str,
    value: &serde_json::Value,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
INSERT INTO c_json_cache ("key", "value", "when")
VALUES ($1, $2, NOW())
ON CONFLICT ("key")
DO UPDATE SET "value" = $2, "when" = NOW()
"#,
        key,
        value
    )
    .execute(db)
    .await?;

    Ok(())
}

pub enum CachedJson {
    Current(serde_json::Value),
    Expired(serde_json::Value),
    Missing,
}

pub async fn cached_json(db: &Database, key: &str, days: i32) -> Result<CachedJson, sqlx::Error> {
    let cached = sqlx::query!(
        r#"SELECT "value" AS "value!", (CURRENT_TIMESTAMP - make_interval(0, 0, 0, $2)) < "when" AS "current!" FROM c_json_cache WHERE "key" = $1 LIMIT 1"#,
        key,
        days,
    )
        .map(|row| if row.current {CachedJson::Current(row.value)} else {CachedJson::Expired(row.value)})
    .fetch_optional(db)
    .await?;

    Ok(cached.unwrap_or(CachedJson::Missing))
}

static FIXUP_LINE_BREAK_ENDING_STRONG: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            < br /? > \s* < / strong >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static FIXUP_ASTERISK: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            \*
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static SPACE: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            (?:\s | \r | \n | \t | &nbsp; )+
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static HEADER_START: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            <h (?<level>\d) .*? >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static HEADER_END: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            </h (?<level>\d) .*? >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static STRONG: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            < (?:b|strong) > \s* |
            \s* < / (?:b|strong) >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static EMPHATIC: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            < (?:i|em) > \s* |
            \s* < /? (?:i|em) >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static LINK: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            <a \s+ .*? href= (?:"|')? (?<link>[^"'>\s]*) .*?>
              \s*
              (?<text>.*?)
              \s*
            </a>
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static IMAGE: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            <img \s+ .*? src= (?:"|')? (?<link>[^"'>\s]+) .*? >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static PARAGRAPH: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            < /p > |
            < br /? >
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static JUNK: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            (?: < .*? > )+
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

static POST_INDENT: Lazy<Regex> = Lazy::new(|| {
    RegexBuilder::new(
        r#"
            \n (?: \x20 | \t )+
        "#,
    )
    .case_insensitive(true)
    .ignore_whitespace(true)
    .build()
    .unwrap()
});

/// Converts HTML (sent by some partners for opportunity descriptions)
/// into MarkDown (required by our specification, but sometimes that's
/// ignored). This is a heuristic, and may produce weird results when
/// presented with unusual inputs.
///
/// Not a sanitizer. Results of untrusted inputs should be run through
/// Ammonia.
pub fn html_to_md(input: &str) -> String {
    let output = FIXUP_ASTERISK.replace_all(input, r"\*");

    let output = FIXUP_LINE_BREAK_ENDING_STRONG.replace_all(&output, "</strong><br>");

    let output = SPACE.replace_all(&output, " ");

    let output = HEADER_START.replace_all(&output, |c: &Captures<'_>| {
        format!(
            "\n\n{} ",
            "#".repeat(
                c.name("level")
                    .map_or(1, |m| m.as_str().parse().unwrap_or(1))
            )
        )
    });

    let output = HEADER_END.replace_all(&output, "\n\n");

    let output = STRONG.replace_all(&output, "**");

    let output = EMPHATIC.replace_all(&output, "*");

    let output = LINK.replace_all(&output, |c: &Captures<'_>| {
        if c.name("link").map_or(true, |m| m.as_str().is_empty())
            || c.name("text").map_or(true, |m| m.as_str().is_empty())
        {
            String::new()
        } else {
            format!(
                "[{}]({})",
                c.name("text").map_or("", |m| m.as_str()),
                c.name("link").map_or("", |m| m.as_str())
            )
        }
    });

    let output = IMAGE.replace_all(&output, "![]($link)");

    let output = PARAGRAPH.replace_all(&output, "\n\n");

    let output = JUNK.replace_all(&output, " ");

    let output = POST_INDENT.replace_all(&output, "\n");

    output.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn soup_html_to_md() {
        let soup = "<div  class=\"tribe-events-schedule tribe-clearfix\" >\n\t<h2 class=\"tribe-events-schedule__datetime\">\n\t\t<span class=\"tribe-events-schedule__date tribe-events-schedule__date--start\">\n\t\t\t\t\t</span>\n\n\t\t\t\t\t<span class=\"tribe-events-schedule__separator tribe-events-schedule__separator--date\">\n\t\t\t\t @ \t\t\t</span>\n\t\t\t<span class=\"tribe-events-schedule__time tribe-events-schedule__time--start\">\n\t\t\t\t\t\t\t</span>\n\t\t\n\t\t\t</h2>\n</div>\n\n\n\n<p><strong>Sensory-Friendly Sunday<br></strong>MODS will open an hour early for sensory-friendly Museum exploration and activities followed by an IMAX film* at 12:30 p.m.</p>\n\n\n\n<p>To provide a more sensory-sensitive environment for individuals on the autism spectrum, the theater lights will be turned up and the sound will be lowered. All films will be presented in 2D. No trailers will be shown before the movie. Audience members are welcome to get up, sing, walk, dance and shout without worrying about someone complaining. Families also can bring snacks from home, as well as blankets and other small items to be comfortable.</p>\n\n\n\n<p>Guests also can participate in a variety of sensory-based activities throughout the Museum. Children will engage in social interactions and improve the processing, modulation and regulation of sensory input to increase participation in a natural environment.</p>\n\n\n\n<p>Film Includes:</p>\n\n\n\n<figure class=\"wp-block-embed is-type-video is-provider-youtube wp-block-embed-youtube wp-embed-aspect-16-9 wp-has-aspect-ratio\"><div class=\"wp-block-embed__wrapper\">\n<iframe loading=\"lazy\" title=\"Jane Goodall - Reasons For Hope Trailer\" width=\"500\" height=\"281\" src=\"https://www.youtube.com/embed/SKWK7xpVe88?feature=oembed\" frameborder=\"0\" allow=\"accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share\" allowfullscreen></iframe>\n</div></figure>\n\n\n\n<p><strong>COST:</strong><br><strong>MODS Member:</strong>&nbsp;Free<br><strong>Non-members:</strong>&nbsp;Free</p>\n\n\n\n<div class=\"wp-block-buttons is-layout-flex wp-block-buttons-is-layout-flex\">\n<div class=\"wp-block-button\"><a class=\"wp-block-button__link wp-element-button\" href=\"https://989.blackbaudhosting.com/989/tickets?tab=2&amp;txobjid=70aea680-eaee-4ab4-8ccf-708f55c4217a\" target=\"_blank\" rel=\"noopener\">Register Here</a></div>\n</div>\n\n\n\n<div style=\"height:32px\" aria-hidden=\"true\" class=\"wp-block-spacer\"></div>\n\n\n\n<p><strong>Sponsored by&nbsp;</strong></p>\n\n\n\n<div class=\"wp-block-group is-nowrap is-layout-flex wp-container-2 wp-block-group-is-layout-flex\">\n<figure class=\"wp-block-image is-resized\"><img decoding=\"async\" loading=\"lazy\" src=\"https://mods.org/wp-content/uploads/2022/11/Sensory-Friendly-BBX-378-x-198-1.png\" alt=\"\" class=\"wp-image-10018\" style=\"width:181px;height:60px\" width=\"181\" height=\"60\" srcset=\"https://mods.org/wp-content/uploads/2022/11/Sensory-Friendly-BBX-378-x-198-1.png 347w, https://mods.org/wp-content/uploads/2022/11/Sensory-Friendly-BBX-378-x-198-1-300x99.png 300w\" sizes=\"(max-width: 181px) 100vw, 181px\" /></figure>\n</div>\n\n\n\n<p>*Film is subject to change.</p>\n\n\n<div  class=\"tribe-block tribe-block__event-price\" >\n\t\t</div>\n\n\n<div  class=\"tribe-block tribe-block__organizer__details tribe-clearfix\" >\n\t<div class=\"tribe-block__organizer__title\">\n\t\t<h3><a href=\"https://mods.org/organizer/museum-of-discovery-science/\" title=\"Museum of Discovery &#038; Science\" target=\"_self\" rel=\"\">Museum of Discovery &#038; Science</a></h3>\n\t</div>\n\t\t\t<p class=\"tribe-block__organizer__phone\">954-467-6637</p>\n\t\t\t\t<p class=\"tribe-block__organizer__website\"><a href=\"https://mods.org\" target=\"_self\" rel=\"external\">View Organizer Website</a></p>\n\t\t</div>\n\n\n<div  class=\"tribe-block tribe-block__venue tribe-clearfix\" >\n\t\n\t\t\n\t</div>\n\n\n\n\n\n\n\t<div  class=\"tribe-block tribe-block__events-link\" >\n\t\t<div class=\"tribe-events tribe-common\">\n\t\t\t<div class=\"tribe-events-c-subscribe-dropdown__container\">\n\t\t\t\t<div class=\"tribe-events-c-subscribe-dropdown\">\n\t\t\t\t\t<div class=\"tribe-common-c-btn-border tribe-events-c-subscribe-dropdown__button\">\n\t\t\t\t\t\t<svg  class=\"tribe-common-c-svgicon tribe-common-c-svgicon--cal-export tribe-events-c-subscribe-dropdown__export-icon\"  viewBox=\"0 0 23 17\" xmlns=\"http://www.w3.org/2000/svg\">\n  <path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M.128.896V16.13c0 .211.145.383.323.383h15.354c.179 0 .323-.172.323-.383V.896c0-.212-.144-.383-.323-.383H.451C.273.513.128.684.128.896Zm16 6.742h-.901V4.679H1.009v10.729h14.218v-3.336h.901V7.638ZM1.01 1.614h14.218v2.058H1.009V1.614Z\" />\n  <path d=\"M20.5 9.846H8.312M18.524 6.953l2.89 2.909-2.855 2.855\" stroke-width=\"1.2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"/>\n</svg>\n\t\t\t\t\t\t<button\n\t\t\t\t\t\t\tclass=\"tribe-events-c-subscribe-dropdown__button-text\"\n\t\t\t\t\t\t\taria-expanded=\"false\"\n\t\t\t\t\t\t\taria-controls=\"tribe-events-subscribe-dropdown-content\"\n\t\t\t\t\t\t\taria-label=\"\"\n\t\t\t\t\t\t>\n\t\t\t\t\t\t\tAdd to calendar\t\t\t\t\t\t</button>\n\t\t\t\t\t\t<svg  class=\"tribe-common-c-svgicon tribe-common-c-svgicon--caret-down tribe-events-c-subscribe-dropdown__button-icon\"  viewBox=\"0 0 10 7\" xmlns=\"http://www.w3.org/2000/svg\"><path fill-rule=\"evenodd\" clip-rule=\"evenodd\" d=\"M1.008.609L5 4.6 8.992.61l.958.958L5 6.517.05 1.566l.958-.958z\" class=\"tribe-common-c-svgicon__svg-fill\"/></svg>\n\t\t\t\t\t</div>\n\t\t\t\t\t<div id=\"tribe-events-subscribe-dropdown-content\" class=\"tribe-events-c-subscribe-dropdown__content\">\n\t\t\t\t\t\t<ul class=\"tribe-events-c-subscribe-dropdown__list\">\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t<li class=\"tribe-events-c-subscribe-dropdown__list-item\">\n\t\t\t\t\t\t\t\t\t<a\n\t\t\t\t\t\t\t\t\t\thref=\"\"\n\t\t\t\t\t\t\t\t\t\tclass=\"tribe-events-c-subscribe-dropdown__list-item-link\"\n\t\t\t\t\t\t\t\t\t\ttarget=\"_blank\"\n\t\t\t\t\t\t\t\t\t\trel=\"noopener noreferrer nofollow noindex\"\n\t\t\t\t\t\t\t\t\t>\n\t\t\t\t\t\t\t\t\t\tGoogle Calendar\t\t\t\t\t\t\t\t\t</a>\n\t\t\t\t\t\t\t\t</li>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t<li class=\"tribe-events-c-subscribe-dropdown__list-item\">\n\t\t\t\t\t\t\t\t\t<a\n\t\t\t\t\t\t\t\t\t\thref=\"\"\n\t\t\t\t\t\t\t\t\t\tclass=\"tribe-events-c-subscribe-dropdown__list-item-link\"\n\t\t\t\t\t\t\t\t\t\ttarget=\"_blank\"\n\t\t\t\t\t\t\t\t\t\trel=\"noopener noreferrer nofollow noindex\"\n\t\t\t\t\t\t\t\t\t>\n\t\t\t\t\t\t\t\t\t\tiCalendar\t\t\t\t\t\t\t\t\t</a>\n\t\t\t\t\t\t\t\t</li>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t<li class=\"tribe-events-c-subscribe-dropdown__list-item\">\n\t\t\t\t\t\t\t\t\t<a\n\t\t\t\t\t\t\t\t\t\thref=\"\"\n\t\t\t\t\t\t\t\t\t\tclass=\"tribe-events-c-subscribe-dropdown__list-item-link\"\n\t\t\t\t\t\t\t\t\t\ttarget=\"_blank\"\n\t\t\t\t\t\t\t\t\t\trel=\"noopener noreferrer nofollow noindex\"\n\t\t\t\t\t\t\t\t\t>\n\t\t\t\t\t\t\t\t\t\tOutlook 365\t\t\t\t\t\t\t\t\t</a>\n\t\t\t\t\t\t\t\t</li>\n\t\t\t\t\t\t\t\t\t\t\t\t\t\t\t<li class=\"tribe-events-c-subscribe-dropdown__list-item\">\n\t\t\t\t\t\t\t\t\t<a\n\t\t\t\t\t\t\t\t\t\thref=\"\"\n\t\t\t\t\t\t\t\t\t\tclass=\"tribe-events-c-subscribe-dropdown__list-item-link\"\n\t\t\t\t\t\t\t\t\t\ttarget=\"_blank\"\n\t\t\t\t\t\t\t\t\t\trel=\"noopener noreferrer nofollow noindex\"\n\t\t\t\t\t\t\t\t\t>\n\t\t\t\t\t\t\t\t\t\tOutlook Live\t\t\t\t\t\t\t\t\t</a>\n\t\t\t\t\t\t\t\t</li>\n\t\t\t\t\t\t\t\t\t\t\t\t\t</ul>\n\t\t\t\t\t</div>\n\t\t\t\t</div>\n\t\t\t</div>\n\t\t</div>\n\t</div>";

        // This 'test' is supposed to fail, so we can see the output
        panic!("HTML->Markdown:\n{}", html_to_md(soup));
    }
}
