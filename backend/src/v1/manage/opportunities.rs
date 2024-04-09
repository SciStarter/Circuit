use super::IntoResponse;
use chrono::{DateTime, FixedOffset};
use common::{
    model::{
        opportunity::{
            Cost, Descriptor, Domain, EntityType, LocationType, OpenDays, OpenHours,
            OpportunityQuery, OpportunityQueryOrdering, OrganizationType, PageLayout, PageOptions,
            Topic, VenueType,
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
use serde_json::{json, Value};
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
            "page__add_opportunities" => EntityType::Page(PageOptions {
                layout: PageLayout::AddOpportunities,
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

        opportunity.exterior.attraction_hours = if self.monday_opens.is_some()
            || self.tuesday_opens.is_some()
            || self.wednesday_opens.is_some()
            || self.thursday_opens.is_some()
            || self.friday_opens.is_some()
            || self.saturday_opens.is_some()
            || self.sunday_opens.is_some()
        {
            Some(OpenDays {
                monday: if let (Some(opens), Some(closes)) = (self.monday_opens, self.monday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
                tuesday: if let (Some(opens), Some(closes)) =
                    (self.tuesday_opens, self.tuesday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
                wednesday: if let (Some(opens), Some(closes)) =
                    (self.wednesday_opens, self.wednesday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
                thursday: if let (Some(opens), Some(closes)) =
                    (self.thursday_opens, self.thursday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
                friday: if let (Some(opens), Some(closes)) = (self.friday_opens, self.friday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
                saturday: if let (Some(opens), Some(closes)) =
                    (self.saturday_opens, self.saturday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
                sunday: if let (Some(opens), Some(closes)) = (self.sunday_opens, self.sunday_closes)
                {
                    Some(OpenHours { opens, closes })
                } else {
                    None
                },
            })
        } else {
            None
        };

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
        let form: OpportunityForm = qs.deserialize_str(&req.body_string().await?)?;

        form.apply(&mut opportunity)?;

        let db = req.state();

        if let Err(err) = opportunity.store(db).await {
            return Ok(OpportunityPage {
                message: dbg!(err).to_string(),
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

    let mut opportunity = Opportunity::default();

    form.apply(&mut opportunity)?;

    opportunity.store(db).await?;

    Ok(redirect(&format!(
        "{}{}",
        req.url().path(),
        opportunity.exterior.uid
    )))
}

#[derive(TemplateOnce)]
#[template(path = "manage/overlay.stpl.html")]
struct OverlayPage {
    message: String,
    opportunity: Opportunity,
    exterior: Value,
    _interior: Value,
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

impl OverlayForm {
    pub fn apply(self, exterior: &mut Value, _interior: &mut Value) {
        fn delete(obj: &mut Value, key: &str) {
            obj.as_object_mut().map(|obj| obj.remove(key));
        }

        if let Some(domain) = self.pes_domain {
            exterior["pes_domain"] = domain.to_option().0.into();
        } else {
            delete(exterior, "pes_domain");
        }

        if !self.opp_descriptor.is_empty() {
            exterior["opp_descriptor"] = self
                .opp_descriptor
                .into_iter()
                .map(|x| x.to_option().0)
                .collect::<Vec<_>>()
                .into();
        } else {
            delete(exterior, "opp_descriptor");
        }

        if !self.opp_topics.is_empty() {
            exterior["opp_topics"] = self
                .opp_topics
                .into_iter()
                .map(|x| x.to_option().0)
                .collect::<Vec<_>>()
                .into();
        } else {
            delete(exterior, "opp_topics");
        }

        if !self.tags.is_empty() {
            exterior["tags"] = self.tags.clone().into();
        } else {
            delete(exterior, "tags");
        }

        if let Some(min_age) = self.min_age {
            if min_age != -1 {
                exterior["min_age"] = min_age.into();
            } else {
                delete(exterior, "min_age");
            }
        } else {
            delete(exterior, "min_age");
        }

        if let Some(max_age) = self.max_age {
            if max_age != -1 {
                exterior["max_age"] = max_age.into();
            } else {
                delete(exterior, "max_age");
            }
        } else {
            delete(exterior, "max_age");
        }

        if let Some(cost) = self.cost {
            exterior["cost"] = cost.to_option().0.into();
        } else {
            delete(exterior, "cost");
        }

        if let Some(online) = self.is_online {
            exterior["is_online"] = online.into();
        } else {
            delete(exterior, "is_online");
        }

        if let Some(loc) = self.location_type {
            exterior["location_type"] = loc.to_option().0.into();
        } else {
            delete(exterior, "location_type");
        }

        if let Some(location_name) = self.location_name {
            exterior["location_name"] = location_name.into();
        } else {
            delete(exterior, "location_name");
        }

        if let Some(address_street) = self.address_street {
            exterior["address_street"] = address_street.into();
        } else {
            delete(exterior, "address_street");
        }

        if let Some(address_city) = self.address_city {
            exterior["address_city"] = address_city.into();
        } else {
            delete(exterior, "address_city");
        }

        if let Some(address_state) = self.address_state {
            exterior["address_state"] = address_state.into();
        } else {
            delete(exterior, "address_state");
        }

        if let Some(address_country) = self.address_country {
            exterior["address_country"] = address_country.into();
        } else {
            delete(exterior, "address_country");
        }

        if let Some(address_zip) = self.address_zip {
            exterior["address_zip"] = address_zip.into();
        } else {
            delete(exterior, "address_zip");
        }
    }
}

async fn overlay(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageOpportunities).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let opportunity = {
        let uid: Uuid = req.param("uid")?.parse()?;
        Opportunity::load_by_uid(req.state(), &uid).await?
    };

    let (mut exterior, mut interior) = {
        sqlx::query!(r#"SELECT exterior as "exterior!", interior as "interior!" FROM c_opportunity_overlay WHERE opportunity_id = $1"#, opportunity.id)
            .fetch_optional(req.state())
            .await?.map(|r| (r.exterior, r.interior)).unwrap_or_else(|| (json!({}), json!({})))
    };

    if let Method::Post = req.method() {
        let qs = serde_qs::Config::new(5, false);
        let form: OverlayForm = qs.deserialize_str(&req.body_string().await?)?;

        form.apply(&mut exterior, &mut interior);

        if let Err(err) = sqlx::query!(
            r#"
INSERT INTO
  c_opportunity_overlay (opportunity_id, exterior, interior)
VALUES
  ($1, $2, $3)
ON CONFLICT
  (opportunity_id)
DO UPDATE SET
  exterior = EXCLUDED.exterior,
  interior = EXCLUDED.interior
WHERE
  c_opportunity_overlay.opportunity_id = $1
"#,
            opportunity.id,
            exterior,
            interior
        )
        .execute(req.state())
        .await
        {
            return Ok(OverlayPage {
                message: err.to_string(),
                opportunity,
                exterior,
                _interior: interior,
            }
            .into_response(StatusCode::Ok)?);
        }

        return Ok(redirect(req.url().path()));
    }

    let form = OverlayPage {
        message: String::new(),
        opportunity,
        exterior,
        _interior: interior,
    };

    Ok(form.into_response(StatusCode::Ok)?)
}
