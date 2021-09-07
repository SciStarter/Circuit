use askama::Template;
use common::{
    model::{
        opportunity::{
            EntityType, OpportunityQuery, OpportunityQueryOrdering, PageLayout, PageOptions,
        },
        partner::PartnerReference,
        person::Permission,
        Opportunity, Pagination, Partner, SelectOption,
    },
    Database,
};
use http_types::{mime, Method};
use serde::Deserialize;
use tide::{Response, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::v1::redirect;

use super::BASE;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(search)
        .post(add_opportunity)
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

async fn search(req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let form: OpportunitiesForm = req.query()?;
    let db = req.state();

    Ok(OpportunitiesPage {
        accepted: form.accepted.unwrap_or(false),
        withdrawn: form.withdrawn.unwrap_or(false),
        title: form.title.clone().unwrap_or_else(String::new),
        partner: form.partner.clone().unwrap_or_default(),
        partners: Partner::catalog(db).await?,
        matches: Opportunity::load_matching(
            db,
            &OpportunityQuery {
                title_contains: form.title,
                partner: form.partner,
                accepted: form.accepted,
                withdrawn: form.withdrawn,
                ..Default::default()
            },
            OpportunityQueryOrdering::Alphabetical,
            Pagination::All,
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
    slug: String,
    entity_type: String,
    partner_name: String,
    partner_opp_url: String,
    tags: String,
    accepted: Option<String>,
    withdrawn: Option<String>,
}

impl OpportunityForm {
    fn apply(self, opportunity: &mut Opportunity) -> Result<(), tide::Error> {
        opportunity.interior.accepted = Some(self.accepted.is_some());
        opportunity.interior.withdrawn = self.withdrawn.is_some();

        opportunity.exterior.entity_type = match self.entity_type.as_ref() {
            "attraction" => EntityType::Attraction,
            "opportunity" => EntityType::Opportunity,
            "page__just_content" => EntityType::Page(PageOptions {
                layout: PageLayout::JustContent,
            }),
            _ => EntityType::Opportunity,
        };

        opportunity.exterior.title = self.title;
        opportunity.exterior.slug = self.slug;
        opportunity.exterior.partner_name = self.partner_name;
        opportunity.exterior.partner_opp_url = self.partner_opp_url;
        opportunity.exterior.tags = self.tags.split(',').map(|s| s.trim().to_string()).collect();

        // !!! TODO this is incomplete

        Ok(())
    }
}

async fn opportunity(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let mut opportunity = {
        let uid: Uuid = req.param("uid")?.parse()?;
        let db = req.state();
        Opportunity::load_by_uid(db, &uid).await?
    };

    if let Method::Post = req.method() {
        let form: OpportunityForm = req.body_form().await?;

        form.apply(&mut opportunity)?;

        let db = req.state();

        if let Err(err) = opportunity.store(db).await {
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

async fn add_opportunity(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let form: OpportunityForm = req.body_form().await?;

    let db = req.state();

    if Opportunity::exists_by_slug(db, &form.slug).await? {
        return Ok(redirect(req.url().path()));
    }

    let mut opportunity = Opportunity::default();

    form.apply(&mut opportunity)?;

    opportunity.store(db).await?;

    Ok(redirect(&format!(
        "{}{}",
        req.url().path(),
        opportunity.exterior.uid
    )))
}
