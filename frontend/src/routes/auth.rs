use poem::web::cookie::CookieJar;
use poem::web::{Data, Form, Query, Redirect};
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::Deserialize;

use crate::chrome::Chrome;
use crate::error::AppError;
use crate::render::page;
use crate::session::{
    clear_token_cookie, current_user, set_token_cookie, token_from_jar, MeResponse,
};
use crate::AppState;

// ---------------------------------------------------------------------------
// Shared template contexts
//
// The login and signup forms each appear on their own standalone page *and*
// side-by-side on `/account`. To keep a single source of truth, the form markup
// lives in `partials/{login,signup}_form.stpl` and is `include!`d by each page.
// Sailfish includes share the caller's scope, so the partials reference a single
// context field (`login` / `signup`) — avoiding name clashes when both forms
// render on the same page.
// ---------------------------------------------------------------------------

#[derive(Default)]
pub struct LoginFormCtx {
    pub next: String,
    pub email: String,
    pub error: Option<String>,
}

impl LoginFormCtx {
    fn fresh(next: String) -> Self {
        LoginFormCtx {
            next,
            ..Default::default()
        }
    }
}

pub struct SignupFormCtx {
    pub next: String,
    pub email: String,
    pub username: String,
    pub zip_code: String,
    pub phone: String,
    pub newsletter: bool,
    pub agree: bool,
    pub error: Option<String>,
}

