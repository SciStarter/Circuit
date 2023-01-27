use common::{model::person::MiscPermission, Database};
use once_cell::sync::Lazy;
use serde::Deserialize;
use tide::{Response, StatusCode};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use uuid::Uuid;

use super::{okay, okay_empty, request_person};

static EVOLVEME_ENDPOINT: Lazy<String> = Lazy::new(|| {
    format!(
        "{}/user/tasks/steps",
        std::env::var("EVOLVEME_HOST")
            .unwrap_or_else(|_| "https://asa-apps-evolveme-api-dev.alliants.net/".into())
    )
});

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
    routes
        .at("extra", |r| {
            r.at(":key", |r| {
                r.get(get_extra).put(set_extra).delete(del_extra)
            })
        })
        .at("evolveme", |r| r.post(evolveme))
}

pub async fn set_extra(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(mut person) = request_person(&mut req).await? {
        let key = req.param("key")?.to_owned();
        let body = req.take_body();

        if key.len() > 64 || body.len() > Some(1024) {
            return Ok(StatusCode::PayloadTooLarge.into());
        }

        let value: serde_json::Value = body.into_json().await?;

        person.set_extra(&key, value, MiscPermission::ClientReadWrite);

        person.store(req.state()).await?;
    }

    okay_empty()
}

pub async fn get_extra(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(person) = request_person(&mut req).await? {
        let key = req.param("key")?;

        if let Some(val) = person.get_extra_json(key, MiscPermission::ClientReadOnly) {
            okay(&val)
        } else {
            okay_empty()
        }
    } else {
        okay_empty()
    }
}

pub async fn del_extra(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(mut person) = request_person(&mut req).await? {
        let key = req.param("key")?;

        person.del_extra(key, MiscPermission::ClientReadWrite);
        person.store(req.state()).await?;
    }

    okay_empty()
}

#[derive(serde::Deserialize)]
struct EvolveMeForm {
    step: u8,
    user_id: u32,
    unique_task_key: Uuid,
}

pub async fn evolveme(mut req: tide::Request<Database>) -> tide::Result {
    if let Some(mut person) = request_person(&mut req).await? {
        let EvolveMeForm {
            step,
            user_id,
            unique_task_key,
        } = req.body_json().await?;

        let current = person
            .get_extra("evolveme-step", MiscPermission::Server)
            .unwrap_or(0);

        if step > current {
            println!(
                "EvolveMe response: {:?}",
                surf::post(&*EVOLVEME_ENDPOINT)
                    .header("X-EM-Signature", "")
                    .body_json(&serde_json::json!({
                        "stepNumber": step,
                        "uniqueTaskKey": unique_task_key,
                        "userid": user_id
                    }))?
                    .recv_string()
                    .await
            );

            person.set_extra("evolveme-step", current, MiscPermission::ClientReadOnly);
            person.store(req.state()).await?;
        }
    }

    okay_empty()
}
