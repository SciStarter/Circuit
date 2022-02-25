use async_std::stream::StreamExt;
use chrono::{FixedOffset, Utc};
use common::{
    model::{
        involvement::{Involvement, Mode},
        person::{Gender, Goal, GoalStatus},
        Opportunity, Pagination, Person,
    },
    Database, ToFixedOffset,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use tide::{Status, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use crate::ui::okay_empty;

use super::{okay, request_person};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .get(get_profile)
        .put(save_profile)
        .delete(delete_profile)
        .at("saved", |r| {
            r.post(add_saved)
                .at("old", |r| r.delete(delete_old_saved))
                .at(":uid", |r| r.delete(delete_saved))
        })
        .at("involved", |r| r.get(get_involved).post(set_involvement))
        .at("partners", |r| r.get(get_partners))
        .at("goals", |r| {
            r.get(get_goals)
                .post(add_goal)
                .at(":id", |r| r.put(save_goal).delete(cancel_goal))
        })
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct ProfilePerson {
    username: Option<String>,
    image_url: Option<String>,
    email: String,
    password: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    genders: Vec<Gender>,
    gender_other: Option<String>,
    phone: Option<String>,
    whatsapp: Option<String>,
    zip_code: Option<String>,
    birth_year: Option<u32>,
    ethnicities: Vec<common::model::person::Ethnicity>,
    ethnicity_other: Option<String>,
    family_income: Option<common::model::person::IncomeLevel>,
    education_level: Option<common::model::person::EducationLevel>,
    opt_in_research: Option<bool>,
    opt_in_volunteer: Option<bool>,
    private: bool,
}

impl ProfilePerson {
    fn update_person(self, person: &mut Person) {
        person.exterior.username = self.username;
        person.exterior.image_url = self.image_url;
        person.interior.email = self.email;
        person.interior.first_name = self.first_name;
        person.interior.last_name = self.last_name;
        person.interior.genders = self.genders;
        person.interior.gender_other = self.gender_other;
        person.interior.phone = self.phone;
        person.interior.whatsapp = self.whatsapp;
        person.interior.zip_code = self.zip_code;
        person.interior.birth_year = self.birth_year;
        person.interior.ethnicities = self.ethnicities;
        person.interior.ethnicity_other = self.ethnicity_other;
        person.interior.family_income = self.family_income;
        person.interior.education_level = self.education_level;
        person.interior.opt_in_research = self.opt_in_research;
        person.interior.opt_in_volunteer = self.opt_in_volunteer;
        person.interior.private = self.private;

        if let Some(password) = self.password {
            person.set_password(&password);
        }
    }
}

impl From<Person> for ProfilePerson {
    fn from(person: Person) -> Self {
        ProfilePerson {
            username: person.exterior.username,
            image_url: person.exterior.image_url,
            email: person.interior.email,
            password: None,
            first_name: person.interior.first_name,
            last_name: person.interior.last_name,
            genders: person.interior.genders,
            gender_other: person.interior.gender_other,
            phone: person.interior.phone,
            whatsapp: person.interior.whatsapp,
            zip_code: person.interior.zip_code,
            birth_year: person.interior.birth_year,
            ethnicities: person.interior.ethnicities,
            ethnicity_other: person.interior.ethnicity_other,
            family_income: person.interior.family_income,
            education_level: person.interior.education_level,
            opt_in_research: person.interior.opt_in_research,
            opt_in_volunteer: person.interior.opt_in_volunteer,
            private: person.interior.private,
        }
    }
}

pub async fn get_profile(mut req: tide::Request<Database>) -> tide::Result {
    let person: ProfilePerson = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?
        .into();

    okay(&person)
}

pub async fn save_profile(mut req: tide::Request<Database>) -> tide::Result {
    let mut person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let prof: ProfilePerson = req.body_json().await?;

    prof.update_person(&mut person);

    person.store(req.state()).await?;

    common::log("ui-save-profile", &json!({"person": person.exterior.uid}));

    okay_empty()
}

pub async fn delete_profile(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    if let Some(id) = person.id {
        sqlx::query("delete from c_person where id = $1")
            .bind(id)
            .execute(req.state())
            .await?;
        common::log("ui-delete-profile", &json!({"person": person.exterior.uid}));
    }

    okay_empty()
}

pub async fn delete_old_saved(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    // Consider moving the meat of this function into a SQL query

    let mut stream = Involvement::all_for_participant(
        req.state(),
        &person.exterior.uid,
        Some(Mode::Saved),
        Some(Mode::Saved),
        None,
        Pagination::All,
    )
    .await?;

    let now = chrono::Utc::now().to_fixed_offset();

    while let Some(result) = stream.next().await {
        if let Ok(mut inv) = result {
            let opp = Opportunity::load_by_uid(req.state(), &inv.exterior.opportunity).await?;

            if opp.expired_as_of(&now) {
                inv.exterior.mode = Mode::Deleted;
                inv.store(req.state()).await?;
            }
        }
    }
    common::log("ui-delete-old-saved", &person.exterior.uid);

    Ok(tide::Response::builder(StatusCode::NoContent).build())
}

pub async fn delete_saved(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let target: Uuid = req.param("uid")?.parse()?;

    let mut inv = Involvement::load_by_participant_and_opportunity(
        req.state(),
        &person.exterior.uid,
        &target,
    )
    .await?
    .ok_or_else(|| tide::Error::from_str(StatusCode::NotFound, "No such saved opportunity"))?;

    inv.exterior.mode = Mode::Deleted;

    inv.store(req.state()).await?;

    common::log(
        "ui-delete-saved",
        &json!({"person": person.exterior.uid, "opportunity": target}),
    );

    Ok(tide::Response::builder(StatusCode::NoContent).build())
}

pub async fn add_saved(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let target: Uuid = req.param("uid")?.parse()?;

    Involvement::upgrade(
        req.state(),
        &person.exterior.uid,
        &target,
        Mode::Saved,
        &None,
    )
    .await?;

    common::log(
        "ui-add-saved",
        &json!({"person": person.exterior.uid, "opportunity": target}),
    );

    Ok(tide::Response::builder(StatusCode::NoContent).build())
}

#[derive(Deserialize)]
struct InvolvedQuery {
    page: Option<u32>,
    min: Option<Mode>,
    max: Option<Mode>,
    opp: Option<bool>,
    text: Option<String>,
}

pub async fn get_involved(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let query: InvolvedQuery = req.query()?;

    let mut involved = Vec::with_capacity(10);

    let pagination = Pagination::Page {
        index: query.page.unwrap_or(0),
        size: 10,
    };

    let mut stream = Involvement::all_for_participant(
        req.state(),
        &person.exterior.uid,
        query.min,
        query.max,
        query.text,
        pagination,
    )
    .await?;

    while let Some(result) = stream.next().await {
        if let Ok(inv) = result {
            if query.opp.unwrap_or(false) {
                if let Ok(opp) =
                    Opportunity::load_by_uid(req.state(), &inv.exterior.opportunity).await
                {
                    let mut obj = serde_json::to_value(inv)?;
                    obj["opportunity"] = serde_json::to_value(opp.exterior)?;
                    involved.push(obj);
                }
            } else {
                involved.push(serde_json::to_value(inv)?);
            };
        }
    }

    let total =
        Involvement::count_for_participant(req.state(), &person.exterior.uid, query.min, query.max)
            .await?;

    let (page_index, last_page, per_page) = pagination.expand(total);

    okay(&json!({
        "pagination": {
            "page_index": page_index,
            "per_page": per_page,
            "last_page": last_page,
            "total": total,
        },
        "matches": involved
    }))
}

#[derive(Deserialize)]
struct InvolvementTarget {
    id: i32,
    mode: Mode,
}

pub async fn set_involvement(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let target: InvolvementTarget = req.body_json().await?;

    let mut inv = Involvement::load_by_id(req.state(), target.id).await?;

    if inv.interior.participant != person.exterior.uid {
        return Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "not authorized",
        ));
    }

    // We're not using Involvement::upgrade here because the user can
    // manually downgrade their involvement. So, we need to also set
    // the latest change datetime.
    inv.exterior.mode = target.mode.min(Mode::Logged);
    inv.exterior.latest = Utc::now().to_fixed_offset();

    inv.store(req.state()).await?;

    common::log(
        "ui-set-involvement",
        &json!({"person": person.exterior.uid, "opportunity": inv.exterior.opportunity, "mode": target.mode}),
    );

    okay_empty()
}

