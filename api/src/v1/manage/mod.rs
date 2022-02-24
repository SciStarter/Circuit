use super::{check_csrf, check_jwt, issue_jwt, random_string, redirect, set_csrf_cookie};
use askama::Template;
use common::model::Pagination;
use common::model::{partner::PartnerReference, person::Permission, Partner, Person};
use common::Database;
use once_cell::sync::Lazy;
use tide::{http::Cookie, prelude::*};
use tide::{Response, StatusCode};
use tide_fluent_routes::prelude::*;
use uuid::Uuid;

pub mod content;
pub mod emails;
pub mod opportunities;

const BASE: &'static str = "/api/v1/manage/";

#[cfg(not(debug_assertions))]
const MANAGE_COOKIE: &'static str = "__Host-manage";
#[cfg(debug_assertions)]
const MANAGE_COOKIE: &'static str = "manage";

static MANAGE_AUDIENCE: Lazy<uuid::Uuid> =
    Lazy::new(|| uuid::Uuid::parse_str("51456ff1-ff31-4d99-a550-7325e5e728a5").unwrap());

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(manage)
        .at("authorize", |r| r.get(authorize).post(authorize))
        .at("persons/", |r| {
            r.get(persons)
                .post(persons)
                .at(":uid", |r| r.get(person).post(person))
        })
        .at("partners/", |r| {
            r.get(partners)
                .post(partners)
                .at(":uid", |r| r.get(partner).post(partner))
        })
        .at("content/", content::routes)
        .at("emails/", emails::routes)
        .at("opportunities/", opportunities::routes)
        .at("health/", |r| r.get(health))
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

