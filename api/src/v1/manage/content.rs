use askama::Template;
use common::model::{block::Block, person::Permission};
use http_types::Method;
use serde::Deserialize;
use sqlx::{Acquire, Postgres};
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};
use tide_sqlx::SQLxRequestExt;

use crate::v1::redirect;

use super::authorized_admin;

pub fn routes(routes: RouteSegment<()>) -> RouteSegment<()> {
    routes.get(content).post(content).at(":language/", |r| {
        r.get(content_language)
            .post(content_language)
            .at(":group/", |r| {
                r.get(content_group)
                    .post(content_group)
                    .at(":item", |r| r.get(content_item).post(content_item))
            })
    })
}

#[derive(Template)]
#[template(path = "manage/content.html")]
struct ContentPage {
    languages: Vec<(String, String)>,
    all_languages: Vec<(String, String)>,
}

#[derive(Deserialize)]
struct ContentForm {
    language: String,
}

async fn content(mut req: tide::Request<()>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: ContentForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}/", req.url().path(), form.language)));
    }

    let mut db = req.sqlx_conn::<Postgres>().await;

    let page = ContentPage {
        languages: Block::list_languages(db.acquire().await?)
            .await?
            .iter()
            .map(|code| {
                (
                    code.to_owned(),
                    common::LANGUAGES.get(code).unwrap_or(code).to_owned(),
                )
            })
            .collect(),
        all_languages: common::LANGUAGES
            .iter()
            .map(|(c, n)| (c.to_owned(), n.to_owned()))
            .collect(),
    };

    Ok(page.into())
}

#[derive(Template, Default)]
#[template(path = "manage/content_language.html")]
struct ContentLanguagePage {
    language_name: String,
    groups: Vec<String>,
}

#[derive(Deserialize)]
struct ContentLanguageForm {
    group: String,
}

async fn content_language(mut req: tide::Request<()>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: ContentLanguageForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}/", req.url().path(), form.group)));
    }

    let mut db = req.sqlx_conn::<Postgres>().await;

    let language = req.param("language")?.to_owned();
    let language_name = common::LANGUAGES
        .get(&language)
        .unwrap_or(&language)
        .to_owned();
    let groups = Block::list_groups(db.acquire().await?, &language).await?;

    Ok(ContentLanguagePage {
        language_name,
        groups,
    }
    .into())
}

#[derive(Template, Default)]
#[template(path = "manage/content_group.html")]
struct ContentGroupPage {
    language_name: String,
    group: String,
    items: Vec<String>,
}

#[derive(Deserialize)]
struct ContentGroupForm {
    item: String,
}

async fn content_group(mut req: tide::Request<()>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: ContentGroupForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}", req.url().path(), form.item)));
    }

    let mut db = req.sqlx_conn::<Postgres>().await;

    let language = req.param("language")?.to_owned();
    let language_name = common::LANGUAGES
        .get(&language)
        .unwrap_or(&language)
        .to_owned();
    let group = req.param("group")?.to_owned();
    let items = Block::list_items(db.acquire().await?, &language, &group).await?;

    Ok(ContentGroupPage {
        language_name,
        group,
        items,
    }
    .into())
}

#[derive(Template, Default)]
#[template(path = "manage/content_item.html")]
struct ContentItemPage {
    language_name: String,
    group: String,
    item: String,
    tags: String,
    label: String,
    content: String,
}

#[derive(Deserialize)]
struct ContentItemForm {
    tags: String,
    label: String,
    content: String,
}

async fn content_item(mut req: tide::Request<()>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    let language = req.param("language")?.to_owned();
    let language_name = common::LANGUAGES
        .get(&language)
        .unwrap_or(&language)
        .to_owned();
    let group = req.param("group")?.to_owned();
    let item = req.param("item")?.to_owned();

    let mut block = {
        let mut db = req.sqlx_conn::<Postgres>().await;
        match Block::load(db.acquire().await?, &language, &group, &item).await {
            Ok(block) => block,
            Err(_) => Block {
                id: None,
                language: language.to_string(),
                group: group.to_string(),
                item: item.to_string(),
                tags: "".to_string(),
                label: "".to_string(),
                content: "".to_string(),
            },
        }
    };

    if let Method::Post = req.method() {
        let form: ContentItemForm = req.body_form().await?;
        let mut db = req.sqlx_conn::<Postgres>().await;
        block.tags = form.tags.trim().to_string();
        block.label = form.label.trim().to_string();
        block.content = form.content.trim().to_string();
        block.store(db.acquire().await?).await?;
        return Ok(redirect(""));
    } else {
        Ok(ContentItemPage {
            language_name,
            group,
            item,
            tags: block.tags,
            label: block.label,
            content: block.content,
        }
        .into())
    }
}