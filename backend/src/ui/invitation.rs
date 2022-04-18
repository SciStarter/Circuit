use common::{
    jwt::issue_jwt,
    model::{invitation::*, Partner, Person},
    Database,
};
use http_types::{mime, StatusCode};
use sailfish::TemplateOnce;
use tide::Response;
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

use super::{
    auth::{token_cookie, SESSION_HOURS},
    UI_AUDIENCE,
};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.at(":uid", |r| r.get(dispatch))
}

#[derive(TemplateOnce, Default)]
#[template(path = "invitation/password_reset.stpl")]
struct ResetPage {
    pub jwt: String,
}

async fn password_reset(req: &mut tide::Request<Database>, inv: Invitation) -> tide::Result {
    let person = Person::load_by_uid(req.state(), &inv.target()).await?;

    let jwt = issue_jwt(&person.exterior.uid, &UI_AUDIENCE, SESSION_HOURS as u64)?;
    let page = ResetPage { jwt: jwt.clone() };
    let mut resp = Response::builder(StatusCode::Ok)
        .content_type(mime::HTML)
        .body(page.render_once()?)
        .build();
    resp.insert_cookie(token_cookie(jwt));

    if let Err((_, err)) = inv.consume(req.state()).await {
        Err(err.into())
    } else {
        Ok(resp)
    }
}

async fn join_organization(req: &mut tide::Request<Database>, inv: Invitation) -> tide::Result {
    if let Some(person) = super::request_person(req).await? {
        let mut org = Partner::load_by_uid(req.state(), &inv.target()).await?;

        org.set_authorized(person.exterior.uid);
        org.store(req.state()).await?;

        if let Err((_, err)) = inv.consume(req.state()).await {
            Err(err.into())
        } else {
            Ok(tide::Redirect::see_other("/my/opportunities").into())
        }
    } else {
        Ok(tide::Redirect::temporary(&format!("/login?next={}", req.url().path())).into())
    }
}

pub async fn dispatch(mut req: tide::Request<Database>) -> tide::Result {
    use InvitationMode::*;

    let uid = Uuid::parse_str(req.param("uid")?)?;

    if let Some(inv) = Invitation::load(req.state(), uid).await? {
        match inv.mode() {
            PasswordReset => password_reset(&mut req, inv).await,
            JoinOrganization => join_organization(&mut req, inv).await,
        }
    } else {
        Ok("Invalid or already used".into())
    }
}
