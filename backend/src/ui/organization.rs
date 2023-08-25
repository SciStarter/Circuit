use std::collections::BTreeMap;

use common::{
    cached_json,
    geo::opp_regional_detailed_counts,
    model::{
        analytics::{RelativeTimePeriod, Status as AnayticsStatus},
        invitation::{Invitation, InvitationMode},
        opportunity::{EntityType, OpportunityQuery, OpportunityQueryOrdering},
        person::PersonPrivilegedReference,
        Opportunity, OpportunityExterior, Pagination, Partner, Person, SelectOption,
    },
    CachedJson, Database,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::{Status, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use super::{okay, okay_empty, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .post(add_organization)
        .at("analytics", |r| r.get(my_analytics))
        .at("opps-regional-overview", |r| r.get(opps_regional_overview))
        .at("opps-regional-detailed", |r| r.get(opps_regional_detailed))
        .at("opps-overview", |r| r.get(opps_overview))
        .at("all", |r| r.get(my_organizations))
        .at("exists", |r| r.post(check_organization))
        .at("types", |r| r.get(organization_types))
        .at(":uid", |r| {
            r.get(get_organization)
                .put(save_organization)
                .at("public", |r| r.get(get_organization_public))
                .at("invite", |r| r.post(invite_managers))
                .at("pending-managers", |r| {
                    r.get(get_pending_managers)
                        .post(add_pending_manager)
                        .delete(remove_pending_manager)
                })
                .at("managers", |r| {
                    r.get(get_managers).post(add_manager).delete(remove_manager)
                })
        })
}

pub async fn organization_types(_req: tide::Request<Database>) -> tide::Result {
    okay(
        &common::model::opportunity::OrganizationType::all_options()
            .into_iter()
            .map(|(x, y, _)| (x, y))
            .collect::<Vec<_>>(),
    )
}

#[derive(Serialize, Deserialize, Debug)]
struct AddOrganizationForm {
    partner: String,
    access: String,
    name: String,
    email: String,
    phone: String,
    address: String,
    city: String,
    state: String,
    about: String,
    number: u32,
    website: String,
}

pub async fn add_organization(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?.ok_or_else(|| {
        tide::Error::from_str(tide::StatusCode::Forbidden, "Authorization required")
    })?;

    let form: AddOrganizationForm = req.body_json().await?;

    common::emails::send(
        "Science Near Me <info@sciencenearme.org>",
        "Science Near Me <info@sciencenearme.org>",
        format!("SNM Partner Request for {}", form.partner),
        format!(
            r#"<p><strong>{}</strong> has requested a partner account for <em>{}</em></p>

<h1># Details</h1>

<p>
<strong>Request from:</strong> {}<br>
<strong>Partner name:</strong> {}<br>
<strong>Access mode:</strong> {}<br>
<strong>Contact name:</strong> {}<br>
<strong>Contact email:</strong> {}<br>
<strong>Contact phone:</strong> {}<br>
<strong>Address:</strong> {}<br>
<strong>City:</strong> {}<br>
<strong>State:</strong> {}<br>
<strong>Website:</strong> {}<br>
<strong>Annual opportunities:</strong> {}
</p>

<h1># About</h1>

{}
"#,
            &person.interior.email,
            &form.partner,
            &person.interior.email,
            &form.partner,
            &form.access,
            &form.name,
            &form.phone,
            &form.email,
            &form.address,
            &form.city,
            &form.state,
            &form.website,
            &form.number,
            &form.about
        ),
    )
    .await;

    common::log(
        Some(&person.exterior.uid),
        "ui-add-organization-request",
        &json!({"subject": form.partner}),
    );

    okay_empty()
}

#[derive(serde::Deserialize, Debug)]
struct CheckOrganizationQuery {
    name: Option<String>,
    uid: Option<Uuid>,
}

pub async fn check_organization(mut req: tide::Request<Database>) -> tide::Result {
    let query: CheckOrganizationQuery = req.body_json().await?;

    okay(&if let Some(name) = query.name {
        common::model::Partner::find_by_name(req.state(), &name)
            .await
            .map_err(|err| {
                let mut error = tide::Error::from_debug(err);
                error.set_status(tide::StatusCode::BadRequest);
                error
            })?
    } else if let Some(uid) = query.uid {
        vec![common::model::Partner::load_by_uid(req.state(), &uid)
            .await
            .map_err(|err| {
                let mut error = tide::Error::from_debug(err);
                error.set_status(tide::StatusCode::BadRequest);
                error
            })?
            .into()]
    } else {
        Vec::new()
    })
}

pub async fn my_organizations(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req).await?.ok_or_else(|| {
        tide::Error::from_str(tide::StatusCode::Forbidden, "Authorization required")
    })?;

    let partners: Vec<common::model::partner::Partner> = person
        .load_partners(req.state())
        .await?
        .into_iter()
        .flatten()
        .map(|p| p.elide())
        .collect();

    okay(&partners)
}

async fn authorized_partner(
    req: &mut tide::Request<Database>,
) -> Result<(Person, Partner), tide::Error> {
    let person = request_person(req).await?.ok_or_else(|| {
        tide::Error::from_str(tide::StatusCode::Forbidden, "Authorization required")
    })?;

    let uid = Uuid::parse_str(req.param("uid")?).with_status(|| StatusCode::BadRequest)?;

    let partner = Partner::load_by_uid(req.state(), &uid)
        .await
        .with_status(|| StatusCode::BadRequest)?;

    if !partner.person_has_permission(&person.exterior.uid) {
        return Err(tide::Error::from_str(StatusCode::Forbidden, "Forbidden"));
    }

    Ok((person, partner))
}

pub async fn get_organization(mut req: tide::Request<Database>) -> tide::Result {
    let (_person, partner) = authorized_partner(&mut req).await?;

    okay(&partner.elide())
}

pub async fn get_organization_public(req: tide::Request<Database>) -> tide::Result {
    let uid = Uuid::parse_str(req.param("uid")?).with_status(|| StatusCode::BadRequest)?;
    let partner = Partner::load_by_uid(req.state(), &uid)
        .await
        .with_status(|| StatusCode::BadRequest)?;
    okay(&partner.exterior)
}

pub async fn save_organization(mut req: tide::Request<Database>) -> tide::Result {
    let (person, partner) = authorized_partner(&mut req).await?;

    let mut incoming: Partner = req
        .body_json()
        .await
        .map_err(|err| tide::Error::from_debug(err))?;

    incoming.id = partner.id;
    incoming.exterior.uid = partner.exterior.uid;
    incoming.interior.secret = partner.interior.secret;
    incoming.interior.prime = partner.interior.prime;

    incoming.interior.authorized = incoming
        .interior
        .authorized
        .into_iter()
        .filter(|x| x != &partner.interior.prime)
        .collect();

    incoming.store(req.state()).await?;

    common::log(
        Some(&person.exterior.uid),
        "ui-save-organization",
        &json!({"partner": partner.exterior.uid}),
    );

    okay_empty()
}

#[derive(Deserialize)]
struct InviteForm {
    emails: Vec<String>,
}

pub async fn invite_managers(mut req: tide::Request<Database>) -> tide::Result {
    let (person, partner) = authorized_partner(&mut req).await?;

    let form: InviteForm = req.body_json().await?;

    if let Some(msg) = common::emails::EmailMessage::load(req.state(), "invite-to-organization")
        .await
        .ok()
    {
        for email in form.emails.iter() {
            let mut inv = Invitation::new(partner.exterior.uid, InvitationMode::JoinOrganization);
            inv.store(req.state()).await?;
            let outgoing = msg.materialize(vec![
                ("invitation", inv.uid().to_string()),
                ("partner_name", partner.exterior.name.clone()),
            ]);
            common::emails::send_message(email, &outgoing).await;
        }
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Email template not found",
        ));
    }

    common::log(
        Some(&person.exterior.uid),
        "ui-invite-organization-managers",
        &json!({"partner": partner.exterior.uid, "emails": form.emails}),
    );

    okay_empty()
}

