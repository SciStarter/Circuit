use super::IntoResponse;
use chrono::{DateTime, FixedOffset};
use common::{
    model::{
        opportunity::{
            Cost, Descriptor, Domain, EntityType, LocationType, OpportunityAll, OpportunityQuery,
            OpportunityQueryOrdering, OrganizationType, Topic, VenueType,
        },
        partner::PartnerReference,
        person::Permission,
        Opportunity, Pagination, Partner, SelectOption,
    },
    Database, INTERNAL_UID,
};
use http_types::{Method, StatusCode};
use sailfish::TemplateOnce;
use serde::Deserialize;

use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::v1::redirect;

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes.get(search).post(add_opportunity).at(":uid", |r| {
        r.at("overlay", |r| r.get(overlay).post(overlay))
            .get(opportunity)
            .post(opportunity)
    })
}

#[derive(TemplateOnce)]
#[template(path = "manage/opportunities.stpl.html")]
struct OpportunitiesPage {
    pub accepted: Option<bool>,
    pub withdrawn: Option<bool>,
    pub current: Option<bool>,
    pub title: String,
    pub partner: Uuid,
    pub partners: Vec<PartnerReference>,
    pub matches: Vec<OpportunityAll>,
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

    let matches = OpportunityAll::load_matching(
        db,
        OpportunityQuery {
            title_contains: form.title.clone(),
            partner: form.partner.clone(),
            accepted: form.accepted,
            withdrawn: form.withdrawn,
            current: Some(form.current.unwrap_or(true)),
            ..Default::default()
        },
        OpportunityQueryOrdering::PartnerName,
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
    .into_response(StatusCode::Ok)?)
}

mod filters {
    pub fn _mapping(mapping: &std::collections::HashMap<String, String>) -> String {
        if mapping.is_empty() {
            return "<em>No data entered</em>".to_string();
        }

        let parts: Vec<String> = mapping
            .iter()
            .map(|(k, v)| format!("{}: {}", k, v))
            .collect();
        parts.join(", ")
    }
}

#[derive(TemplateOnce)]
#[template(path = "manage/opportunity.stpl.html")]
struct OpportunityPage {
    message: String,
    opportunity: OpportunityAll,
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
    partner_opp_url: Option<String>,
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
    monday_opens: Option<String>,
    monday_closes: Option<String>,
    tuesday_opens: Option<String>,
    tuesday_closes: Option<String>,
    wednesday_opens: Option<String>,
    wednesday_closes: Option<String>,
    thursday_opens: Option<String>,
    thursday_closes: Option<String>,
    friday_opens: Option<String>,
    friday_closes: Option<String>,
    saturday_opens: Option<String>,
    saturday_closes: Option<String>,
    sunday_opens: Option<String>,
    sunday_closes: Option<String>,
}

