use async_std::stream::StreamExt;
use common::{
    model::{
        involvement::{self, Involvement, Mode},
        Opportunity, Pagination,
    },
    Database, ToFixedOffset,
};
use serde::Deserialize;
use serde_json::json;
use tide::StatusCode;
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
}

pub async fn get_profile(_req: tide::Request<Database>) -> tide::Result {
    todo!()
}

pub async fn save_profile(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-save-profile", "");
    todo!()
}

pub async fn delete_profile(_req: tide::Request<Database>) -> tide::Result {
    common::log("ui-delete-profile", "");
    todo!()
}

pub async fn delete_old_saved(mut req: tide::Request<Database>) -> tide::Result {
    let person = request_person(&mut req)
        .await?
        .ok_or_else(|| tide::Error::from_str(401, "Authorization required"))?;

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

            if opp.ended_as_of(&now) {
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
        .ok_or_else(|| tide::Error::from_str(401, "Authorization required"))?;

    let target: Uuid = req.param("uid")?.parse()?;

    let mut inv = Involvement::load_by_participant_and_opportunity(
        req.state(),
        &person.exterior.uid,
        &target,
    )
    .await?
    .ok_or_else(|| tide::Error::from_str(404, "No such saved opportunity"))?;

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
        .ok_or_else(|| tide::Error::from_str(401, "Authorization required"))?;

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
        .ok_or_else(|| tide::Error::from_str(401, "Authorization required"))?;

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
            let obj = if query.opp.unwrap_or(false) {
                let opp = Opportunity::load_by_uid(req.state(), &inv.exterior.opportunity).await?;
                let mut obj = serde_json::to_value(inv)?;
                obj["opportunity"] = serde_json::to_value(opp.exterior)?;
                obj
            } else {
                serde_json::to_value(inv)?
            };

            involved.push(obj);
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
        .ok_or_else(|| tide::Error::from_str(401, "Authorization required"))?;

    let target: InvolvementTarget = req.body_json().await?;

    let mut inv = Involvement::load_by_id(req.state(), target.id).await?;

    if inv.interior.participant != person.exterior.uid {
        return Err(tide::Error::from_str(
            StatusCode::Forbidden,
            "not authorized",
        ));
    }

    inv.exterior.mode = target.mode;

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
        .ok_or_else(|| tide::Error::from_str(401, "Authorization required"))?;

    let partners: Vec<common::model::partner::PartnerExterior> = person
        .load_partners(req.state())
        .await?
        .into_iter()
        .flatten()
        .map(|p| p.exterior)
        .collect();

    okay(&partners)
}