pub async fn get_pending_managers(mut req: tide::Request<Database>) -> tide::Result {
    let (_person, partner) = authorized_partner(&mut req).await?;

    okay(
        &partner
            .load_pending_persons(req.state())
            .await
            .with_status(|| StatusCode::BadRequest)?
            .iter()
            .flatten()
            .map(|person| person.into())
            .collect::<Vec<PersonPrivilegedReference>>(),
    )
}

pub async fn add_pending_manager(mut req: tide::Request<Database>) -> tide::Result {
    let (person, _partner) = authorized_partner(&mut req).await?;
    common::log(Some(&person.exterior.uid), "ui-add-pending-manager", "");
    todo!()
}

pub async fn remove_pending_manager(mut req: tide::Request<Database>) -> tide::Result {
    let (person, _partner) = authorized_partner(&mut req).await?;
    common::log(Some(&person.exterior.uid), "ui-remove-pending-manager", "");
    todo!()
}

pub async fn get_managers(mut req: tide::Request<Database>) -> tide::Result {
    let (_person, partner) = authorized_partner(&mut req).await?;

    okay(
        &partner
            .load_authorized_persons(req.state())
            .await
            .with_status(|| StatusCode::BadRequest)?
            .iter()
            .flatten()
            .map(|person| person.into())
            .collect::<Vec<PersonPrivilegedReference>>(),
    )
}