impl SignupFormCtx {
    /// A blank signup form. Newsletter and agreement default to checked, matching
    /// the old SignupForm.vue defaults.
    fn fresh(next: String) -> Self {
        SignupFormCtx {
            next,
            email: String::new(),
            username: String::new(),
            zip_code: String::new(),
            phone: String::new(),
            newsletter: true,
            agree: true,
            error: None,
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

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

/// If the request already carries a valid session, produce a redirect to
/// `next`. Used by the login/signup pages, which the old app skipped for
/// already-authenticated visitors.
async fn redirect_if_authenticated(
    state: &AppState,
    jar: &CookieJar,
    next: &str,
) -> Option<Response> {
    let token = token_from_jar(jar);
    if current_user(&state.api, token.as_deref()).await.is_some() {
        Some(Redirect::see_other(next.to_string()).into_response())
    } else {
        None
    }
}

/// Outcome of posting credentials to an auth endpoint that returns a user.
enum AuthOutcome {
    /// Session established and cookie set.
    Success,
    /// The API rejected the attempt; carries a message (may contain HTML, e.g.
    /// the login endpoint's "Forgot password?" link).
    Failure(String),
}

/// POST a credential payload to an auth endpoint that returns a Person JSON on
/// success, set the session cookie from the returned token, and report the
/// outcome. Shared by the email/password login, SciStarter login, and signup
/// flows — every one of which authenticates by the same contract.
///
/// On failure the API's response body is surfaced (signup/SciStarter return a
/// descriptive message), but the `login` endpoint replies `403` with an empty
/// body, so `fallback` supplies a context-appropriate message in that case.
async fn establish_session(
    state: &AppState,
    jar: &CookieJar,
    path: &str,
    body: &serde_json::Value,
    fallback: &str,
) -> Result<AuthOutcome, AppError> {
    let resp = state.api.post_json(path, None, body).await?;
    let success = resp.status().is_success();
    let body = resp.text().await.unwrap_or_default();

    if success {
        if let Some(user) = serde_json::from_str::<MeResponse>(&body)
            .ok()
            .and_then(MeResponse::into_user)
        {
            set_token_cookie(jar, user.token);
            return Ok(AuthOutcome::Success);
        }
    }

    let error = if body.trim().is_empty() {
        fallback.to_string()
    } else {
        body
    };
    Ok(AuthOutcome::Failure(error))
}

// ---------------------------------------------------------------------------
// /login
// ---------------------------------------------------------------------------

#[derive(TemplateSimple)]
#[template(path = "pages/login.stpl")]
struct LoginPage {
    login: LoginFormCtx,
}

#[handler]
pub async fn login_form(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<NextQuery>,
) -> Result<Response, AppError> {
    let next = safe_next(q.next);
    if let Some(redirect) = redirect_if_authenticated(&state, jar, &next).await {
        return Ok(redirect);
    }
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Log in | Science Near Me",
        LoginPage {
            login: LoginFormCtx::fresh(next),
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

    let outcome = establish_session(
        &state,
        jar,
        "/api/ui/auth/login",
        &serde_json::json!({ "email": input.email, "password": input.password }),
        r#"Wrong email or password. <a href="/forgot">Forgot password?</a>"#,
    )
    .await?;

    match outcome {
        AuthOutcome::Success => Ok(Redirect::see_other(next).into_response()),
        AuthOutcome::Failure(error) => {
            let chrome = build_chrome(&state, jar, req).await;
            Ok(page(
                chrome,
                "Log in | Science Near Me",
                LoginPage {
                    login: LoginFormCtx {
                        next,
                        email: input.email,
                        error: Some(error),
                    },
                },
            )?
            .into_response())
        }
    }
}

// ---------------------------------------------------------------------------
// /signup
// ---------------------------------------------------------------------------

#[derive(TemplateSimple)]
#[template(path = "pages/signup.stpl")]
struct SignupPage {
    signup: SignupFormCtx,
}

#[handler]
pub async fn signup_form(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<NextQuery>,
) -> Result<Response, AppError> {
    let next = safe_next(q.next);
    if let Some(redirect) = redirect_if_authenticated(&state, jar, &next).await {
        return Ok(redirect);
    }
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Create an account | Science Near Me",
        SignupPage {
            signup: SignupFormCtx::fresh(next),
        },
    )?
    .into_response())
}

#[derive(Debug, Deserialize)]
pub struct SignupInput {
    email: String,
    username: String,
    password: String,
    password_repeat: String,
    #[serde(default)]
    zip_code: String,
    #[serde(default)]
    phone: String,
    // Unchecked checkboxes are omitted by the browser, so presence == checked.
    #[serde(default)]
    agree: Option<String>,
    #[serde(default)]
    newsletter: Option<String>,
    #[serde(default)]
    next: Option<String>,
}

/// Light server-side validation mirroring the old SignupForm.vue rules. Returns
/// an error message if the input is unacceptable.
fn validate_signup(input: &SignupInput, agree: bool) -> Option<&'static str> {
    let at = input.email.find('@');
    if !matches!(at, Some(i) if i > 0 && i < input.email.len() - 1) {
        return Some("Invalid email address");
    }
    if input.password.len() < 7 {
        return Some("Password is too short");
    }
    if input.password != input.password_repeat {
        return Some("Passwords do not match");
    }
    if !agree {
        return Some("You must agree to the Terms of Service to create an account.");
    }
    None
}

#[handler]
pub async fn signup_submit(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(input): Form<SignupInput>,
) -> Result<Response, AppError> {
    let next = safe_next(input.next.clone());
    let agree = input.agree.is_some();
    let newsletter = input.newsletter.is_some();

    // Re-render the form (preserving non-secret values) with an error message.
    let reject = |error: String| {
        SignupFormCtx {
            next: next.clone(),
            email: input.email.clone(),
            username: input.username.clone(),
            zip_code: input.zip_code.clone(),
            phone: input.phone.clone(),
            newsletter,
            agree,
            error: Some(error),
        }
    };

    if let Some(msg) = validate_signup(&input, agree) {
        let chrome = build_chrome(&state, jar, req).await;
        return Ok(page(
            chrome,
            "Create an account | Science Near Me",
            SignupPage {
                signup: reject(msg.to_string()),
            },
        )?
        .into_response());
    }

    let mut body = serde_json::json!({
        "email": input.email,
        "username": input.username,
        "password": input.password,
        "newsletter": newsletter,
    });
    if !input.zip_code.is_empty() {
        body["zip_code"] = input.zip_code.clone().into();
    }
    if !input.phone.is_empty() {
        body["phone"] = input.phone.clone().into();
    }

    let outcome = establish_session(
        &state,
        jar,
        "/api/ui/auth/signup",
        &body,
        "We couldn't create your account. Please try again.",
    )
    .await?;

    match outcome {
        AuthOutcome::Success => Ok(Redirect::see_other(next).into_response()),
        AuthOutcome::Failure(error) => {
            let chrome = build_chrome(&state, jar, req).await;
            Ok(page(
                chrome,
                "Create an account | Science Near Me",
                SignupPage {
                    signup: reject(error),
                },
            )?
            .into_response())
        }
    }
}

// ---------------------------------------------------------------------------
// /login-scistarter
// ---------------------------------------------------------------------------

#[derive(TemplateSimple)]
#[template(path = "pages/login_scistarter.stpl")]
struct ScistarterPage {
    next: String,
    email: String,
    error: Option<String>,
}

#[handler]
pub async fn scistarter_form(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<NextQuery>,
) -> Result<Response, AppError> {
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Log in with SciStarter | Science Near Me",
        ScistarterPage {
            next: safe_next(q.next),
            email: String::new(),
            error: None,
        },
    )?
    .into_response())
}

