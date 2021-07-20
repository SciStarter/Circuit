use askama::Template;
use common::model::person::Permission;
use http_types::Method;
use serde::Deserialize;
use sqlx::{Acquire, Postgres};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use tide_sqlx::SQLxRequestExt;
use uuid::Uuid;

use crate::v1::redirect;
use common::model::opportunity::OpportunityQuery;
use common::model::partner::PartnerReference;
use common::model::Opportunity;
use common::model::Partner;
use common::model::SelectOption;

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes
        .get(search)
        .at(":uid", |r| r.get(opportunity).post(opportunity))
}

#[derive(Template)]
#[template(path = "manage/opportunities.html")]
struct OpportunitiesPage {
    pub accepted: bool,
    pub withdrawn: bool,
    pub title: String,
    pub partner: Uuid,
    pub partners: Vec<PartnerReference>,
    pub matches: Vec<Opportunity>,
}

#[derive(Deserialize)]
struct OpportunitiesForm {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub title: Option<String>,
    pub partner: Option<Uuid>,
}

async fn search(req: tide::Request<()>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let form: OpportunitiesForm = req.query()?;
    let mut db = req.sqlx_conn::<Postgres>().await;

    Ok(OpportunitiesPage {
        accepted: form.accepted.unwrap_or(false),
        withdrawn: form.withdrawn.unwrap_or(false),
        title: form.title.clone().unwrap_or_else(String::new),
        partner: form.partner.clone().unwrap_or_default(),
        partners: Partner::catalog(db.acquire().await?).await?,
        matches: Opportunity::load_matching(
            db.acquire().await?,
            OpportunityQuery {
                title_contains: form.title,
                partner: form.partner,
                accepted: form.accepted,
                withdrawn: form.withdrawn,
                ..Default::default()
            },
        )
        .await?,
    }
    .into())
}

mod filters {
    pub fn mapping(
        mapping: &std::collections::HashMap<String, String>,
    ) -> ::askama::Result<String> {
        if mapping.is_empty() {
            return Ok("<em>No data entered</em>".to_string());
        }

        let parts: Vec<String> = mapping
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        Ok(parts.join(", "))
    }
}

#[derive(Template)]
#[template(path = "manage/opportunity.html")]
struct OpportunityPage {
    message: String,
    opportunity: Opportunity,
}

#[derive(Deserialize)]
struct OpportunityForm {
    title: String,
    partner_name: String,
}

async fn opportunity(mut req: tide::Request<()>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let mut opportunity = {
        let uid: Uuid = req.param("uid")?.parse()?;
        let mut db = req.sqlx_conn::<Postgres>().await;
        Opportunity::load_by_uid(db.acquire().await?, &uid).await?
    };

    if let Method::Post = req.method() {
        let form: OpportunityForm = req.body_form().await?;

        opportunity.exterior.title = form.title.clone();
        opportunity.exterior.partner_name = form.partner_name.clone();

        // TODO this is incomplete

        let mut db = req.sqlx_conn::<Postgres>().await;

        if let Err(err) = opportunity.store(db.acquire().await?).await {
            return Ok(OpportunityPage {
                message: err.to_string(),
                opportunity,
            }
            .into());
        }

        return Ok(redirect(req.url().path()));
    }

    let form = OpportunityPage {
        message: String::new(),
        opportunity,
    };

    Ok(form.into())
}
