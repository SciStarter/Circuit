use super::{check_csrf, check_jwt, issue_jwt, random_string, redirect, set_csrf_cookie};
use askama::Template;
use common::model::{partner::PartnerReference, person::Permission, Partner, Person};
use common::Database;
use once_cell::sync::Lazy;
use tide::{http::Cookie, prelude::*};
use tide::{Response, StatusCode};
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

pub mod content;
pub mod opportunities;

const BASE: &'static str = "/api/v1/manage/";

static MANAGE_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("51456ff1-ff31-4d99-a550-7325e5e728a5").unwrap());

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(manage)
        .at("authorize", |r| r.get(authorize).post(authorize))
        .at("partners/", |r| {
            r.get(partners)
                .post(partners)
                .at(":uid", |r| r.get(partner).post(partner))
        })
        .at("content/", content::routes)
        .at("opportunities/", opportunities::routes)
}

#[derive(Template)]
#[template(path = "manage/manage.html")]
struct ManagePage {
    pub admin: Person,
}

async fn manage(req: tide::Request<Database>) -> tide::Result {
    let admin = match authorized_admin(&req, &Permission::ManageSomething).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let page = ManagePage { admin };

    Ok(page.into())
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

async fn authorize(mut req: tide::Request<Database>) -> tide::Result {
    match req.method() {
        Method::Get => {
            let csrf = random_string();
            let mut form: AuthorizeForm = req.query()?;
            form.csrf = Some(csrf.clone());
            Ok(set_csrf_cookie(form.into(), &csrf))
        }
        Method::Post => {
            let mut form: AuthorizeForm = req.body_form().await?;

            if let Some(csrf) = &form.csrf {
                if !check_csrf(&req, csrf) {
                    return Ok("CSRF validation failed".into());
                }
            } else {
                return Ok("CSRF validation failed".into());
            }

            if let (Some(email), Some(password)) = (&form.email, &form.password) {
                let db = req.state();

                let person = Person::load_by_email(db, email).await?;
                if person.check_password(password) {
                    if !person.check_permission(&Permission::ManageSomething) {
                        return Ok(redirect("/"));
                    }
                    let mut resp = redirect(&form.next.unwrap_or_else(|| BASE.to_string()));
                    resp.insert_cookie(
                        Cookie::build(
                            "manage",
                            issue_jwt(&person.exterior.uid, &MANAGE_AUDIENCE, 6)?,
                        )
                        .domain(std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()))
                        .path(BASE)
                        .secure(cfg!(not(debug_assertions))) // Allow HTTP when in debug mode, require HTTPS in release mode
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
    manager_name: String,
    manager_email: String,
    manager_phone: Option<String>,
    manager_mailing: Option<String>,
}

async fn partners(mut req: tide::Request<Database>) -> tide::Result {
    let admin = match authorized_admin(&req, &Permission::ManagePartners).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    match req.method() {
        Method::Get => {
            let db = req.state();

            let csrf = random_string();
            let page = PartnersPage {
                partners: Partner::catalog(db).await?,
                suggested_secret: csrf.to_string(),
                csrf: csrf.to_string(),
            };
            Ok(set_csrf_cookie(page.into(), &csrf))
        }
        Method::Post => {
            let form: PartnersForm = req.body_form().await?;

            if !check_csrf(&req, &form.csrf) {
                return Ok("CSRF validation failed".into());
            }

            let mut partner = Partner::default();
            partner.interior.prime = admin.exterior.uid.clone();
            partner.exterior.name = form.name;
            partner.set_secret(&form.secret);
            partner.interior.manager.name = form.manager_name;
            partner.interior.manager.email = form.manager_email;
            partner.interior.manager.phone = form.manager_phone;
            partner.interior.manager.mailing = form.manager_mailing;

            let db = req.state();
            partner.store(db).await?;

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

#[derive(Template, Default)]
#[template(path = "manage/partner.html")]
struct PartnerPage {
    partner: Partner,
}

async fn partner(req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManagePartners).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let uid = Uuid::parse_str(req.param("uid")?)?;

    let db = req.state();

    let partner = Partner::load_by_uid(db, &uid).await?;

    let page = PartnerPage { partner };

    Ok(page.into())
}

async fn authorized_admin(
    req: &tide::Request<Database>,
    needed: &Permission,
) -> Result<Person, tide::Response> {
    let token = match req.cookie("manage") {
        Some(token) => token.value().to_string(),
        None => return Err(redirect(&format!("{}authorize", BASE))),
    };

    let uid = match check_jwt(&token, &MANAGE_AUDIENCE) {
        Ok(uid) => uid,
        Err(_) => return Err(redirect(&format!("{}authorize", BASE))),
    };

    let db = req.state();

    let person = match Person::load_by_uid(db, &uid).await {
        Ok(person) => person,
        Err(_) => {
            return Err(Response::builder(StatusCode::InternalServerError)
                .body("Error retrieving person from database")
                .build())
        }
    };

    if !person.check_permission(needed) {
        return Err(redirect(&format!(
            "{}authorize?next={}",
            BASE,
            urlencoding::encode(req.url().as_str())
        )));
    }

    Ok(person)
}