pub async fn get_partners(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let partners: Vec<common::model::partner::PartnerExterior> = person
        .load_partners(req.state())
        .await?
        .into_iter()
        .flatten()
        .map(|p| p.exterior)
        .collect();

    okay(&partners)
}

pub async fn get_goals(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let mut stream = person
        .goals_by_status(req.state(), GoalStatus::Working)
        .await?;

    let mut goals = Vec::with_capacity(3);

    while let Some(result) = stream.next().await {
        let goal = result?.into_fixed_offset();

        let progress = {
            let mut progress = Vec::new();

            let mut stream = person
                .all_participation_between(req.state(), &goal.begin, &goal.end)
                .await?;

            while let Some(result) = stream.next().await {
                progress.push(result?.expand(req.state()).await?);
            }

            progress
        };

        let mut goal = serde_json::to_value(goal)?;
        goal["progress"] = serde_json::to_value(progress)?;
        goals.push(goal);
    }

    okay(&goals)
}

pub async fn add_goal(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let goal: Goal<FixedOffset> = req.body_json().await?;

    let goal_id = person.add_goal(req.state(), goal).await?;

    common::log(
        "ui-add-goal",
        &json!({"person": person.exterior.uid, "goal": goal_id}),
    );

    okay(&goal_id)
}

pub async fn save_goal(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let mut goal: Goal<FixedOffset> = req.body_json().await?;
    let goal_id = req.param("id")?.parse()?;
    goal.id = goal_id;

    person
        .save_goal(req.state(), goal)
        .await
        .with_status(|| StatusCode::BadRequest)?;

    common::log(
        "ui-save-goal",
        &json!({"person": person.exterior.uid, "goal": goal_id}),
    );

    okay_empty()
}

pub async fn cancel_goal(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(StatusCode::Forbidden, "Authorization required"))?;

    let goal_id = req.param("id")?.parse()?;

    let mut goal = person
        .goal_by_id(req.state(), goal_id)
        .await
        .with_status(|| StatusCode::NotFound)?
        .into_fixed_offset();

    goal.status = GoalStatus::Canceled;

    person.save_goal(req.state(), goal).await?;

    common::log(
        "ui-cancel-goal",
        &json!({"person": person.exterior.uid, "goal": goal_id}),
    );

    okay_empty()
}