#[handler]
pub async fn scistarter_submit(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(input): Form<LoginInput>,
) -> Result<Response, AppError> {
    let next = safe_next(input.next.clone());

    let outcome = establish_session(
        &state,
        jar,
        "/api/ui/auth/login-scistarter",
        &serde_json::json!({ "email": input.email, "password": input.password }),
        "Incorrect SciStarter email or password.",
    )
    .await?;

    match outcome {
        AuthOutcome::Success => Ok(Redirect::see_other(next).into_response()),
        AuthOutcome::Failure(error) => {
            let chrome = build_chrome(&state, jar, req).await;
            Ok(page(
                chrome,
                "Log in with SciStarter | Science Near Me",
                ScistarterPage {
                    next,
                    email: input.email,
                    error: Some(error),
                },
            )?
            .into_response())
        }
    }
}

// ---------------------------------------------------------------------------
// /forgot
// ---------------------------------------------------------------------------

#[derive(TemplateSimple)]
#[template(path = "pages/forgot.stpl")]
struct ForgotPage {
    sent: bool,
}

#[handler]
pub async fn forgot_form(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Forgot your password? | Science Near Me",
        ForgotPage { sent: false },
    )?
    .into_response())
}

#[derive(Debug, Deserialize)]
pub struct ForgotInput {
    email: String,
}

#[handler]
pub async fn forgot_submit(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(input): Form<ForgotInput>,
) -> Result<Response, AppError> {
    // Best-effort: the API errors if the address is unknown, but we never reveal
    // whether an account exists — always show the same confirmation.
    let _ = state
        .api
        .post_json(
            "/api/ui/auth/reset",
            None,
            &serde_json::json!({ "email": input.email }),
        )
        .await;

    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Forgot your password? | Science Near Me",
        ForgotPage { sent: true },
    )?
    .into_response())
}

// ---------------------------------------------------------------------------
// /account — combined login + signup entry (anonymous)
// ---------------------------------------------------------------------------

#[derive(TemplateSimple)]
#[template(path = "pages/account.stpl")]
struct AccountPage {
    login: LoginFormCtx,
    signup: SignupFormCtx,
}

#[handler]
pub async fn account(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<NextQuery>,
) -> Result<Response, AppError> {
    let next = safe_next(q.next);
    let chrome = build_chrome(&state, jar, req).await;
    Ok(page(
        chrome,
        "Sign in or create an account | Science Near Me",
        AccountPage {
            login: LoginFormCtx::fresh(next.clone()),
            signup: SignupFormCtx::fresh(next),
        },
    )?
    .into_response())
}

// ---------------------------------------------------------------------------
// /logout
// ---------------------------------------------------------------------------

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
