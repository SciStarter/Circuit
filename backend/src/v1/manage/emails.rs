use common::emails::EmailMessage;
use common::model::person::Permission;
use common::Database;
use http_types::{Method, StatusCode};
use sailfish::TemplateOnce;
use serde::Deserialize;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

use crate::v1::redirect;

use super::{authorized_admin, IntoResponse};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(emails)
        .post(emails)
        .at(":slug", |r| r.get(email).post(email))
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/emails.stpl.html")]
struct EmailsPage {
    slugs: Vec<String>,
}

#[derive(Deserialize)]
struct EmailsForm {
    slug: String,
}

async fn emails(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: EmailsForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}", req.url().path(), form.slug)));
    }

    let db = req.state();

    let slugs = EmailMessage::list_messages(db).await?;

    Ok(EmailsPage { slugs }.into_response(StatusCode::Ok)?)
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/email.stpl.html")]
struct EmailPage {
    slug: String,
    subject: String,
    body: String,
    notes: String,
}

#[derive(Deserialize)]
struct EmailForm {
    subject: String,
    body: String,
    notes: String,
}

async fn email(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let slug = req.param("slug")?.to_owned();

    let mut message = match EmailMessage::load(req.state(), &slug).await {
        Ok(message) => message,
        Err(_) => EmailMessage::new(slug, "", ""),
    };

    if let Method::Post = req.method() {
        let form: EmailForm = req.body_form().await?;
        message.subject = form.subject;
        message.body = form.body;
        message.notes = form.notes;
        message
            .store(req.state())
            .await
            .map_err(|err| tide::Error::from_debug(err))?;
        return Ok(redirect(""));
    } else {
        Ok(EmailPage {
            slug: message.slug().unwrap(),
            subject: message.subject,
            body: message.body,
            notes: message.notes,
        }
        .into_response(StatusCode::Ok)?)
    }
}
