use common::model::{block::Block, person::Permission};
use common::Database;
use http_types::{Method, StatusCode};
use sailfish::TemplateOnce;
use serde::Deserialize;
use tide_fluent_routes::{
    routebuilder::{RouteBuilder, RouteBuilderExt},
    RouteSegment,
};

use crate::v1::redirect;

use super::{authorized_admin, IntoResponse};

pub fn routes(routes: RouteSegment<Database>) -> RouteSegment<Database> {
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

#[derive(TemplateOnce)]
#[template(path = "manage/content.stpl.html")]
struct ContentPage {
    languages: Vec<(String, String)>,
    all_languages: Vec<(String, String)>,
}

#[derive(Deserialize)]
struct ContentForm {
    language: String,
}

async fn content(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match super::authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: ContentForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}/", req.url().path(), form.language)));
    }

    let db = req.state();

    let page = ContentPage {
        languages: Block::list_languages(db)
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

    Ok(page.into_response(StatusCode::Ok)?)
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/content_language.stpl.html")]
struct ContentLanguagePage {
    language_name: String,
    groups: Vec<String>,
}

#[derive(Deserialize)]
struct ContentLanguageForm {
    group: String,
}

async fn content_language(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: ContentLanguageForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}/", req.url().path(), form.group)));
    }

    let db = req.state();

    let language = req.param("language")?.to_owned();
    let language_name = common::LANGUAGES
        .get(&language)
        .unwrap_or(&language)
        .to_owned();
    let groups = Block::list_groups(db, &language).await?;

    Ok(ContentLanguagePage {
        language_name,
        groups,
    }
    .into_response(StatusCode::Ok)?)
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/content_group.stpl.html")]
struct ContentGroupPage {
    language_name: String,
    group: String,
    items: Vec<String>,
}

#[derive(Deserialize)]
struct ContentGroupForm {
    item: String,
}

async fn content_group(mut req: tide::Request<Database>) -> tide::Result {
    let _admin = match authorized_admin(&req, &Permission::ManageContent).await {
        Ok(person) => person,
        Err(resp) => return Ok(resp),
    };

    if let Method::Post = req.method() {
        let form: ContentGroupForm = req.body_form().await?;
        return Ok(redirect(&format!("{}{}", req.url().path(), form.item)));
    }

    let db = req.state();

    let language = req.param("language")?.to_owned();
    let language_name = common::LANGUAGES
        .get(&language)
        .unwrap_or(&language)
        .to_owned();
    let group = req.param("group")?.to_owned();
    let items = Block::list_items(db, &language, &group).await?;

    Ok(ContentGroupPage {
        language_name,
        group,
        items,
    }
    .into_response(StatusCode::Ok)?)
}

#[derive(TemplateOnce, Default)]
#[template(path = "manage/content_item.stpl.html")]
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

async fn content_item(mut req: tide::Request<Database>) -> tide::Result {
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
        let db = req.state();
        match Block::load(db, &language, &group, &item).await {
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
        let db = req.state();
        block.tags = form.tags.trim().to_string();
        block.label = form.label.trim().to_string();
        block.content = form.content.trim().to_string();
        block.store(db).await?;
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
        .into_response(StatusCode::Ok)?)
    }
}
