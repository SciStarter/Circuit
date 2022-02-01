use askama::Template;
use chrono::{DateTime, FixedOffset};
use common::{
    model::{
        opportunity::{
            Cost, Descriptor, Domain, EntityType, LocationType, OpportunityQuery,
            OpportunityQueryOrdering, OrganizationType, PageLayout, PageOptions, Topic, VenueType,
        },
        partner::PartnerReference,
        person::Permission,
        Opportunity, Pagination, Partner, SelectOption,
    },
    Database, INTERNAL_UID,
};
use http_types::Method;
use serde::Deserialize;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::v1::redirect;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(search)
        .post(add_opportunity)
        .at(":uid", |r| r.get(opportunity).post(opportunity))
}

#[derive(Template)]
#[template(path = "manage/opportunities.html")]
struct OpportunitiesPage {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub current: Option<bool>,
    pub title: String,
    pub partner: Uuid,
    pub partners: Vec<PartnerReference>,
    pub matches: Vec<Opportunity>,
    pub num_matches: usize,
}

#[derive(Deserialize)]
struct OpportunitiesForm {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub title: Option<String>,
    pub partner: Option<Uuid>,
    pub current: Option<bool>,
}

async fn search(req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let form: OpportunitiesForm = req.query()?;
    let db = req.state();

    let matches = Opportunity::load_matching(
        db,
        &OpportunityQuery {
            title_contains: form.title.clone(),
            partner: form.partner.clone(),
            accepted: form.accepted,
            withdrawn: form.withdrawn,
            current: Some(form.current.unwrap_or(true)),
            ..Default::default()
        },
        OpportunityQueryOrdering::Alphabetical,
        Pagination::All,
    )
    .await?;

    let num_matches = matches.len();

    Ok(OpportunitiesPage {
        accepted: form.accepted,
        withdrawn: form.withdrawn,
        current: form.current,
        title: form.title.unwrap_or_default(),
        partner: form.partner.unwrap_or_default(),
        partners: Partner::catalog(db).await?,
        matches,
        num_matches,
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
    all_partners: Vec<PartnerReference>,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
struct OpportunityForm {
    title: String,
    slug: String,
    entity_type: String,
    partner: Option<Uuid>,
    partner_name: String,
    partner_website: Option<String>,
    partner_logo_url: Option<String>,
    partner_opp_url: String,
    organization_name: Option<String>,
    organization_type: Option<OrganizationType>,
    organization_website: Option<String>,
    organization_logo_url: Option<String>,
    opp_venue: Option<Vec<VenueType>>,
    opp_descriptor: Option<Vec<Descriptor>>,
    min_age: Option<i16>,
    max_age: Option<i16>,
    pes_domain: Option<Domain>,
    tags: String,
    opp_topics: Option<Vec<Topic>>,
    ticket_required: Option<bool>,
    description: Option<String>,
    short_desc: Option<String>,
    image_url: Option<String>,
    image_credit: Option<String>,
    start_datetimes: Option<Vec<Option<DateTime<FixedOffset>>>>,
    end_datetimes: Option<Vec<Option<DateTime<FixedOffset>>>>,
    cost: Option<Cost>,
    languages: Option<String>,
    opp_hashtags: Option<String>,
    is_online: Option<bool>,
    location_type: Option<LocationType>,
    address_street: Option<String>,
    address_city: Option<String>,
    address_state: Option<String>,
    address_country: Option<String>,
    address_zip: Option<String>,
    contact_name: Option<String>,
    contact_email: Option<String>,
    contact_phone: Option<String>,
    accepted: Option<String>,
    withdrawn: Option<String>,
}

impl OpportunityForm {
    fn apply(self, opportunity: &mut Opportunity) -> Result<(), tide::Error> {
        opportunity.interior.accepted = Some(self.accepted.is_some());
        opportunity.interior.withdrawn = self.withdrawn.is_some();

        opportunity.exterior.partner = self.partner.unwrap_or_else(|| INTERNAL_UID.clone());

        opportunity.exterior.entity_type = match self.entity_type.as_ref() {
            "attraction" => EntityType::Attraction,
            "opportunity" => EntityType::Opportunity,
            "page__just_content" => EntityType::Page(PageOptions {
                layout: PageLayout::JustContent,
            }),
            _ => EntityType::Opportunity,
        };

        opportunity.exterior.languages = self
            .languages
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        opportunity.exterior.opp_hashtags = self
            .opp_hashtags
            .unwrap_or_default()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        opportunity.exterior.start_datetimes = self
            .start_datetimes
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .collect();

        opportunity.exterior.end_datetimes = self
            .end_datetimes
            .unwrap_or_default()
            .into_iter()
            .flatten()
            .collect();

        opportunity.exterior.title = self.title;
        opportunity.exterior.slug = self.slug;
        opportunity.exterior.partner_name = self.partner_name;
        opportunity.exterior.partner_opp_url = self.partner_opp_url;
        opportunity.exterior.partner_website = self.partner_website;
        opportunity.exterior.partner_logo_url = self.partner_logo_url;
        opportunity.exterior.organization_name = self.organization_name.unwrap_or_default();
        opportunity.exterior.organization_type = self.organization_type.unwrap_or_default();
        opportunity.exterior.organization_website = self.organization_website;
        opportunity.exterior.organization_logo_url = self.organization_logo_url;
        opportunity.exterior.opp_venue = self.opp_venue.unwrap_or_default();
        opportunity.exterior.opp_descriptor = self.opp_descriptor.unwrap_or_default();
        opportunity.exterior.min_age = self.min_age.unwrap_or(0);
        opportunity.exterior.max_age = self.max_age.unwrap_or(999);
        opportunity.exterior.pes_domain = self.pes_domain.unwrap_or_default();
        opportunity.exterior.tags = self.tags.split(',').map(|s| s.trim().to_string()).collect();
        opportunity.exterior.opp_topics = self.opp_topics.unwrap_or_default();
        opportunity.exterior.ticket_required = self.ticket_required.unwrap_or(false);
        opportunity.exterior.description = self.description.unwrap_or_default();
        opportunity.exterior.short_desc = self.short_desc.unwrap_or_default();
        opportunity.exterior.image_url = self.image_url.unwrap_or_default();
        opportunity.exterior.image_credit = self.image_credit.unwrap_or_default();
        opportunity.exterior.cost = self.cost.unwrap_or_default();
        opportunity.exterior.is_online = self.is_online.unwrap_or(false);
        opportunity.exterior.location_type = self.location_type.unwrap_or_default();
        opportunity.exterior.address_street = self.address_street.unwrap_or_default();
        opportunity.exterior.address_city = self.address_city.unwrap_or_default();
        opportunity.exterior.address_state = self.address_state.unwrap_or_default();
        opportunity.exterior.address_country = self.address_country.unwrap_or_default();
        opportunity.exterior.address_zip = self.address_zip.unwrap_or_default();

        opportunity.interior.contact_name = self.contact_name.unwrap_or_default();
        opportunity.interior.contact_email = self.contact_email.unwrap_or_default();
        opportunity.interior.contact_phone = self.contact_phone.unwrap_or_default();

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
        let qs = serde_qs::Config::new(5, false);
        let form: OpportunityForm = dbg!(qs.deserialize_str(&dbg!(req.body_string().await?))?);

        form.apply(&mut opportunity)?;

        let db = req.state();

        if let Err(err) = opportunity.store(db).await {
            return Ok(OpportunityPage {
                message: err.to_string(),
                all_partners: Partner::catalog(db).await?,
                opportunity,
            }
            .into());
        }

        return Ok(redirect(req.url().path()));
    }

    let db = req.state();

    let form = OpportunityPage {
        message: String::new(),
        all_partners: Partner::catalog(db).await?,
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
