use std::collections::HashMap;

use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;

use crate::Database;

static MAILER_ENDPOINT: Lazy<String> = Lazy::new(|| {
    format!(
        "http://{}:{}/enqueue",
        std::env::var("CIRCUIT_MAILER_SERVICE_SERVICE_HOST")
            .unwrap_or_else(|_| "localhost".to_string()),
        std::env::var("CIRCUIT_MAILER_SERVICE_SERVICE_PORT").unwrap_or_else(|_| "9100".to_string()),
    )
});

pub struct EmailMessage {
    slug: Option<String>,
    pub subject: String,
    pub body: String,
}

struct EmailMessageFromDB {
    slug: String,
    subject: String,
    body: String,
}

impl From<EmailMessageFromDB> for EmailMessage {
    fn from(src: EmailMessageFromDB) -> Self {
        EmailMessage {
            slug: Some(src.slug),
            subject: src.subject,
            body: src.body,
        }
    }
}

impl EmailMessage {
    pub fn new<S0: AsRef<str>, S1: AsRef<str>, S2: AsRef<str>>(
        slug: S0,
        subject: S1,
        body: S2,
    ) -> Self {
        EmailMessage {
            slug: Some(slug.as_ref().to_string()),
            subject: subject.as_ref().to_string(),
            body: body.as_ref().to_string(),
        }
    }

    pub async fn load<S: AsRef<str>>(
        db: &Database,
        slug: S,
    ) -> Result<EmailMessage, Box<dyn std::error::Error>> {
        Ok(sqlx::query_as!(
            EmailMessageFromDB,
            "SELECT slug, subject, body FROM c_email_message WHERE slug = $1",
            slug.as_ref()
        )
        .fetch_one(db)
        .await?
        .into())
    }

    pub async fn store(&self, db: &Database) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(slug) = &self.slug {
            sqlx::query!(
                r"INSERT INTO c_email_message (slug, subject, body)
                  VALUES ($1, $2, $3)
                  ON CONFLICT (slug)
                  DO UPDATE SET subject = $2, body = $3 WHERE c_email_message.slug = $1",
                slug,
                &self.subject,
                &self.body
            )
            .execute(db)
            .await?;
            Ok(())
        } else {
            return Err(Box::new(crate::Error::Email(
                "Can't save an EmailMessage without a slug".to_string(),
            )));
        }
    }

    pub fn materialize(&self, bindings: &HashMap<String, String>) -> Self {
        let (patterns, replacements): (Vec<_>, Vec<_>) = bindings
            .iter()
            .map(|(k, v)| (format!("{{{k}}}"), v))
            .unzip();

        let ac = AhoCorasick::new(patterns);

        EmailMessage {
            slug: None,
            subject: ac.replace_all(&self.subject, &replacements),
            body: ac.replace_all(&self.body, &replacements),
        }
    }
}

pub async fn send<S0: AsRef<str>, S1: AsRef<str>, S2: AsRef<str>, S3: AsRef<str>>(
    to: S0,
    from: S1,
    subject: S2,
    body: S3,
) {
    async_std::task::spawn(
        surf::post(&*MAILER_ENDPOINT)
            .body(serde_json::json!({"to": to.as_ref(), "from": from.as_ref(), "subject": subject.as_ref(), "body": body.as_ref()}))
            .send(),
    );
}
