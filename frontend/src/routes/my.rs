//! The authenticated `/my/*` account area: a signed-in user's saved
//! opportunities, activity log, goals, profile, and (for partners) the
//! opportunity- and organization-management pages. Every handler requires a
//! session; anonymous visitors are redirected to `/login?next=…`.

use poem::web::cookie::CookieJar;
use poem::web::{Data, Form, Query, Redirect};
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::{Deserialize, Serialize};

use common::model::OpportunityExterior;

use crate::api::json_or_err;
use crate::chrome::Chrome;
use crate::error::AppError;
use crate::opportunity::CardView;
use crate::render::page;
use crate::routes::finder::{PaginationInfo, SearchResults};
use crate::session::token_from_jar;
use crate::AppState;

/// Resolve the session for a `/my/*` page. On success returns the built chrome
/// (which carries the current user) and the session token for API calls. For an
/// anonymous visitor it returns a redirect to the login page that preserves the
/// originally requested path as `next`.
async fn require_user(
    state: &AppState,
    jar: &CookieJar,
    req: &Request,
) -> Result<(Chrome, String), Response> {
    let chrome = Chrome::for_request(state, jar, req).await;
    if chrome.authenticated() {
        let token = token_from_jar(jar).unwrap_or_default();
        Ok((chrome, token))
    } else {
        let dest = req
            .uri()
            .path_and_query()
            .map(|pq| pq.as_str())
            .unwrap_or("/");
        Err(Redirect::see_other(format!("/login?next={}", urlencode(dest))).into_response())
    }
}

/// Percent-encode a string for use in a query-string value.
fn urlencode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(b as char)
            }
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

// ---------------------------------------------------------------------------
// /my/saved — Saved Opportunities
// ---------------------------------------------------------------------------

/// The saved-list query as it appears in the page URL (and is round-tripped
/// through the search box, sort control, and pagination links).
#[derive(Debug, Default, Clone, Deserialize)]
struct SavedQuery {
    #[serde(default)]
    search: String,
    #[serde(default)]
    sort: String,
    #[serde(default)]
    page: Option<i64>,
}

impl SavedQuery {
    /// The effective sort, defaulting to "closest" (matching the old app).
    fn sort_or_default(&self) -> &str {
        if self.sort.is_empty() {
            "closest"
        } else {
            &self.sort
        }
    }

    /// `/my/saved` URL for a given page, preserving the search text and sort.
    fn page_url(&self, page: i64) -> String {
        let mut parts = Vec::new();
        if !self.search.is_empty() {
            parts.push(format!("search={}", urlencode(&self.search)));
        }
        if !self.sort.is_empty() && self.sort != "closest" {
            parts.push(format!("sort={}", urlencode(&self.sort)));
        }
        if page > 0 {
            parts.push(format!("page={page}"));
        }
        if parts.is_empty() {
            "/my/saved".to_string()
        } else {
            format!("/my/saved?{}", parts.join("&"))
        }
    }
}

/// The API query for a saved search (`GET /api/ui/finder/search`).
#[derive(Debug, Serialize)]
struct SavedApiQuery<'a> {
    page: i64,
    per_page: i64,
    saved: bool,
    person: &'a str,
    #[serde(skip_serializing_if = "str::is_empty")]
    text: &'a str,
    sort: &'a str,
}