impl OpportunityForm {
    fn apply(self, opportunity: &mut OpportunityAll) -> Result<(), tide::Error> {
        opportunity.interior.accepted = Some(self.accepted.is_some());
        opportunity.interior.withdrawn = self.withdrawn.is_some();

        opportunity.exterior.opp.partner = self.partner.unwrap_or_else(|| INTERNAL_UID.clone());

        opportunity.exterior.opp.entity_type = match self.entity_type.as_ref() {
            "attraction" => EntityType::Attraction,
            "opportunity" => EntityType::Opportunity,
            "page__just_content" => EntityType::PageJustContent,
            "page__add_opportunities" => EntityType::PageAddOpportunities,
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

        opportunity.exterior.opp.title = self.title;
        opportunity.exterior.opp.slug = self.slug;
        opportunity.exterior.opp.partner_name = self.partner_name;
        opportunity.exterior.opp.partner_opp_url = self.partner_opp_url;
        opportunity.exterior.opp.partner_website = self.partner_website;
        opportunity.exterior.opp.partner_logo_url = self.partner_logo_url;
        opportunity.exterior.opp.organization_name = self.organization_name.unwrap_or_default();
        opportunity.exterior.opp.organization_type = self.organization_type.unwrap_or_default();
        opportunity.exterior.opp.organization_website = self.organization_website;
        opportunity.exterior.opp.organization_logo_url = self.organization_logo_url;
        opportunity.exterior.opp_venue = self.opp_venue.unwrap_or_default();
        opportunity.exterior.opp_descriptor = self.opp_descriptor.unwrap_or_default();
        opportunity.exterior.opp.min_age = self.min_age.unwrap_or(0);
        opportunity.exterior.opp.max_age = self.max_age.unwrap_or(999);
        opportunity.exterior.opp.pes_domain = self.pes_domain.unwrap_or_default();
        opportunity.exterior.tags = self.tags.split(',').map(|s| s.trim().to_string()).collect();
        opportunity.exterior.opp_topics = self.opp_topics.unwrap_or_default();
        opportunity.exterior.opp.ticket_required = self.ticket_required.unwrap_or(false);
        opportunity.exterior.opp.description = self.description.unwrap_or_default();
        opportunity.exterior.opp.short_desc = self.short_desc.unwrap_or_default();
        opportunity.exterior.opp.image_url = self.image_url.unwrap_or_default();
        opportunity.exterior.opp.image_credit = self.image_credit.unwrap_or_default();
        opportunity.exterior.opp.cost = self.cost.unwrap_or_default();
        opportunity.exterior.opp.is_online = self.is_online.unwrap_or(false);
        opportunity.exterior.opp.location_type = self.location_type.unwrap_or_default();
        opportunity.exterior.opp.address_street = self.address_street.unwrap_or_default();
        opportunity.exterior.opp.address_city = self.address_city.unwrap_or_default();
        opportunity.exterior.opp.address_state = self.address_state.unwrap_or_default();
        opportunity.exterior.opp.address_country = self.address_country.unwrap_or_default();
        opportunity.exterior.opp.address_zip = self.address_zip.unwrap_or_default();

        opportunity.interior.contact_name = self.contact_name.unwrap_or_default();
        opportunity.interior.contact_email = self.contact_email.unwrap_or_default();
        opportunity.interior.contact_phone = self.contact_phone.unwrap_or_default();

        // opportunity.exterior.opp.attraction_hours = if self.monday_opens.is_some()
        //     || self.tuesday_opens.is_some()
        //     || self.wednesday_opens.is_some()
        //     || self.thursday_opens.is_some()
        //     || self.friday_opens.is_some()
        //     || self.saturday_opens.is_some()
        //     || self.sunday_opens.is_some()
        // {
        //     Some(OpenDays {
        //         monday: if let (Some(opens), Some(closes)) = (self.monday_opens, self.monday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //         tuesday: if let (Some(opens), Some(closes)) =
        //             (self.tuesday_opens, self.tuesday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //         wednesday: if let (Some(opens), Some(closes)) =
        //             (self.wednesday_opens, self.wednesday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //         thursday: if let (Some(opens), Some(closes)) =
        //             (self.thursday_opens, self.thursday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //         friday: if let (Some(opens), Some(closes)) = (self.friday_opens, self.friday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //         saturday: if let (Some(opens), Some(closes)) =
        //             (self.saturday_opens, self.saturday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //         sunday: if let (Some(opens), Some(closes)) = (self.sunday_opens, self.sunday_closes)
        //         {
        //             Some(OpenHours { opens, closes })
        //         } else {
        //             None
        //         },
        //     })
        // } else {
        //     None
        // };

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
        OpportunityAll::load_by_uid(db, uid).await?
    };

    if let Method::Post = req.method() {
        let qs = serde_qs::Config::new(5, false);
        let form: OpportunityForm = qs.deserialize_str(&req.body_string().await?)?;

        form.apply(&mut opportunity)?;

        let db = req.state();

        if let Err(err) = opportunity.store(db).await {
            return Ok(OpportunityPage {
                message: err.to_string(),
                all_partners: Partner::catalog(db).await?,
                opportunity,
            }
            .into_response(StatusCode::Ok)?);
        }

        return Ok(redirect(req.url().path()));
    }

    let db = req.state();

    let form = OpportunityPage {
        message: String::new(),
        all_partners: Partner::catalog(db).await?,
        opportunity,
    };

    Ok(form.into_response(StatusCode::Ok)?)
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

    let mut opportunity = OpportunityAll::default();

    form.apply(&mut opportunity)?;

    opportunity.store(db).await?;

    Ok(redirect(&format!(
        "{}{}",
        req.url().path(),
        opportunity.exterior.opp.uid
    )))
}

#[derive(TemplateOnce)]
#[template(path = "manage/overlay.stpl.html")]
struct OverlayPage {
    message: String,
    opportunity: Opportunity,
    form: OverlayForm,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
struct OverlayForm {
    pes_domain: Option<common::model::opportunity::Domain>,
    opp_descriptor: Vec<common::model::opportunity::Descriptor>,
    opp_topics: Vec<common::model::opportunity::Topic>,
    tags: Vec<String>,
    min_age: Option<i16>,
    max_age: Option<i16>,
    cost: Option<common::model::opportunity::Cost>,
    is_online: Option<bool>,
    location_type: Option<common::model::opportunity::LocationType>,
    location_name: Option<String>,
    address_street: Option<String>,
    address_city: Option<String>,
    address_state: Option<String>,
    address_country: Option<String>,
    address_zip: Option<String>,
}

async fn overlay(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let opportunity = {
        let uid: Uuid = req.param("uid")?.parse()?;
        Opportunity::load_by_uid(req.state(), uid).await?
    };

    if let Method::Post = req.method() {
        let qs = serde_qs::Config::new(5, false);
        let form: OverlayForm = qs.deserialize_str(&req.body_string().await?)?;
        let db = req.state();

        // array(SELECT "descriptor" FROM c_opportunity_descriptor WHERE opportunity_id = $1 AND overlay = true) AS "opp_descriptor!: _",
        // array(SELECT "topic" FROM c_opportunity_topic WHERE opportunity_id = $1 AND overlay = true) AS "opp_topics!: _",
        // array(SELECT "tag" FROM c_opportunity_tag WHERE opportunity_id = $1 AND overlay = true) AS "tags!: _",

        for item in form.opp_descriptor.iter() {
            sqlx::query!(
                r#"
                INSERT INTO c_opportunity_descriptor ("opportunity_id", "overlay", "descriptor")
                VALUES ($1, true, $2)
                ON CONFLICT DO NOTHING
                "#,
                opportunity.id,
                item as &Descriptor,
            )
            .execute(db)
            .await?;
        }

        for item in form.opp_topics.iter() {
            sqlx::query!(
                r#"
            INSERT INTO c_opportunity_topic ("opportunity_id", "overlay", "topic")
            VALUES ($1, true, $2)
            ON CONFLICT DO NOTHING
            "#,
                opportunity.id,
                item as &Topic,
            )
            .execute(db)
            .await?;
        }

        for item in form.tags.iter() {
            sqlx::query!(
                r#"
            INSERT INTO c_opportunity_tag ("opportunity_id", "overlay", "tag")
            VALUES ($1, true, $2)
            ON CONFLICT DO NOTHING
            "#,
                opportunity.id,
                item,
            )
            .execute(db)
            .await?;
        }

        if let Err(err) = sqlx::query!(
            r#"
            INSERT INTO c_opportunity_overlay (
              opportunity_id,
              "pes_domain",
              "min_age",
              "max_age",
              "cost",
              "is_online",
              "location_type",
              "location_name",
              "address_street",
              "address_city",
              "address_state",
              "address_country",
              "address_zip"
            )
            VALUES (
              $1,
              $2,
              $3,
              $4,
              $5,
              $6,
              $7,
              $8,
              $9,
              $10,
              $11,
              $12,
              $13
            )
            ON CONFLICT (opportunity_id) DO UPDATE
            SET
              "pes_domain" = EXCLUDED."pes_domain",
              "min_age" = EXCLUDED."min_age",
              "max_age" = EXCLUDED."max_age",
              "cost" = EXCLUDED."cost",
              "is_online" = EXCLUDED."is_online",
              "location_type" = EXCLUDED."location_type",
              "location_name" = EXCLUDED."location_name",
              "address_street" = EXCLUDED."address_street",
              "address_city" = EXCLUDED."address_city",
              "address_state" = EXCLUDED."address_state",
              "address_country" = EXCLUDED."address_country",
              "address_zip" = EXCLUDED."address_zip"
            WHERE
              c_opportunity_overlay.opportunity_id = $1
            "#,
            opportunity.id,
            form.pes_domain as Option<Domain>,
            form.min_age,
            form.max_age,
            form.cost as Option<Cost>,
            form.is_online,
            form.location_type as Option<LocationType>,
            form.location_name,
            form.address_street,
            form.address_city,
            form.address_state,
            form.address_country,
            form.address_zip,
        )
        .execute(db)
        .await
        {
            return Ok(OverlayPage {
                message: err.to_string(),
                opportunity,
                form,
            }
            .into_response(StatusCode::Ok)?);
        }

        return Ok(redirect(req.url().path()));
    }

    let form = {
        sqlx::query_as!(
            OverlayForm,
            r#"
            SELECT
              "pes_domain" AS "pes_domain: _",
              array(SELECT "descriptor" FROM c_opportunity_descriptor WHERE opportunity_id = $1 AND overlay = true) AS "opp_descriptor!: _",
              array(SELECT "topic" FROM c_opportunity_topic WHERE opportunity_id = $1 AND overlay = true) AS "opp_topics!: _",
              array(SELECT "tag" FROM c_opportunity_tag WHERE opportunity_id = $1 AND overlay = true) AS "tags!: _",
              "min_age" AS "min_age: _",
              "max_age" AS "max_age: _",
              "cost" AS "cost: _",
              "is_online" AS "is_online: _",
              "location_type" AS "location_type: _",
              "location_name" AS "location_name: _",
              "address_street" AS "address_street: _",
              "address_city" AS "address_city: _",
              "address_state" AS "address_state: _",
              "address_country" AS "address_country: _",
              "address_zip" AS "address_zip: _"
            FROM c_opportunity_overlay
            WHERE opportunity_id = $1
            "#,
            opportunity.id
        )
        .fetch_optional(req.state())
        .await?
        .unwrap_or_else(|| OverlayForm::default())
    };

    let page = OverlayPage {
        message: String::new(),
        opportunity,
        form,
    };

    Ok(page.into_response(StatusCode::Ok)?)
}