pub async fn add_manager(mut req: tide::Request<Database>) -> tide::Result {
    let (person, _partner) = authorized_partner(&mut req).await?;
    common::log(Some(&person.exterior.uid), "ui-add-manager", "");
    todo!()
}

pub async fn remove_manager(mut req: tide::Request<Database>) -> tide::Result {
    let (person, _partner) = authorized_partner(&mut req).await?;
    common::log(Some(&person.exterior.uid), "ui-remove-manager", "");
    todo!()
}

#[derive(serde::Deserialize, Debug)]
struct AnalyticsRequest {
    about: Uuid,
    kind: i32,
    period: RelativeTimePeriod,
    status: AnayticsStatus,
    field: Option<String>,
}

pub async fn my_analytics(mut req: tide::Request<Database>) -> tide::Result {
    if let Ok(params) = req.query::<AnalyticsRequest>() {
        if params.about != Uuid::nil() {
            let _person = request_person(&mut req).await?.ok_or_else(|| {
                tide::Error::from_str(tide::StatusCode::Forbidden, "Authorization required")
            })?;
        }

        let data: serde_json::Value = sqlx::query!(
            r#"
SELECT
  "data" as "data!"
FROM
  c_analytics_compiled
WHERE
  "about" = $1 AND
  "kind" = $2 AND
  "period" = $3 AND
  "status" = $4
"#,
            params.about,
            params.kind,
            params.period.discriminate(),
            params.status.discriminate(),
        )
        .map(|row| row.data)
        .fetch_optional(req.state())
        .await?
        .unwrap_or_default();

        if let Some(field) = params.field {
            okay(&data[field]["data"])
        } else {
            okay(&data)
        }
    } else {
        let person = request_person(&mut req).await?.ok_or_else(|| {
            tide::Error::from_str(tide::StatusCode::Forbidden, "Authorization required")
        })?;

        let mut partners = person
            .load_partners(req.state())
            .await?
            .into_iter()
            .flatten();

        let mut toplevel: BTreeMap<String, serde_json::Value> = BTreeMap::new();

        let Some(first) = partners.next() else {
            return okay(&toplevel);
        };

        toplevel.insert(
            "initial".into(),
            serde_json::Value::String(first.exterior.name.clone()),
        );

        toplevel.insert(
            first.exterior.name,
            serde_json::to_value(
                sqlx::query!(
                    r#"
SELECT
  "data" as "data!"
FROM
  c_analytics_compiled
WHERE
  "about" = $1 AND
  "kind" = $2 AND
  "period" = $3 AND
  "status" = $4
"#,
                    first.exterior.uid,
                    0,
                    0,
                    0,
                )
                .map(|row| row.data)
                .fetch_one(req.state())
                .await?,
            )?,
        );

        for part in partners {
            toplevel.insert(
                part.exterior.name,
                json!({"organization": part.exterior.uid}),
            );
        }

        okay(&toplevel)
    }
}

#[derive(Serialize)]
struct LabeledAllCurrent {
    label: String,
    all: i64,
    current: i64,
}