#[derive(Template, Default, Serialize, Deserialize, Debug)]
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

                let person = match Person::load_by_email(db, email).await {
                    Ok(p) => p,
                    Err(_) => return Ok("invalid username or password".into()),
                };

                if person.check_password(password) {
                    if !person.check_permission(&Permission::ManageSomething) {
                        return Ok(redirect("/"));
                    }
                    let mut resp = redirect(&form.next.unwrap_or_else(|| BASE.to_string()));
                    resp.insert_cookie(
                        Cookie::build(
                            MANAGE_COOKIE,
                            issue_jwt(&person.exterior.uid, &MANAGE_AUDIENCE, 6)?,
                        )
                        //.domain(std::env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()))
                        .path("/")
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
            let secret = random_string();
            let page = PartnersPage {
                partners: Partner::catalog(db).await?,
                suggested_secret: secret.to_string(),
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
    let token = match req.cookie(MANAGE_COOKIE) {
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

#[derive(Template, Default)]
#[template(path = "manage/persons.html")]
struct PersonsPage {
    pub persons: Vec<Person>,
    pub suggested_password: String,
    pub csrf: String,
    pub total: u32,
    pub page_size: u32,
    pub cur_page: u32,
    pub last_page: u32,
}

#[derive(Default, Serialize, Deserialize)]
struct PersonsForm {
    csrf: String,
    name: String,
    email: String,
    password: String,
}

async fn persons(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManagePersons).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let pagination: Pagination = match req.query() {
        Ok(pagination) => pagination,
        Err(_) => Pagination::default(),
    };

    match req.method() {
        Method::Get => {
            let db = req.state();

            let total = Person::total(db).await?;
            let (cur_page, last_page, page_size) = pagination.expand(total);

            let csrf = random_string();
            let password = random_string();
            let page = PersonsPage {
                persons: Person::catalog(db, pagination).await?,
                suggested_password: password.to_string(),
                csrf: csrf.to_string(),
                total,
                page_size,
                cur_page,
                last_page,
            };
            Ok(set_csrf_cookie(page.into(), &csrf))
        }
        Method::Post => {
            let form: PersonsForm = req.body_form().await?;

            if !check_csrf(&req, &form.csrf) {
                return Ok("CSRF validation failed".into());
            }

            let mut person = Person::default();

            person.exterior.username = Some(form.name);
            person.interior.email = form.email;

            person.set_password(&form.password);

            let db = req.state();

            person.store(db).await?;

            Ok(redirect(&person.exterior.uid.to_string()))
        }
        _ => unimplemented!(),
    }
}

#[derive(Template, Default)]
#[template(path = "manage/person.html")]
struct PersonPage {
    pub person: Person,
    pub csrf: String,
}

#[derive(Default, Serialize, Deserialize)]
struct PersonForm {
    csrf: String,
    username: String,
    email: String,
    manage_opportunitues: Option<bool>,
    manage_partners: Option<bool>,
    manage_persons: Option<bool>,
    manage_content: Option<bool>,
    new_password: Option<String>,
}

fn set_permission(person: &mut Person, perm: Permission, state: bool) {
    match (
        state,
        person.interior.permissions.iter().any(|x| x == &perm),
    ) {
        (true, true) => {}
        (true, false) => person.interior.permissions.push(perm),
        (false, true) => {
            person.interior.permissions = person
                .interior
                .permissions
                .iter()
                .copied()
                .filter(|x| *x != perm)
                .collect();
        }
        (false, false) => {}
    }
}

async fn person(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManagePersons).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let uid = Uuid::parse_str(req.param("uid")?)?;

    match req.method() {
        Method::Get => {
            let db = req.state();
            let csrf = random_string();
            let person = Person::load_by_uid(db, &uid).await?;
            let page = PersonPage {
                person,
                csrf: csrf.clone(),
            };
            Ok(set_csrf_cookie(page.into(), &csrf))
        }
        Method::Post => {
            let form: PersonForm = req.body_form().await?;

            if !check_csrf(&req, &form.csrf) {
                return Ok("CSRF validation failed".into());
            }

            let db = req.state();
            let mut person = Person::load_by_uid(db, &uid).await?;

            person.exterior.username = Some(form.username);
            person.interior.email = form.email;

            set_permission(
                &mut person,
                Permission::ManageContent,
                form.manage_content.unwrap_or(false),
            );
            set_permission(
                &mut person,
                Permission::ManagePartners,
                form.manage_partners.unwrap_or(false),
            );
            set_permission(
                &mut person,
                Permission::ManagePersons,
                form.manage_persons.unwrap_or(false),
            );
            set_permission(
                &mut person,
                Permission::ManageOpportunities,
                form.manage_opportunitues.unwrap_or(false),
            );

            if let Some(password) = form.new_password {
                if !password.is_empty() {
                    person.set_password(&password);
                }
            }

            person.store(db).await?;

            Ok(redirect(&person.exterior.uid.to_string()))
        }
        _ => unimplemented!(),
    }
}

mod filters {
    pub trait Value {
        fn as_f32(&self) -> f32;
    }

    impl Value for f32 {
        fn as_f32(&self) -> f32 {
            *self
        }
    }

    impl Value for &f32 {
        fn as_f32(&self) -> f32 {
            **self
        }
    }

    pub fn health<V: Value>(ratio: V) -> ::askama::Result<String> {
        let ratio = ratio.as_f32();
        let color = if ratio < 0.5 {
            format!("#FF{:02X}00", (ratio * 512.0) as u8)
        } else {
            format!("#{:02X}FF00", ((1.0 - ratio) * 512.0) as u8)
        };

        Ok(format!(
            r#"<div style="text-align: center; background-color: {}"><span style="color: #fff; font-weight: bold; mix-blend-mode: exclusion">{:.1}%</span></div>"#,
            &color,
            ratio * 100.0
        ))
    }
}

#[derive(Template)]
#[template(path = "manage/health.html")]
struct HealthPage {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub current: Option<bool>,
    pub partner: Option<Uuid>,
    pub partners: Vec<PartnerReference>,
    pub num_matches: f32,
    pub partner_created: f32,
    pub partner_updated: f32,
    pub slug: f32,
    pub partner_name: f32,
    pub partner_website: f32,
    pub partner_logo_url: f32,
    pub partner_opp_url: f32,
    pub organization_name: f32,
    pub organization_type: f32,
    pub organization_website: f32,
    pub organization_logo_url: f32,
    pub opp_venue: f32,
    pub opp_descriptor: f32,
    pub min_age: f32,
    pub max_age: f32,
    pub pes_domain: f32,
    pub tags: f32,
    pub opp_topics: f32,
    pub ticket_required: f32,
    pub title: f32,
    pub description: f32,
    pub short_desc: f32,
    pub image_url: f32,
    pub image_credit: f32,
    pub start_datetimes: f32,
    pub end_datetimes: f32,
    pub attraction_hours: f32,
    pub cost: f32,
    pub languages: f32,
    pub opp_hashtags: f32,
    pub opp_social_handles: f32,
    pub is_online: f32,
    pub location_type: f32,
    pub location_point: f32,
    pub location_polygon: f32,
    pub address_street: f32,
    pub address_city: f32,
    pub address_state: f32,
    pub address_country: f32,
    pub address_zip: f32,
    pub contact_name: f32,
    pub contact_email: f32,
    pub contact_phone: f32,
}

#[derive(Deserialize)]
struct HealthForm {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub current: Option<bool>,
    pub partner: Option<Uuid>,
}

async fn health(req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let form: HealthForm = req.query()?;

    let mut narrow = "(exterior ->> 'entity_type') = 'opportunity'".to_string();
    let mut params = vec![];

    if let Some(uid) = &form.partner {
        narrow.push_str(" AND (exterior ->> 'partner') = $1");
        params.push(uid.to_string());
    }

    if let Some(accepted) = &form.accepted {
        if *accepted {
            narrow.push_str(" AND (interior ->> 'accepted') = 'true'");
        } else {
            narrow.push_str(" AND (interior ->> 'accepted') != 'true'");
        }
    }

    if let Some(withdrawn) = &form.accepted {
        if *withdrawn {
            narrow.push_str(" AND (interior ->> 'withdrawn') = 'true'");
        } else {
            narrow.push_str(" AND (interior ->> 'withdrawn') != 'true'");
        }
    }

    if let Some(current) = &form.current {
        if *current {
            narrow.push_str(" AND current = true");
        } else {
            narrow.push_str(" AND current = false");
        }
    }

    let db = req.state();

    async fn count(db: &Database, narrow: &str, params: &Vec<String>) -> Result<f32, sqlx::Error> {
        let query = format!("SELECT COUNT(*) FROM c_opportunity WHERE {}", narrow);
        let ret: i64 = params
            .iter()
            .fold(sqlx::query_scalar(&query), |q, p| q.bind(p))
            .fetch_one(db)
            .await?;
        Ok(ret as f32)
    }

    let num_matches = count(db, &narrow, &params).await?;

    Ok(HealthPage {
        accepted: form.accepted,
        withdrawn: form.withdrawn,
        current: form.current,
        partner: form.partner,
        partners: Partner::catalog(db).await?,
        num_matches,
        partner_created: {
            count(
                db,
                &format!(
                    "{} AND (exterior -> 'partner_created') != 'null'::jsonb",
                    &narrow
                ),
                &params,
            )
            .await?
                / num_matches
        },
        partner_updated: {
            count(
                db,
                &format!(
                    "{} AND (exterior -> 'partner_updated') != 'null'::jsonb",
                    &narrow
                ),
                &params,
            )
            .await?
                / num_matches
        },
        slug: {
            count(
                db,
                &format!("{} AND (exterior -> 'slug') != 'null'::jsonb AND (exterior ->> 'slug') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        partner_name: {
            count(
                db,
                &format!("{} AND (exterior -> 'partner_name') != 'null'::jsonb AND (exterior ->> 'partner_name') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        partner_website: {
            count(
                db,
                &format!("{} AND (exterior -> 'partner_website') != 'null'::jsonb AND (exterior ->> 'partner_website') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        partner_logo_url: {
            count(
                db,
                &format!("{} AND (exterior -> 'partner_logo_url') != 'null'::jsonb AND (exterior ->> 'partner_logo_url') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        partner_opp_url: {
            count(
                db,
                &format!("{} AND (exterior -> 'partner_opp_url') != 'null'::jsonb AND (exterior ->> 'partner_opp_url') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        organization_name: {
            count(
                db,
                &format!("{} AND (exterior -> 'organization_name') != 'null'::jsonb AND (exterior ->> 'organization_name') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        organization_type: {
            count(
                db,
                &format!("{} AND (exterior -> 'organization_type') != 'null'::jsonb AND (exterior ->> 'organization_type') != 'unspecified'", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        organization_website: {
            count(
                db,
                &format!("{} AND (exterior -> 'organization_website') != 'null'::jsonb AND (exterior ->> 'organization_website') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        organization_logo_url: {
            count(
                db,
                &format!("{} AND (exterior -> 'organization_logo_url') != 'null'::jsonb AND (exterior ->> 'organization_logo_url') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        opp_venue: {
            count(
                db,
                &format!("{} AND (exterior -> 'opp_venue') != 'null'::jsonb AND (exterior -> 'opp_venue') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        opp_descriptor: {
            count(
                db,
                &format!("{} AND (exterior -> 'opp_descriptor') != 'null'::jsonb AND (exterior -> 'opp_descriptor') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        min_age: {
            count(
                db,
                &format!("{} AND (exterior -> 'min_age') != 'null'::jsonb AND (exterior -> 'min_age') != '0'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        max_age: {
            count(
                db,
                &format!("{} AND (exterior -> 'max_age') != 'null'::jsonb AND (exterior -> 'max_age') != '999'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        pes_domain: {
            count(
                db,
                &format!("{} AND (exterior -> 'pes_domain') != 'null'::jsonb AND (exterior ->> 'pes_domain') != 'unspecified'", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        tags: {
            count(
                db,
                &format!(r#"{} AND (exterior -> 'tags') != 'null'::jsonb AND (exterior -> 'tags') != '[]'::jsonb AND (exterior -> 'tags') != '[""]'::jsonb"#, &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        opp_topics: {
            count(
                db,
                &format!("{} AND (exterior -> 'opp_topics') != 'null'::jsonb AND (exterior -> 'opp_topics') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        ticket_required: {
            count(
                db,
                &format!("{} AND (exterior -> 'ticket_required') != 'null'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        title: {
            count(
                db,
                &format!("{} AND (exterior -> 'title') != 'null'::jsonb AND (exterior ->> 'title') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        description: {
            count(
                db,
                &format!("{} AND (exterior -> 'description') != 'null'::jsonb AND (exterior ->> 'description') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        short_desc: {
            count(
                db,
                &format!("{} AND (exterior -> 'short_desc') != 'null'::jsonb AND (exterior ->> 'short_desc') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        image_url: {
            count(
                db,
                &format!("{} AND (exterior -> 'image_url') != 'null'::jsonb AND (exterior ->> 'image_url') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        image_credit: {
            count(
                db,
                &format!("{} AND (exterior -> 'image_credit') != 'null'::jsonb AND (exterior ->> 'image_credit') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        start_datetimes: {
            count(
                db,
                &format!("{} AND (exterior -> 'start_datetimes') != 'null'::jsonb AND (exterior -> 'start_datetimes') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        end_datetimes: {
            count(
                db,
                &format!("{} AND (exterior -> 'end_datetimes') != 'null'::jsonb AND (exterior -> 'end_datetimes') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        attraction_hours: {
            count(
                db,
                &format!("{} AND (exterior -> 'attraction_hours') != 'null'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        cost: {
            count(
                db,
                &format!("{} AND (exterior -> 'cost') != 'null'::jsonb AND (exterior ->> 'cost') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        languages: {
            count(
                db,
                &format!("{} AND (exterior -> 'languages') != 'null'::jsonb AND (exterior -> 'languages') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        opp_hashtags: {
            count(
                db,
                &format!("{} AND (exterior -> 'opp_hashtags') != 'null'::jsonb AND (exterior -> 'opp_hashtags') != '[]'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        opp_social_handles: {
            count(
                db,
                &format!("{} AND (exterior -> 'opp_social_handles') != 'null'::jsonb AND (exterior -> 'opp_social_handles') != '{{}}'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        is_online: {
            count(
                db,
                &format!("{} AND (exterior -> 'is_online') != 'null'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        location_type: {
            count(
                db,
                &format!("{} AND (exterior -> 'location_type') != 'null'::jsonb AND (exterior ->> 'location_type') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        location_point: {
            count(
                db,
                &format!("{} AND (exterior -> 'location_point') != 'null'::jsonb AND (exterior -> 'location_point') != '{{}}'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        location_polygon: {
            count(
                db,
                &format!("{} AND (exterior -> 'location_polygon') != 'null'::jsonb AND (exterior -> 'location_polygon') != '{{}}'::jsonb", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        address_street: {
            count(
                db,
                &format!("{} AND (exterior -> 'address_street') != 'null'::jsonb AND (exterior ->> 'address_street') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        address_city: {
            count(
                db,
                &format!("{} AND (exterior -> 'address_city') != 'null'::jsonb AND (exterior ->> 'address_city') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        address_state: {
            count(
                db,
                &format!("{} AND (exterior -> 'address_state') != 'null'::jsonb AND (exterior ->> 'address_state') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        address_country: {
            count(
                db,
                &format!("{} AND (exterior -> 'address_country') != 'null'::jsonb AND (exterior ->> 'address_country') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        address_zip: {
            count(
                db,
                &format!("{} AND (exterior -> 'address_zip') != 'null'::jsonb AND (exterior ->> 'address_zip') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        contact_name: {
            count(
                db,
                &format!("{} AND (interior -> 'contact_name') != 'null'::jsonb AND (interior ->> 'contact_name') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        contact_email: {
            count(
                db,
                &format!("{} AND (interior -> 'contact_email') != 'null'::jsonb AND (interior ->> 'contact_email') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
        contact_phone: {
            count(
                db,
                &format!("{} AND (interior -> 'contact_phone') != 'null'::jsonb AND (interior ->> 'contact_phone') != ''", &narrow),
                &params,
            )
            .await?
                / num_matches
        },
    }
    .into())
}