/// One row in the saved list: the shared card plus the opportunity uid (needed
/// for the per-item "remove" action).
struct SavedItem {
    card: CardView,
    uid: String,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_saved.stpl")]
struct SavedPage {
    search: String,
    sort: String,
    items: Vec<SavedItem>,
    page_index: i64,
    last_page: i64,
    has_prev: bool,
    has_next: bool,
    prev_url: String,
    next_url: String,
}

#[handler]
pub async fn saved_opportunities(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<SavedQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let uid = chrome.uid().unwrap_or_default().to_string();

    let page_index = q.page.unwrap_or(0).max(0);
    let api_query = SavedApiQuery {
        page: page_index,
        per_page: 10,
        saved: true,
        person: &uid,
        text: &q.search,
        sort: q.sort_or_default(),
    };
    let api_qs = serde_qs::to_string(&api_query).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let resp = state
        .api
        .get(&format!("/api/ui/finder/search?{api_qs}"), Some(&token))
        .await?;
    let results: SearchResults = json_or_err(resp).await?;

    let last_page = results.pagination.last_page;
    let items = results
        .matches
        .into_iter()
        .map(|opp| SavedItem {
            uid: opp.uid.to_string(),
            card: CardView::from(opp),
        })
        .collect();

    Ok(page(
        chrome,
        "Saved Opportunities | Science Near Me",
        SavedPage {
            search: q.search.clone(),
            sort: q.sort_or_default().to_string(),
            items,
            page_index,
            last_page,
            has_prev: page_index > 0,
            has_next: page_index < last_page,
            prev_url: q.page_url(page_index - 1),
            next_url: q.page_url(page_index + 1),
        },
    )?
    .into_response())
}

/// Where to return after a saved-list mutation, preserving the search/sort the
/// user was viewing.
#[derive(Debug, Default, Deserialize)]
struct SavedReturn {
    #[serde(default)]
    search: String,
    #[serde(default)]
    sort: String,
}

impl SavedReturn {
    fn back(&self) -> String {
        SavedQuery {
            search: self.search.clone(),
            sort: self.sort.clone(),
            page: None,
        }
        .page_url(0)
    }
}

#[handler]
pub async fn remove_saved(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(uid): poem::web::Path<String>,
    Form(ret): Form<SavedReturn>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let _ = state
        .api
        .delete(&format!("/api/ui/profile/saved/{uid}"), Some(&token))
        .await?;
    Ok(Redirect::see_other(ret.back()).into_response())
}

#[handler]
pub async fn remove_old_saved(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(ret): Form<SavedReturn>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let _ = state
        .api
        .delete("/api/ui/profile/saved/old", Some(&token))
        .await?;
    Ok(Redirect::see_other(ret.back()).into_response())
}

// ---------------------------------------------------------------------------
// /my/science — Activity Log
//
// Two views over the user's `Involvement` records (`/api/ui/profile/involved`):
//   • "Report Your Science": opportunities the user signalled interest in
//     (Interest..=Saved) but hasn't confirmed — each offers "I did this" /
//     "I didn't do this".
//   • "Your Activity Log": confirmed participation (Logged and above),
//     searchable by text.
// `Mode` is serialized by the API as an integer (Ignored=5, Interest=10,
// Saved=20, Logged=30); we send those literals.
// ---------------------------------------------------------------------------

const MODE_IGNORED: i32 = 5;
const MODE_INTEREST: i32 = 10;
const MODE_SAVED: i32 = 20;
const MODE_LOGGED: i32 = 30;

/// One involvement row from `/api/ui/profile/involved` (with `opp=true`, so the
/// opportunity is embedded).
#[derive(Debug, Deserialize)]
struct InvolvedRow {
    id: i32,
    opportunity: OpportunityExterior,
}

#[derive(Debug, Deserialize)]
struct InvolvedResults {
    pagination: PaginationInfo,
    matches: Vec<InvolvedRow>,
}

/// The involved-list API query.
#[derive(Debug, Serialize)]
struct InvolvedApiQuery<'a> {
    page: i64,
    min: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    max: Option<i32>,
    opp: bool,
    #[serde(skip_serializing_if = "str::is_empty")]
    text: &'a str,
}

#[derive(Debug, Default, Clone, Deserialize)]
struct ScienceQuery {
    /// Which tab is active on load: "report" or "log" (default "report").
    #[serde(default)]
    tab: String,
    /// Page within the "Report Your Science" list.
    #[serde(default)]
    rpage: Option<i64>,
    /// Page within the "Your Activity Log" list.
    #[serde(default)]
    lpage: Option<i64>,
    /// Search text for the activity log.
    #[serde(default)]
    text: String,
}

impl ScienceQuery {
    /// Build a `/my/science` URL, overriding the page of one tab while keeping
    /// the other tab's state and the active-tab/search context.
    fn url(&self, tab: &str, rpage: i64, lpage: i64) -> String {
        let mut parts = vec![format!("tab={tab}")];
        if rpage > 0 {
            parts.push(format!("rpage={rpage}"));
        }
        if lpage > 0 {
            parts.push(format!("lpage={lpage}"));
        }
        if !self.text.is_empty() {
            parts.push(format!("text={}", urlencode(&self.text)));
        }
        format!("/my/science?{}", parts.join("&"))
    }
}

/// A paginated list of involvement cards (one per tab).
struct InvolvedList {
    items: Vec<InvolvedItem>,
    total: i64,
    page_index: i64,
    last_page: i64,
    has_prev: bool,
    has_next: bool,
    prev_url: String,
    next_url: String,
}

