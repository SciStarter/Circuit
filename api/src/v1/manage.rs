use super::{check_jwt, issue_jwt, random_string, redirect, set_csrf_cookie};
use crate::model::{partner::PartnerReference, person::Permission, Partner, Person};
use askama::Template;
use sqlx::postgres::Postgres;
use sqlx::prelude::*;
use tide::{http::Cookie, prelude::*};
use tide::{Response, StatusCode};
use tide_fluent_routes::prelude::*;
use tide_sqlx::SQLxRequestExt;

const BASE: &'static str = "/api/v1/manage";

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .at("authorize", |r| r.get(authorize).post(authorize))
        .at("partners", |r| {
            r.get(partners)
                .post(partners)
                .at(":uid", |r| r.get(partner).post(partner))
        })
}

#[derive(Template, Default, Serialize, Deserialize)]
#[template(path = "manage/authorize.html")]
struct AuthorizeForm {
    next: Option<String>,
    error: Option<String>,
    email: Option<String>,
    password: Option<String>,
    csrf: Option<String>,
}

async fn authorize(mut req: tide::Request<()>) -> tide::Result {
    match req.method() {
        Method::Get => {
            let csrf = random_string();
            let mut form: AuthorizeForm = req.query()?;
            form.csrf = Some(csrf.clone());
            Ok(set_csrf_cookie(form.into(), &csrf))
        }
        Method::Post => {
            let mut form: AuthorizeForm = req.body_form().await?;

            if form.csrf != req.cookie("csrftoken").map(|c| c.value().to_string()) {
                return Ok("CSRF validation failed".into());
            }

            if let (Some(email), Some(password)) = (&form.email, &form.password) {
                let mut db = req.sqlx_conn::<Postgres>().await;

                let person = Person::load_by_email(db.acquire().await?, email).await?;
                if person.check_password(password) {
                    if person.check_permission(&Permission::ManageSomething) {
                        return Ok(redirect("/"));
                    }
                    let mut resp = redirect(&form.next.unwrap_or_else(|| BASE.to_string()));
                    resp.insert_cookie(
                        Cookie::build("token", issue_jwt(&person.exterior.uid)?)
                            .path(BASE)
                            .secure(true)
                            .http_only(true)
                            .same_site(tide::http::cookies::SameSite::Strict)
                            .finish(),
                    );
                    return Ok(resp);
                } else {
                    form.error = Some("invalid username or password".to_string());
                    return Ok(form.into());
                }
            } else {
                form.error = Some("email and password are required".to_string());
                return Ok(form.into());
            }
        }
        _ => unimplemented!(),
    }
}

#[derive(Template, Default)]
#[template(path = "manage/partners.html")]
struct PartnersPage {
    pub partners: Vec<PartnerReference>,
    pub suggested_secret: String,
    pub csrf: String,
}

#[derive(Template, Default)]
#[template(path = "manage/partners_created.html")]
struct PartnersCreatedPage {
    pub name: String,
    pub uid: String,
    pub secret: String,
}

#[derive(Default, Serialize, Deserialize)]
struct PartnersForm {
    csrf: String,
    name: String,
    secret: String,
}

async fn partners(mut req: tide::Request<()>) -> tide::Result {
    let _manager = match authorized_manager(&req, &Permission::ManagePartners).await {
        Ok(uid) => uid,
        Err(resp) => return Ok(resp),
    };

    match req.method() {
        Method::Get => {
            let mut db = req.sqlx_conn::<Postgres>().await;

            let csrf = random_string();
            let page = PartnersPage {
                partners: Partner::catalog(db.acquire().await?).await?,
                suggested_secret: csrf.to_string(),
                csrf: csrf.to_string(),
            };
            Ok(set_csrf_cookie(page.into(), &csrf))
        }
        Method::Post => {
            let form: PartnersForm = req.body_form().await?;

            let mut partner = Partner::default();
            partner.exterior.name = form.name;
            partner.set_secret(&form.secret);

            let mut db = req.sqlx_conn::<Postgres>().await;
            partner.store(db.acquire().await?).await?;

            let page = PartnersCreatedPage {
                name: partner.exterior.name.to_string(),
                uid: partner.exterior.uid.to_string(),
                secret: form.secret,
            };
            Ok(page.into())
        }
        _ => unimplemented!(),
    }
}

async fn partner(_req: tide::Request<()>) -> tide::Result {
    todo!()
}

async fn authorized_manager(
    req: &tide::Request<()>,
    needed: &Permission,
) -> Result<Person, tide::Response> {
    let token = match req.cookie("token") {
        Some(token) => token.value().to_string(),
        None => return Err(redirect(&format!("{}/authorize", BASE))),
    };

    let uid = match check_jwt(&token) {
        Ok(Some(uid)) => uid,
        Ok(None) => return Err(redirect(&format!("{}/authorize", BASE))),
        Err(_) => return Err(redirect(&format!("{}/authorize", BASE))),
    };

    let mut db = req.sqlx_conn::<Postgres>().await;

    let person = match Person::load_by_uid(
        match db.acquire().await {
            Ok(x) => x,
            Err(_) => {
                return Err(Response::builder(StatusCode::InternalServerError)
                    .body("Error retrieving database connection from pool")
                    .build())
            }
        },
        &uid,
    )
    .await
    {
        Ok(person) => person,
        Err(_) => {
            return Err(Response::builder(StatusCode::InternalServerError)
                .body("Error retrieving person from database")
                .build())
        }
    };

    if !person.check_permission(needed) {
        return Err(redirect(&format!(
            "{}/authorize?next={}",
            BASE,
            urlencoding::encode(req.url().as_str())
        )));
    }

    Ok(person)
}