#[derive(Serialize)]
struct LabeledValue {
    label: String,
    value: i64,
}

pub async fn opps_overview(req: tide::Request<Database>) -> tide::Result {
    let total = sqlx::query_scalar!(r#"SELECT COUNT(*) AS "result!" FROM c_opportunity"#)
        .fetch_one(req.state())
        .await?;

    let active = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "result!" FROM c_opportunity WHERE c_opportunity_is_current(interior, exterior)"#
    )
    .fetch_one(req.state())
    .await?;

    let online = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "result!" FROM c_opportunity WHERE exterior->>'is_online' = 'true'"#
    )
    .fetch_one(req.state())
    .await?;

    let regional = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "result!" FROM c_opportunity WHERE exterior->>'location_type' = 'near'"#
    )
    .fetch_one(req.state())
    .await?;

    let at_point = sqlx::query_scalar!(
        r#"SELECT COUNT(*) AS "result!" FROM c_opportunity WHERE exterior->>'location_type' = 'at'"#
    )
    .fetch_one(req.state())
    .await?;

    let domain_all = sqlx::query!(
        r#"
SELECT exterior->>'pes_domain' AS "domain!", COUNT(*) AS "total!"
FROM c_opportunity
WHERE exterior->>'pes_domain' != 'unspecified'
GROUP BY exterior->>'pes_domain'
ORDER BY "total!" DESC
"#
    )
    .map(|row| (row.domain.to_owned(), row.total))
    .fetch_all(req.state())
    .await?;
    let domain_all_max = domain_all.iter().map(|(_, t)| *t).max();

    let domain_current = sqlx::query!(
        r#"
SELECT exterior->>'pes_domain' AS "domain!", COUNT(*) AS "total!"
FROM c_opportunity
WHERE c_opportunity_is_current(interior, exterior) AND exterior->>'pes_domain' != 'unspecified'
GROUP BY exterior->>'pes_domain'
ORDER BY "total!" DESC
"#
    )
    .map(|row| (row.domain.to_owned(), row.total))
    .fetch_all(req.state())
    .await?;
    let domain_current_max = domain_current.iter().map(|(_, t)| *t).max();

    let domain_current: BTreeMap<String, i64> = domain_current.into_iter().collect();

    let activity_all = sqlx::query!(
        r#"
SELECT v.descriptor AS "descriptor!", count(*) AS "total!"
FROM c_opportunity o JOIN jsonb_array_elements_text(exterior->'opp_descriptor') v(descriptor) ON true
GROUP BY v.descriptor
ORDER BY "total!" DESC
"#
    )
    .map(|row| (row.descriptor.to_owned(), row.total))
    .fetch_all(req.state())
    .await?;
    let activity_all_max = activity_all.iter().map(|(_, t)| *t).max();

    let activity_current = sqlx::query!(
        r#"
SELECT v.descriptor AS "descriptor!", count(*) AS "total!"
FROM c_opportunity o JOIN jsonb_array_elements_text(exterior->'opp_descriptor') v(descriptor) ON true
WHERE c_opportunity_is_current(interior, exterior)
GROUP BY v.descriptor;
"#
    )
    .map(|row| (row.descriptor.to_owned(), row.total))
    .fetch_all(req.state())
    .await?;
    let activity_current_max = activity_current.iter().map(|(_, t)| *t).max();

    let activity_current: BTreeMap<String, i64> = activity_current.into_iter().collect();

    let indoor_all = sqlx::query_scalar!(r#"SELECT count(*) AS "total!" FROM c_opportunity WHERE exterior->'opp_venue' @> '"indoors"'::jsonb"#).fetch_one(req.state()).await?;
    let outdoor_all = sqlx::query_scalar!(r#"SELECT count(*) AS "total!" FROM c_opportunity WHERE exterior->'opp_venue' @> '"outdoors"'::jsonb"#).fetch_one(req.state()).await?;
    let indoor_current = sqlx::query_scalar!(r#"SELECT count(*) AS "total!" FROM c_opportunity WHERE c_opportunity_is_current(interior, exterior) AND exterior->'opp_venue' @> '"indoors"'::jsonb"#).fetch_one(req.state()).await?;
    let outdoor_current = sqlx::query_scalar!(r#"SELECT count(*) AS "total!" FROM c_opportunity WHERE c_opportunity_is_current(interior, exterior) AND exterior->'opp_venue' @> '"outdoors"'::jsonb"#).fetch_one(req.state()).await?;

    let keywords = sqlx::query!(r#"SELECT lower(trim(v.tag)) AS "text!", count(*) as "total!" FROM c_opportunity o JOIN jsonb_array_elements_text(exterior->'tags') v(tag) ON true GROUP BY lower(trim(v.tag)) ORDER BY "total!" DESC LIMIT 30"#).map(|row| (row.text.to_owned(), row.total)).fetch_all(req.state()).await?;

    let providers = sqlx::query!(r#"SELECT p.exterior->>'name' AS "name!", count(*) AS "total!" FROM c_opportunity o JOIN c_partner p ON p.exterior->'uid' = o.exterior->'partner' GROUP BY p.exterior->>'name' ORDER BY "total!" DESC"#)
        .map(|row| LabeledValue {
            label: row.name.to_owned(),
            value: row.total,
        })
        .fetch_all(req.state())
        .await?;

    let providers_max = providers.iter().map(|lv| lv.value).max();

    okay(&json!({
        "total": total,

        "active": active,
        "inactive": total - active,

        "online": online,
        "in_person": total - online,

        "global": total - regional - at_point,
        "regional": regional,
        "at_point": at_point,

        "attribute": {
            "domain": {"max": {"all": domain_all_max, "current": domain_current_max}, "rows": domain_all.into_iter().map(|(label, all)| LabeledAllCurrent {current: *domain_current.get(&label).unwrap_or(&0), label, all}).collect::<Vec<_>>()},
            "activity": {"max": {"all": activity_all_max, "current": activity_current_max}, "rows": activity_all.into_iter().map(|(label, all)| LabeledAllCurrent {current: *activity_current.get(&label).unwrap_or(&0), label, all}).collect::<Vec<_>>()},
            "indoor": {"max": {"all": indoor_all.max(outdoor_all), "current": indoor_current.max(outdoor_current)}, "rows": [{"label": "Indoors", "all": indoor_all, "current": indoor_current}, {"label": "Outdoors", "all": outdoor_all, "current": outdoor_current}]},
        },

        "keywords": keywords,

        "providers": {
            "max": providers_max,
            "rows": providers,
        }
    }))
}

pub async fn opps_regional_overview(req: tide::Request<Database>) -> tide::Result {
    match cached_json(req.state(), "geoexp-regional-overview", 7).await? {
        CachedJson::Current(json) => okay(&json),
        CachedJson::Expired(json) => {
            async_std::task::spawn(common::geo::opps_regional_overview_calc(
                req.state().clone(),
            ));
            okay(&json)
        }
        CachedJson::Missing => {
            async_std::task::spawn(common::geo::opps_regional_overview_calc(
                req.state().clone(),
            ));
            okay(
                &json!({"anywhere": 0, "states": {}, "counts": {"max": 0, }, "status": "processing..."}),
            )
        }
    }
}

#[derive(Deserialize)]
struct RegionalDetailedQuery {
    name: String,
}

pub async fn opps_regional_detailed(req: tide::Request<Database>) -> tide::Result {
    let params: RegionalDetailedQuery = req.query()?;

    let mut query = OpportunityQuery::default();

    query.entity_type = Some(vec![
        EntityType::Opportunity,
        EntityType::Attraction,
        EntityType::Unspecified,
    ]);

    query.accepted = Some(true);
    query.withdrawn = Some(false);
    query.current = Some(true);
    query.region = Some(params.name.clone());

    let matches: Vec<OpportunityExterior> = Opportunity::load_matching(
        req.state(),
        &query,
        OpportunityQueryOrdering::Any,
        Pagination::All,
    )
    .await?
    .into_iter()
    .map(|m| m.exterior)
    .collect();

    okay(&json!({
        "data": matches,
        "counts": opp_regional_detailed_counts(req.state().clone(), Some(params.name)).await?,
    }))
}