/// One activity row: the shared card plus the involvement id (for "report"
/// actions).
struct InvolvedItem {
    card: CardView,
    id: i32,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_science.stpl")]
struct SciencePage {
    active_tab: String,
    search: String,
    report: InvolvedList,
    log: InvolvedList,
}

/// Fetch one involved list and turn it into a renderable, paginated `InvolvedList`.
async fn fetch_involved(
    state: &AppState,
    token: &str,
    query: &InvolvedApiQuery<'_>,
    make_url: impl Fn(i64) -> String,
) -> Result<InvolvedList, AppError> {
    let qs = serde_qs::to_string(query).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let resp = state
        .api
        .get(&format!("/api/ui/profile/involved?{qs}"), Some(token))
        .await?;
    let results: InvolvedResults = json_or_err(resp).await?;

    let page_index = results.pagination.page_index;
    let last_page = results.pagination.last_page;
    Ok(InvolvedList {
        total: results.pagination.total,
        page_index,
        last_page,
        has_prev: page_index > 0,
        has_next: page_index < last_page,
        prev_url: make_url(page_index - 1),
        next_url: make_url(page_index + 1),
        items: results
            .matches
            .into_iter()
            .map(|row| InvolvedItem {
                id: row.id,
                card: CardView::from(row.opportunity),
            })
            .collect(),
    })
}

#[handler]
pub async fn science(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<ScienceQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };

    let rpage = q.rpage.unwrap_or(0).max(0);
    let lpage = q.lpage.unwrap_or(0).max(0);

    let report = fetch_involved(
        &state,
        &token,
        &InvolvedApiQuery {
            page: rpage,
            min: MODE_INTEREST,
            max: Some(MODE_SAVED),
            opp: true,
            text: "",
        },
        |p| q.url("report", p, lpage),
    )
    .await?;

    let log = fetch_involved(
        &state,
        &token,
        &InvolvedApiQuery {
            page: lpage,
            min: MODE_LOGGED,
            max: None,
            opp: true,
            text: &q.text,
        },
        |p| q.url("log", rpage, p),
    )
    .await?;

    let active_tab = if q.tab == "log" { "log" } else { "report" }.to_string();

    Ok(page(
        chrome,
        "My Activity Log | Science Near Me",
        SciencePage {
            active_tab,
            search: q.text.clone(),
            report,
            log,
        },
    )?
    .into_response())
}

/// "I did this" (mode=Logged) / "I didn't do this" (mode=Ignored) on a pending
/// item, then return to the report tab.
#[derive(Debug, Deserialize)]
struct ReportInput {
    id: i32,
    mode: i32,
}

#[handler]
pub async fn report_involvement(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(input): Form<ReportInput>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    // Only the two report transitions are valid here.
    let mode = if input.mode == MODE_IGNORED {
        MODE_IGNORED
    } else {
        MODE_LOGGED
    };
    let _ = state
        .api
        .post_json(
            "/api/ui/profile/involved",
            Some(&token),
            &serde_json::json!({ "id": input.id, "mode": mode }),
        )
        .await?;
    Ok(Redirect::see_other("/my/science?tab=report").into_response())
}

// ---------------------------------------------------------------------------
// /my/profile — Profile & Settings
//
// The full profile is round-tripped through `GET`/`PUT /api/ui/profile/`: the
// API's `PUT` overwrites every field, so we fetch the current record, overlay
// the submitted form values, and write the merged object back — preserving
// fields the form doesn't expose (image_url, whatsapp, research opt-ins).
// Demographic enums are passed through verbatim as their kebab-case codes.
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
struct Profile {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    #[serde(default)]
    email: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    password: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    first_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    last_name: Option<String>,
    #[serde(default)]
    genders: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    gender_other: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    whatsapp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    zip_code: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    birth_year: Option<u32>,
    #[serde(default)]
    ethnicities: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ethnicity_other: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    family_income: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    education_level: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    opt_in_research: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    opt_in_volunteer: Option<bool>,
    #[serde(default)]
    private: bool,
    #[serde(default)]
    allow_emails: bool,
}

impl Profile {
    fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("")
    }
    fn first_name(&self) -> &str {
        self.first_name.as_deref().unwrap_or("")
    }
    fn last_name(&self) -> &str {
        self.last_name.as_deref().unwrap_or("")
    }
    fn phone(&self) -> &str {
        self.phone.as_deref().unwrap_or("")
    }
    fn zip_code(&self) -> &str {
        self.zip_code.as_deref().unwrap_or("")
    }
    fn gender_other(&self) -> &str {
        self.gender_other.as_deref().unwrap_or("")
    }
    fn ethnicity_other(&self) -> &str {
        self.ethnicity_other.as_deref().unwrap_or("")
    }
    fn has_gender(&self, v: &str) -> bool {
        self.genders.iter().any(|g| g == v)
    }
    fn has_ethnicity(&self, v: &str) -> bool {
        self.ethnicities.iter().any(|e| e == v)
    }
    fn income_is(&self, v: &str) -> bool {
        self.family_income.as_deref() == Some(v)
    }
    fn education_is(&self, v: &str) -> bool {
        self.education_level.as_deref() == Some(v)
    }
}

