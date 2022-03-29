use common::{
    model::{
        invitation::{Invitation, InvitationMode},
        person::PersonPrivilegedReference,
        Partner, Person, SelectOption,
    },
    Database,
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
        .at("all", |r| r.get(my_organizations))
        .at("exists", |r| r.post(check_organization))
        .at("types", |r| r.get(organization_types))
        .at(":uid", |r| {
            r.get(get_organization)
                .put(save_organization)
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
        "ui-add-organization-request",
        &json!({"person": person.exterior.uid, "subject": form.partner}),
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
        "ui-save-organization",
        &json!({"person": person.exterior.uid, "partner": partner.exterior.uid}),
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
            let outgoing = msg.materialize(vec![("invitation", inv.uid())]);
            common::emails::send_message(email, &outgoing).await;
        }
    } else {
        return Err(tide::Error::from_str(
            StatusCode::InternalServerError,
            "Email template not found",
        ));
    }

    common::log(
        "ui-invite-organization-managers",
        &json!({"person": person.exterior.uid, "partner": partner.exterior.uid, "emails": form.emails}),
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

pub async fn add_pending_manager(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-add-pending-manager", "");
    todo!()
}

pub async fn remove_pending_manager(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-remove-pending-manager", "");
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

pub async fn add_manager(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-add-manager", "");
    todo!()
}

pub async fn remove_manager(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-remove-manager", "");
    todo!()
}
