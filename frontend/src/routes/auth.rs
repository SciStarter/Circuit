use poem::web::cookie::CookieJar;
use poem::web::{Data, Form, Query, Redirect};
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::Deserialize;

use crate::chrome::Chrome;
use crate::error::AppError;
use crate::render::page;
use crate::session::{clear_token_cookie, set_token_cookie, token_from_jar, MeResponse};
use crate::AppState;

#[derive(TemplateSimple)]
#[template(path = "pages/login.stpl")]
struct LoginPage {
    next: String,
    error: Option<String>,
}

/// Only allow same-site redirect targets (paths), defaulting to the finder.
fn safe_next(next: Option<String>) -> String {
    match next {
        Some(n) if n.starts_with('/') && !n.starts_with("//") => n,
        _ => "/find".to_string(),
    }
}

#[derive(Debug, Deserialize)]
pub struct NextQuery {
    next: Option<String>,
}

#[handler]
pub async fn login_form(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<NextQuery>,
) -> Result<Response, AppError> {
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Log in | Science Near Me",
        LoginPage {
            next: safe_next(q.next),
            error: None,
        },
    )?
    .into_response())
}

#[derive(Debug, Deserialize)]
pub struct LoginInput {
    email: String,
    password: String,
    #[serde(default)]
    next: Option<String>,
}

#[handler]
pub async fn login_submit(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(input): Form<LoginInput>,
) -> Result<Response, AppError> {
    let next = safe_next(input.next.clone());

    let resp = state
        .api
        .post_json(
            "/api/ui/auth/login",
            None,
            &serde_json::json!({ "email": input.email, "password": input.password }),
        )
        .await?;

    let success = resp.status().is_success();
    let body = resp.text().await.unwrap_or_default();

    if success {
        if let Some(user) = serde_json::from_str::<MeResponse>(&body)
            .ok()
            .and_then(MeResponse::into_user)
        {
            set_token_cookie(jar, user.token);
            return Ok(Redirect::see_other(next).into_response());
        }
    }

    // Surface the API's error message (it returns a short HTML snippet), or a
    // generic message if the response wasn't a usable user.
    let error = if body.is_empty() {
        "Login failed. Please try again.".to_string()
    } else {
        body
    };
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Log in | Science Near Me",
        LoginPage {
            next,
            error: Some(error),
        },
    )?
    .into_response())
}

#[handler]
pub async fn logout(state: Data<&AppState>, jar: &CookieJar) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    // Best-effort server-side logout; ignore failures.
    let _ = state
        .api
        .post_json("/api/ui/auth/logout", token.as_deref(), &serde_json::json!({}))
        .await;
    clear_token_cookie(jar);
    Ok(Redirect::see_other("/find").into_response())
}

/// Build the chrome for an auth page (resolving any existing session).
async fn build_chrome(state: &AppState, jar: &CookieJar, req: &Request) -> Chrome {
    let token = token_from_jar(jar);
    Chrome::build(
        &state.api,
        token.as_deref(),
        req.header("host"),
        req.uri().path().to_string(),
        state.config.domain.clone(),
    )
    .await
}
