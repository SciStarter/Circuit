use std::fmt::Display;

use aho_corasick::AhoCorasick;
use once_cell::sync::Lazy;
use sqlx::Error;

use crate::{BoxedError, Database};

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
    pub notes: String,
}

impl EmailMessage {
    pub async fn list_messages(db: &Database) -> Result<Vec<String>, Error> {
        Ok(sqlx::query!(
            r#"
SELECT "slug" from "c_email_message" ORDER BY "slug" ASC
"#
        )
        .fetch_all(db)
        .await?
        .iter()
        .map(|x| x.slug.to_string())
        .collect())
    }

    pub fn new<S0: AsRef<str>, S1: AsRef<str>, S2: AsRef<str>>(
        slug: S0,
        subject: S1,
        body: S2,
    ) -> Self {
        EmailMessage {
            slug: Some(slug.as_ref().to_string()),
            subject: subject.as_ref().to_string(),
            body: body.as_ref().to_string(),
            notes: String::new(),
        }
    }

    pub async fn load<S: AsRef<str>>(db: &Database, slug: S) -> Result<EmailMessage, BoxedError> {
        Ok(sqlx::query_as!(
            EmailMessage,
            r#"
SELECT slug AS "slug?", subject, body, notes FROM c_email_message WHERE slug = $1
"#,
            slug.as_ref()
        )
        .fetch_one(db)
        .await?)
    }

    pub async fn load_or_default<Slug: AsRef<str>, Subject: AsRef<str>, Body: AsRef<str>>(
        db: &Database,
        slug: Slug,
        subject: Subject,
        body: Body,
    ) -> EmailMessage {
        match EmailMessage::load(db, slug).await {
            Ok(msg) => msg,
            Err(_) => EmailMessage {
                slug: None,
                subject: subject.as_ref().to_string(),
                body: body.as_ref().to_string(),
                notes: String::new(),
            },
        }
    }

    pub async fn store(&self, db: &Database) -> Result<(), BoxedError> {
        if let Some(slug) = &self.slug {
            sqlx::query!(
                r"
INSERT INTO c_email_message (slug, subject, body, notes)
VALUES ($1, $2, $3, $4)
ON CONFLICT (slug) DO
UPDATE SET subject = $2, body = $3, notes = $4",
                slug,
                &self.subject,
                &self.body,
                &self.notes,
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

    pub fn materialize<S0, S1, B>(&self, bindings: B) -> Self
    where
        S0: Display,
        S1: Display,
        B: IntoIterator<Item = (S0, S1)>,
    {
        let (patterns, replacements): (Vec<_>, Vec<_>) = bindings
            .into_iter()
            .map(|(k, v)| (format!("{{{k}}}"), v.to_string()))
            .unzip();

        let ac = AhoCorasick::new(patterns);

        EmailMessage {
            slug: None,
            subject: ac.replace_all(&self.subject, &replacements),
            body: ac.replace_all(&self.body, &replacements),
            notes: self.notes.clone(),
        }
    }

    pub fn slug(&self) -> Option<String> {
        self.slug.clone()
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

pub async fn send_message<S0: AsRef<str>>(to: S0, msg: &EmailMessage) {
    send(
        to,
        "Science Near Me <info@sciencenearme.org>",
        &msg.subject,
        &msg.body,
    )
    .await
}