/// The submitted profile form. Array fields use `name="genders[]"` /
/// `name="ethnicities[]"`, so the body is parsed with `serde_qs` (which, in
/// non-strict mode, folds repeated bracketed keys into a `Vec`) rather than the
/// flat `Form` extractor. Checkbox booleans are `Option<String>` (present ==
/// checked).
#[derive(Debug, Default, Deserialize)]
struct ProfileForm {
    #[serde(default)]
    email: String,
    #[serde(default)]
    password: String,
    #[serde(default)]
    username: String,
    #[serde(default)]
    zip_code: String,
    #[serde(default)]
    phone: String,
    #[serde(default)]
    first_name: String,
    #[serde(default)]
    last_name: String,
    #[serde(default)]
    private: Option<String>,
    #[serde(default)]
    allow_emails: Option<String>,
    #[serde(default)]
    genders: Vec<String>,
    #[serde(default)]
    gender_other: String,
    #[serde(default)]
    birth_year: String,
    #[serde(default)]
    ethnicities: Vec<String>,
    #[serde(default)]
    ethnicity_other: String,
    #[serde(default)]
    family_income: String,
    #[serde(default)]
    education_level: String,
}

/// Fold a non-empty string into `Some`, an empty one into `None`.
fn some_if_set(s: String) -> Option<String> {
    if s.trim().is_empty() {
        None
    } else {
        Some(s)
    }
}

impl ProfileForm {
    /// Overlay the submitted values onto the existing profile, preserving the
    /// fields the form doesn't manage.
    fn apply_to(self, p: &mut Profile) {
        p.email = self.email;
        // A blank password field leaves the password unchanged.
        p.password = some_if_set(self.password);
        p.username = some_if_set(self.username);
        p.zip_code = some_if_set(self.zip_code);
        p.phone = some_if_set(self.phone);
        p.first_name = some_if_set(self.first_name);
        p.last_name = some_if_set(self.last_name);
        p.private = self.private.is_some();
        p.allow_emails = self.allow_emails.is_some();
        p.genders = self.genders;
        p.gender_other = some_if_set(self.gender_other);
        p.birth_year = self.birth_year.trim().parse().ok();
        p.ethnicities = self.ethnicities;
        p.ethnicity_other = some_if_set(self.ethnicity_other);
        p.family_income = some_if_set(self.family_income);
        p.education_level = some_if_set(self.education_level);
    }
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_profile.stpl")]
struct ProfilePage {
    profile: Profile,
    saved: bool,
    pwreset: bool,
}

#[derive(Debug, Default, Deserialize)]
struct ProfileQuery {
    #[serde(default)]
    saved: Option<String>,
    #[serde(default)]
    pwreset: Option<String>,
}

#[handler]
pub async fn profile_settings(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<ProfileQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let profile: Profile = state
        .api
        .get_json("/api/ui/profile/", Some(&token))
        .await?;

    Ok(page(
        chrome,
        "My Profile & Settings | Science Near Me",
        ProfilePage {
            profile,
            saved: q.saved.is_some(),
            pwreset: q.pwreset.is_some(),
        },
    )?
    .into_response())
}

#[handler]
pub async fn save_profile(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    body: String,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let form: ProfileForm = serde_qs::Config::new(5, false)
        .deserialize_str(&body)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Merge onto the current record so unmanaged fields survive the overwrite.
    let mut profile: Profile = state
        .api
        .get_json("/api/ui/profile/", Some(&token))
        .await?;
    form.apply_to(&mut profile);

    let resp = state
        .api
        .put_json("/api/ui/profile/", Some(&token), &profile)
        .await?;
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::ApiStatus {
            status: poem::http::StatusCode::from_u16(status.as_u16())
                .unwrap_or(poem::http::StatusCode::BAD_GATEWAY),
            body,
        });
    }
    Ok(Redirect::see_other("/my/profile?saved=1").into_response())
}

#[handler]
pub async fn delete_account(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let _ = state
        .api
        .delete("/api/ui/profile/", Some(&token))
        .await?;
    crate::session::clear_token_cookie(jar);
    Ok(Redirect::see_other("/").into_response())
}
