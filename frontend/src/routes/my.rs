//! The authenticated `/my/*` account area: a signed-in user's saved
//! opportunities, activity log, goals, profile, and (for partners) the
//! opportunity- and organization-management pages. Every handler requires a
//! session; anonymous visitors are redirected to `/login?next=…`.

use chrono::{DateTime, Duration, FixedOffset, Utc};
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
use crate::render::{page, Render};
use crate::routes::opp_form::{empty_or, empty_to_null};
use crate::routes::finder::{PaginationInfo, SearchResults};
use crate::session::token_from_jar;
use crate::AppState;

/// Resolve the session for a `/my/*` page. On success returns the built chrome
/// (which carries the current user) and the session token for API calls. For an
/// anonymous visitor it returns a redirect to the login page that preserves the
/// originally requested path as `next`.
pub(crate) async fn require_user(
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
pub(crate) fn urlencode(s: &str) -> String {
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

/// The swappable results region of the saved list (`#saved-results`): rendered
/// inside the page on first load, and returned on its own to HTMX after a
/// per-item or bulk removal so pagination and the empty state stay correct
/// without a full-page reload. Carries `search`/`sort` so the re-rendered
/// remove forms and pagination links keep the current view.
#[derive(TemplateSimple)]
#[template(path = "partials/my_saved_results.stpl")]
struct SavedResultsView {
    search: String,
    sort: String,
    page: i64,
    items: Vec<SavedItem>,
    page_index: i64,
    last_page: i64,
    has_prev: bool,
    has_next: bool,
    prev_url: String,
    next_url: String,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_saved.stpl")]
struct SavedPage {
    search: String,
    sort: String,
    results_html: String,
}

/// Run the saved search for `q` and build the renderable results region. Shared
/// by the full-page load and the HTMX removal responses (single source of the
/// list rendering).
async fn saved_results(
    state: &AppState,
    token: &str,
    uid: &str,
    q: &SavedQuery,
) -> Result<SavedResultsView, AppError> {
    let page_index = q.page.unwrap_or(0).max(0);
    let api_query = SavedApiQuery {
        page: page_index,
        per_page: 10,
        saved: true,
        person: uid,
        text: &q.search,
        sort: q.sort_or_default(),
    };
    let api_qs = serde_qs::to_string(&api_query).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let resp = state
        .api
        .get(&format!("/api/ui/finder/search?{api_qs}"), Some(token))
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

    Ok(SavedResultsView {
        search: q.search.clone(),
        sort: q.sort_or_default().to_string(),
        page: page_index,
        items,
        page_index,
        last_page,
        has_prev: page_index > 0,
        has_next: page_index < last_page,
        prev_url: q.page_url(page_index - 1),
        next_url: q.page_url(page_index + 1),
    })
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
    let results_html = saved_results(&state, &token, &uid, &q).await?.render_once()?;

    Ok(page(
        chrome,
        "Saved Opportunities | Science Near Me",
        SavedPage {
            search: q.search.clone(),
            sort: q.sort_or_default().to_string(),
            results_html,
        },
    )?
    .into_response())
}

/// Either re-render the results region (HTMX requests) or fall back to a
/// redirect that reloads the saved page (no-JS), preserving the current view.
async fn saved_mutation_response(
    state: &AppState,
    chrome: &Chrome,
    token: &str,
    req: &Request,
    q: &SavedQuery,
) -> Result<Response, AppError> {
    if req.header("HX-Request").is_some() {
        let uid = chrome.uid().unwrap_or_default().to_string();
        let view = saved_results(state, token, &uid, q).await?;
        Ok(Render(view).into_response())
    } else {
        Ok(Redirect::see_other(q.page_url(q.page.unwrap_or(0))).into_response())
    }
}

#[handler]
pub async fn remove_saved(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(uid): poem::web::Path<String>,
    Form(q): Form<SavedQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let _ = state
        .api
        .delete(&format!("/api/ui/profile/saved/{uid}"), Some(&token))
        .await?;
    saved_mutation_response(&state, &chrome, &token, req, &q).await
}

#[handler]
pub async fn remove_old_saved(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(q): Form<SavedQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let _ = state
        .api
        .delete("/api/ui/profile/saved/old", Some(&token))
        .await?;
    saved_mutation_response(&state, &chrome, &token, req, &q).await
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

/// The swappable report-tab region (`#report-results`), shared by the page load
/// and the HTMX response after an "I did/didn't do this" action. When `oob` is
/// set it also emits an `<hx-partial>` that updates *every* pending-count badge
/// at once — the tab badge and both (mobile + desktop) nav badges, matched by
/// their shared `.reports-badge` class (htmx 4 multi-target update).
#[derive(TemplateSimple)]
#[template(path = "partials/my_science_report.stpl")]
struct ReportResultsView {
    list: InvolvedList,
    oob: bool,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_science.stpl")]
struct SciencePage {
    active_tab: String,
    search: String,
    report_total: i64,
    report_html: String,
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
    let report_total = report.total;
    let report_html = ReportResultsView {
        list: report,
        oob: false,
    }
    .render_once()?;

    Ok(page(
        chrome,
        "My Activity Log | Science Near Me",
        SciencePage {
            active_tab,
            search: q.text.clone(),
            report_total,
            report_html,
            log,
        },
    )?
    .into_response())
}

/// Fetch the pending-report list (Interest..=Saved) for the given page, with
/// pagination links pointing back at the report tab.
async fn report_list(
    state: &AppState,
    token: &str,
    rpage: i64,
) -> Result<InvolvedList, AppError> {
    let sq = ScienceQuery {
        tab: "report".to_string(),
        rpage: Some(rpage),
        lpage: None,
        text: String::new(),
    };
    fetch_involved(
        state,
        token,
        &InvolvedApiQuery {
            page: rpage,
            min: MODE_INTEREST,
            max: Some(MODE_SAVED),
            opp: true,
            text: "",
        },
        |p| sq.url("report", p, 0),
    )
    .await
}

/// "I did this" (mode=Logged) / "I didn't do this" (mode=Ignored) on a pending
/// item. For HTMX it re-renders the report region and updates every pending
/// badge in one response; otherwise it falls back to a redirect.
#[derive(Debug, Deserialize)]
struct ReportInput {
    id: i32,
    mode: i32,
    #[serde(default)]
    rpage: Option<i64>,
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

    if req.header("HX-Request").is_some() {
        let rpage = input.rpage.unwrap_or(0).max(0);
        let list = report_list(&state, &token, rpage).await?;
        Ok(Render(ReportResultsView { list, oob: true }).into_response())
    } else {
        Ok(Redirect::see_other("/my/science?tab=report").into_response())
    }
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

// ---------------------------------------------------------------------------
// /my/goals — Goals
//
// The API returns only the user's *working* goals (`GET /api/ui/profile/goals`),
// each with the participation `progress` accrued within its window. A working
// goal whose progress has reached its target is "succeeded" (shown with a
// congratulations banner until the user starts a new one); one that has expired
// without reaching its target is "failed". With no goals at all, the page shows
// the goal-setting cards. All times are kept in their original offset.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct ProgressOpp {
    slug: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct GoalProgress {
    opportunity: ProgressOpp,
}

#[derive(Debug, Deserialize)]
struct ApiGoal {
    id: i32,
    category: String,
    target: i32,
    begin: DateTime<FixedOffset>,
    end: DateTime<FixedOffset>,
    #[serde(default)]
    progress: Vec<GoalProgress>,
}

struct GoalItem {
    slug: String,
    title: String,
}

/// A goal prepared for display: progress/time computed against "now" so the
/// template stays declarative.
struct GoalView {
    id: i32,
    category: String,
    target: i32,
    count: i32,
    reached: bool,
    failed: bool,
    begin_label: String,
    end_label: String,
    duration_label: String,
    days_left: i64,
    opp_pct: i64,
    time_pct: i64,
    items: Vec<GoalItem>,
}

/// Human-friendly date `M/D/YYYY` (matching the old `toLocaleDateString`).
fn date_label(dt: &DateTime<FixedOffset>) -> String {
    dt.format("%-m/%-d/%Y").to_string()
}

/// Coarse humanized span for a goal window (every preset is one year).
fn humanize_days(d: i64) -> String {
    if (360..=372).contains(&d) {
        "1 year".to_string()
    } else if d == 1 {
        "1 day".to_string()
    } else {
        format!("{d} days")
    }
}

impl GoalView {
    fn from(g: ApiGoal, now: DateTime<FixedOffset>) -> GoalView {
        let count = g.progress.len() as i32;
        let reached = count >= g.target;
        let expired = g.end < now;
        let days_total = (g.end - g.begin).num_days().max(1);
        let days_elapsed = (now - g.begin).num_days().clamp(0, days_total);
        let days_left = (g.end - now).num_days().max(0);
        let opp_pct = if g.target > 0 {
            ((count as i64 * 100) / g.target as i64).clamp(0, 100)
        } else {
            0
        };
        let time_pct = ((days_elapsed * 100) / days_total).clamp(0, 100);

        GoalView {
            id: g.id,
            category: g.category,
            target: g.target,
            count,
            reached,
            failed: !reached && expired,
            begin_label: date_label(&g.begin),
            end_label: date_label(&g.end),
            duration_label: humanize_days(days_total),
            days_left,
            opp_pct,
            time_pct,
            items: g
                .progress
                .into_iter()
                .map(|p| GoalItem {
                    slug: p.opportunity.slug,
                    title: p.opportunity.title,
                })
                .collect(),
        }
    }
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_goals.stpl")]
struct GoalsPage {
    set_mode: bool,
    goals: Vec<GoalView>,
}

#[handler]
pub async fn goals_page(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let api: Vec<ApiGoal> = state
        .api
        .get_json("/api/ui/profile/goals", Some(&token))
        .await?;
    let now = Utc::now().fixed_offset();
    let goals: Vec<GoalView> = api.into_iter().map(|g| GoalView::from(g, now)).collect();
    let set_mode = goals.is_empty();

    Ok(page(
        chrome,
        "My Goals | Science Near Me",
        GoalsPage { set_mode, goals },
    )?
    .into_response())
}

/// One of the three preset goals being set.
#[derive(Debug, Deserialize)]
struct SetGoalInput {
    category: String,
    target: i32,
}

#[handler]
pub async fn set_goal(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Form(input): Form<SetGoalInput>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let begin = Utc::now();
    let end = begin + Duration::days(366);
    let _ = state
        .api
        .post_json(
            "/api/ui/profile/goals",
            Some(&token),
            &serde_json::json!({
                "category": input.category,
                "target": input.target,
                "begin": begin.to_rfc3339(),
                "end": end.to_rfc3339(),
                "status": "working",
            }),
        )
        .await?;
    Ok(Redirect::see_other("/my/goals").into_response())
}

/// Transition a working goal to a terminal status ("succeeded" when the user
/// retires a met goal, "failed" when they give up on an expired one). The full
/// goal is re-read from the API so we don't trust client-supplied dates/targets.
#[derive(Debug, Deserialize)]
struct GoalStatusInput {
    status: String,
}

#[handler]
pub async fn update_goal_status(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(id): poem::web::Path<i32>,
    Form(input): Form<GoalStatusInput>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    // Only terminal transitions are permitted here.
    let status = match input.status.as_str() {
        "succeeded" => "succeeded",
        "failed" => "failed",
        _ => return Ok(Redirect::see_other("/my/goals").into_response()),
    };
    let api: Vec<ApiGoal> = state
        .api
        .get_json("/api/ui/profile/goals", Some(&token))
        .await?;
    if let Some(g) = api.into_iter().find(|g| g.id == id) {
        let _ = state
            .api
            .put_json(
                &format!("/api/ui/profile/goals/{id}"),
                Some(&token),
                &serde_json::json!({
                    "id": g.id,
                    "category": g.category,
                    "target": g.target,
                    "begin": g.begin.to_rfc3339(),
                    "end": g.end.to_rfc3339(),
                    "status": status,
                }),
            )
            .await?;
    }
    Ok(Redirect::see_other("/my/goals").into_response())
}

#[handler]
pub async fn delete_goal(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(id): poem::web::Path<i32>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let _ = state
        .api
        .delete(&format!("/api/ui/profile/goals/{id}"), Some(&token))
        .await?;
    Ok(Redirect::see_other("/my/goals").into_response())
}

// ---------------------------------------------------------------------------
// /my/opportunities — Your Opportunities (owner)
//
// Three views over the partner's opportunities, all from
// `/api/ui/finder/search?mine=true&sort=alphabetical`:
//   • live  — current=true
//   • draft — current=false, withdrawn=true
//   • past  — current=false, withdrawn=false
// All three lists load on each render (CSS tabs switch between them); the active
// tab additionally honours a text search and page. Editing/trashing acts per
// opportunity.
// ---------------------------------------------------------------------------

/// API query for one owner tab (`/api/ui/finder/search`).
#[derive(Debug, Serialize)]
struct OwnerApiQuery<'a> {
    mine: bool,
    sort: &'a str,
    current: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    withdrawn: Option<bool>,
    page: i64,
    per_page: i64,
    #[serde(skip_serializing_if = "str::is_empty")]
    text: &'a str,
}

/// One owner row: shared card plus the identifiers the owner actions need.
struct OwnerItem {
    card: CardView,
    uid: String,
    slug: String,
}

/// A renderable owner tab.
struct OwnerList {
    items: Vec<OwnerItem>,
    editable: bool,
    page_index: i64,
    last_page: i64,
    has_prev: bool,
    has_next: bool,
    prev_url: String,
    next_url: String,
}

#[derive(Debug, Default, Clone, Deserialize)]
struct OwnerQuery {
    #[serde(default)]
    tab: String,
    #[serde(default)]
    q: String,
    #[serde(default)]
    page: Option<i64>,
}

impl OwnerQuery {
    fn page_url(&self, tab: &str, page: i64) -> String {
        let mut parts = vec![format!("tab={tab}")];
        if !self.q.is_empty() {
            parts.push(format!("q={}", urlencode(&self.q)));
        }
        if page > 0 {
            parts.push(format!("page={page}"));
        }
        format!("/my/opportunities?{}", parts.join("&"))
    }
}

/// The three tab kinds and their `current`/`withdrawn` filters.
const OWNER_TABS: &[(&str, bool, Option<bool>, bool)] = &[
    // (tab, current, withdrawn, editable)
    ("live", true, None, true),
    ("draft", false, Some(true), true),
    ("past", false, Some(false), false),
];

async fn owner_list(
    state: &AppState,
    token: &str,
    tab: &str,
    current: bool,
    withdrawn: Option<bool>,
    editable: bool,
    q: &str,
    page: i64,
    owner_q: &OwnerQuery,
) -> Result<OwnerList, AppError> {
    let api_query = OwnerApiQuery {
        mine: true,
        sort: "alphabetical",
        current,
        withdrawn,
        page,
        per_page: 10,
        text: q,
    };
    let qs = serde_qs::to_string(&api_query).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let resp = state
        .api
        .get(&format!("/api/ui/finder/search?{qs}"), Some(token))
        .await?;
    let results: SearchResults = json_or_err(resp).await?;

    let last_page = results.pagination.last_page;
    let items = results
        .matches
        .into_iter()
        .map(|opp| OwnerItem {
            uid: opp.uid.to_string(),
            slug: opp.slug.clone(),
            card: CardView::from(opp),
        })
        .collect();
    Ok(OwnerList {
        items,
        editable,
        page_index: page,
        last_page,
        has_prev: page > 0,
        has_next: page < last_page,
        prev_url: owner_q.page_url(tab, page - 1),
        next_url: owner_q.page_url(tab, page + 1),
    })
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_opportunities.stpl")]
struct OpportunitiesPage {
    active_tab: String,
    search: String,
    live: OwnerList,
    draft: OwnerList,
    past: OwnerList,
}

#[handler]
pub async fn opportunities(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<OwnerQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };

    let active = match q.tab.as_str() {
        "draft" => "draft",
        "past" => "past",
        _ => "live",
    };
    let page = q.page.unwrap_or(0).max(0);

    // Build each tab; only the active one applies the search text + page.
    let mut lists = std::collections::HashMap::new();
    for &(tab, current, withdrawn, editable) in OWNER_TABS {
        let (tq, tp) = if tab == active { (q.q.as_str(), page) } else { ("", 0) };
        let list = owner_list(
            &state, &token, tab, current, withdrawn, editable, tq, tp, &q,
        )
        .await?;
        lists.insert(tab, list);
    }

    Ok(page_response(
        chrome,
        OpportunitiesPage {
            active_tab: active.to_string(),
            search: q.q.clone(),
            past: lists.remove("past").unwrap(),
            draft: lists.remove("draft").unwrap(),
            live: lists.remove("live").unwrap(),
        },
    )?)
}

/// Small wrapper so the handler reads cleanly.
fn page_response(chrome: Chrome, p: OpportunitiesPage) -> Result<Response, AppError> {
    Ok(page(chrome, "Your Opportunities | Science Near Me", p)?.into_response())
}

/// Trash (un-accept) an opportunity, then return to the listing.
#[derive(Debug, Default, Deserialize)]
struct TrashReturn {
    #[serde(default)]
    tab: String,
}

#[handler]
pub async fn trash_opportunity(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(uid): poem::web::Path<String>,
    Form(ret): Form<TrashReturn>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    // Fetch, mark un-accepted, write back.
    let mut opp: serde_json::Value = state
        .api
        .get_json(&format!("/api/ui/opportunity/{uid}"), Some(&token))
        .await?;
    if let Some(obj) = opp.as_object_mut() {
        obj.insert("accepted".into(), serde_json::json!(false));
    }
    let _ = state
        .api
        .put_json(&format!("/api/ui/opportunity/{uid}"), Some(&token), &opp)
        .await?;
    let tab = if ret.tab.is_empty() { "live" } else { &ret.tab };
    Ok(Redirect::see_other(format!("/my/opportunities?tab={tab}")).into_response())
}

/// Stream the partner's opportunity export as a CSV download. The API returns
/// `{ filename, content }`; we re-serve `content` with a download disposition.
#[derive(Debug, Deserialize)]
struct CsvExport {
    filename: String,
    content: String,
}

#[handler]
pub async fn export_opportunities(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let export: CsvExport = state
        .api
        .get_json("/api/ui/profile/opportunities.csv", Some(&token))
        .await?;
    Ok(Response::builder()
        .content_type("text/csv; charset=utf-8")
        .header(
            "content-disposition",
            format!("attachment; filename=\"{}\"", export.filename),
        )
        .body(export.content))
}

// ---------------------------------------------------------------------------
// /my/organization — Your Partner Organization (owner)
//
// Manages the partner orgs the user belongs to (`/api/ui/organization/all`).
// Settings + contact info are one round-tripped partner object (PUT
// /api/ui/organization/:uid); manager membership is edited by mutating the
// partner's `authorized`/`pending` uid arrays and writing back; invitations
// POST to `/api/ui/organization/:uid/invite`.
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
struct OrgManager {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    phone: Option<String>,
    #[serde(default)]
    mailing: Option<String>,
}

impl OrgManager {
    fn name(&self) -> &str {
        self.name.as_deref().unwrap_or("")
    }
    fn email(&self) -> &str {
        self.email.as_deref().unwrap_or("")
    }
    fn phone(&self) -> &str {
        self.phone.as_deref().unwrap_or("")
    }
    fn mailing(&self) -> &str {
        self.mailing.as_deref().unwrap_or("")
    }
}

#[derive(Debug, Default, Deserialize)]
struct OrgView {
    #[serde(default)]
    uid: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    organization_type: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    image_url: Option<String>,
    #[serde(default)]
    background_color: Option<String>,
    #[serde(default)]
    primary_color: Option<String>,
    #[serde(default)]
    secondary_color: Option<String>,
    #[serde(default)]
    tertiary_color: Option<String>,
    #[serde(default)]
    manager: OrgManager,
    #[serde(default)]
    prime: Option<String>,
}

impl OrgView {
    fn url(&self) -> &str {
        self.url.as_deref().unwrap_or("")
    }
    fn image_url(&self) -> &str {
        self.image_url.as_deref().unwrap_or("")
    }
    fn type_is(&self, code: &str) -> bool {
        self.organization_type == code
    }
    fn color(&self, which: &str) -> &str {
        match which {
            "background" => self.background_color.as_deref(),
            "primary" => self.primary_color.as_deref(),
            "secondary" => self.secondary_color.as_deref(),
            "tertiary" => self.tertiary_color.as_deref(),
            _ => None,
        }
        .unwrap_or("#ffffff")
    }
}

#[derive(Debug, Deserialize)]
struct OrgMember {
    uid: String,
    #[serde(default)]
    username: Option<String>,
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    phone: Option<String>,
}

impl OrgMember {
    fn display_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(f), Some(l)) if !f.is_empty() && !l.is_empty() => format!("{f} {l}"),
            _ => self.username.clone().unwrap_or_default(),
        }
    }
    fn email(&self) -> &str {
        self.email.as_deref().unwrap_or("")
    }
    fn phone(&self) -> &str {
        self.phone.as_deref().unwrap_or("")
    }
}

#[derive(Debug, Deserialize)]
struct OrgOpt {
    uid: String,
    name: String,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_organization.stpl")]
struct OrgPage {
    no_org: bool,
    partners: Vec<OrgOpt>,
    org: OrgView,
    org_types: Vec<(String, String)>,
    managers: Vec<OrgMember>,
    pending: Vec<OrgMember>,
    user_uid: String,
    is_prime: bool,
}

#[derive(Debug, Default, Deserialize)]
struct OrgQuery {
    #[serde(default)]
    partner: String,
}

#[handler]
pub async fn organization(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<OrgQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let user_uid = chrome.uid().unwrap_or_default().to_string();

    let partners: Vec<OrgOpt> = state
        .api
        .get_json("/api/ui/organization/all", Some(&token))
        .await
        .unwrap_or_default();

    if partners.is_empty() {
        return Ok(page(
            chrome,
            "Your Partner Organization | Science Near Me",
            OrgPage {
                no_org: true,
                partners,
                org: OrgView::default(),
                org_types: Vec::new(),
                managers: Vec::new(),
                pending: Vec::new(),
                user_uid,
                is_prime: false,
            },
        )?
        .into_response());
    }

    // The selected partner: the requested one, or the first.
    let selected = partners
        .iter()
        .find(|p| p.uid == q.partner)
        .map(|p| p.uid.clone())
        .unwrap_or_else(|| partners[0].uid.clone());

    let org: OrgView = state
        .api
        .get_json(&format!("/api/ui/organization/{selected}"), Some(&token))
        .await?;
    let org_types = state
        .api
        .get_json::<Vec<(String, String)>>("/api/ui/organization/types", Some(&token))
        .await
        .unwrap_or_default();
    let managers = state
        .api
        .get_json::<Vec<OrgMember>>(
            &format!("/api/ui/organization/{selected}/managers"),
            Some(&token),
        )
        .await
        .unwrap_or_default();
    let pending = state
        .api
        .get_json::<Vec<OrgMember>>(
            &format!("/api/ui/organization/{selected}/pending-managers"),
            Some(&token),
        )
        .await
        .unwrap_or_default();

    let is_prime = org.prime.as_deref() == Some(user_uid.as_str());

    Ok(page(
        chrome,
        "Your Partner Organization | Science Near Me",
        OrgPage {
            no_org: false,
            partners,
            org,
            org_types,
            managers,
            pending,
            user_uid,
            is_prime,
        },
    )?
    .into_response())
}

/// Org settings or contact info. The `section` distinguishes which tab
/// submitted, so each is an independent form that overlays only its own fields.
#[derive(Debug, Default, Deserialize)]
struct OrgSettingsForm {
    #[serde(default)]
    section: String,
    #[serde(default)]
    name: String,
    #[serde(default)]
    organization_type: String,
    #[serde(default)]
    url: String,
    #[serde(default)]
    image_url: String,
    #[serde(default)]
    background_color: String,
    #[serde(default)]
    primary_color: String,
    #[serde(default)]
    secondary_color: String,
    #[serde(default)]
    tertiary_color: String,
    #[serde(default)]
    manager_name: String,
    #[serde(default)]
    manager_email: String,
    #[serde(default)]
    manager_phone: String,
    #[serde(default)]
    manager_mailing: String,
}

#[handler]
pub async fn save_organization(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(uid): poem::web::Path<String>,
    body: String,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let form: OrgSettingsForm = serde_qs::Config::new(5, false)
        .deserialize_str(&body)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    let mut org: serde_json::Value = state
        .api
        .get_json(&format!("/api/ui/organization/{uid}"), Some(&token))
        .await?;
    if let Some(obj) = org.as_object_mut() {
        if form.section == "contact" {
            obj.insert(
                "manager".into(),
                serde_json::json!({
                    "name": empty_to_null(&form.manager_name),
                    "email": empty_to_null(&form.manager_email),
                    "phone": empty_to_null(&form.manager_phone),
                    "mailing": empty_to_null(&form.manager_mailing),
                }),
            );
        } else {
            obj.insert("name".into(), serde_json::json!(form.name));
            obj.insert(
                "organization_type".into(),
                serde_json::json!(empty_or(&form.organization_type, "unspecified")),
            );
            obj.insert("url".into(), empty_to_null(&form.url));
            obj.insert("image_url".into(), empty_to_null(&form.image_url));
            obj.insert("background_color".into(), empty_to_null(&form.background_color));
            obj.insert("primary_color".into(), empty_to_null(&form.primary_color));
            obj.insert("secondary_color".into(), empty_to_null(&form.secondary_color));
            obj.insert("tertiary_color".into(), empty_to_null(&form.tertiary_color));
        }
    }
    put_org(&state, &token, &uid, &org).await?;
    Ok(Redirect::see_other(format!("/my/organization?partner={uid}")).into_response())
}

/// PUT a partner record, surfacing upstream errors.
async fn put_org(
    state: &AppState,
    token: &str,
    uid: &str,
    org: &serde_json::Value,
) -> Result<(), AppError> {
    let resp = state
        .api
        .put_json(&format!("/api/ui/organization/{uid}"), Some(token), org)
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
    Ok(())
}

/// Approve / discard a pending manager, remove an authorized one, or leave the
/// org — all by mutating the partner's uid arrays and writing back.
#[derive(Debug, Default, Deserialize)]
struct MemberAction {
    #[serde(default)]
    action: String,
    #[serde(default)]
    target: String,
}

#[handler]
pub async fn manage_org_member(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(uid): poem::web::Path<String>,
    Form(form): Form<MemberAction>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let user_uid = chrome.uid().unwrap_or_default().to_string();

    let mut org: serde_json::Value = state
        .api
        .get_json(&format!("/api/ui/organization/{uid}"), Some(&token))
        .await?;

    // Helpers to add/remove a uid in a string array field.
    fn retain(org: &mut serde_json::Value, field: &str, drop: &str) {
        if let Some(arr) = org.get_mut(field).and_then(|v| v.as_array_mut()) {
            arr.retain(|x| x.as_str() != Some(drop));
        }
    }
    fn push(org: &mut serde_json::Value, field: &str, add: &str) {
        if let Some(arr) = org.get_mut(field).and_then(|v| v.as_array_mut()) {
            if !arr.iter().any(|x| x.as_str() == Some(add)) {
                arr.push(serde_json::json!(add));
            }
        }
    }

    let leaving = form.action == "leave";
    match form.action.as_str() {
        "approve" => {
            retain(&mut org, "pending", &form.target);
            push(&mut org, "authorized", &form.target);
        }
        "discard" => retain(&mut org, "pending", &form.target),
        "remove" => retain(&mut org, "authorized", &form.target),
        "leave" => retain(&mut org, "authorized", &user_uid),
        _ => return Ok(Redirect::see_other(format!("/my/organization?partner={uid}")).into_response()),
    }

    put_org(&state, &token, &uid, &org).await?;

    let dest = if leaving {
        "/my/profile".to_string()
    } else {
        format!("/my/organization?partner={uid}")
    };
    Ok(Redirect::see_other(dest).into_response())
}

#[derive(Debug, Default, Deserialize)]
struct InviteForm {
    #[serde(default)]
    emails: String,
}

#[handler]
pub async fn invite_managers(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    poem::web::Path(uid): poem::web::Path<String>,
    Form(form): Form<InviteForm>,
) -> Result<Response, AppError> {
    let token = match require_user(&state, jar, req).await {
        Ok((_, token)) => token,
        Err(redirect) => return Ok(redirect),
    };
    let emails: Vec<String> = form
        .emails
        .split(|c: char| c.is_whitespace() || c == ',')
        .map(|e| e.trim())
        .filter(|e| !e.is_empty())
        .map(|e| e.to_string())
        .collect();
    if !emails.is_empty() {
        let _ = state
            .api
            .post_json(
                &format!("/api/ui/organization/{uid}/invite"),
                Some(&token),
                &serde_json::json!({ "emails": emails }),
            )
            .await?;
    }
    Ok(Redirect::see_other(format!("/my/organization?partner={uid}")).into_response())
}
