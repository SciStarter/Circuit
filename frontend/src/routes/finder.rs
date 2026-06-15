use poem::web::cookie::CookieJar;
use poem::web::Data;
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::{Deserialize, Serialize};

use common::model::OpportunityExterior;

use crate::api::json_or_err;
use crate::chrome::Chrome;
use crate::error::AppError;
use crate::iplookup;
use crate::opportunity::CardView;
use crate::render::{page, Render};
use crate::session::token_from_jar;
use crate::AppState;

/// Search response from `GET /api/ui/finder/search`.
#[derive(Debug, Deserialize)]
struct SearchResults {
    pagination: PaginationInfo,
    matches: Vec<OpportunityExterior>,
}

/// Response from the backend geolocate endpoint (`/api/ui/finder/geolocate`).
#[derive(Debug, Deserialize)]
struct GeolocateResponse {
    longitude: f32,
    latitude: f32,
    #[serde(default)]
    near: String,
}

/// One option for the Partner Organization filter (`/api/ui/finder/partners`).
#[derive(Debug, Deserialize)]
struct PartnerOption {
    name: String,
    uid: String,
}

#[derive(Debug, Deserialize)]
struct PaginationInfo {
    page_index: i64,
    #[allow(dead_code)]
    per_page: i64,
    last_page: i64,
    total: i64,
}

/// The finder query. Parsed from and serialized back to the query string with
/// `serde_qs` (the same library the API uses to parse it), so it is the single
/// typed representation used both to call the API and to build links. Empty
/// fields are omitted from the serialized query.
#[derive(Debug, Default, Clone, Deserialize, Serialize)]
struct FindParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    near: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    longitude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    latitude: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    proximity: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    physical: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    temporal: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    beginning: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    ending: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    host: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    partner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    cost: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    venue_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    min_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_age: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    kids_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    adults_only: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    topics: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    descriptors: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    page: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    per_page: Option<i64>,
}

impl FindParams {
    /// Drop blank string fields so they don't clutter serialized queries,
    /// produce invalid empty numeric params, or override API defaults.
    fn normalized(mut self) -> FindParams {
        for field in [
            &mut self.text,
            &mut self.near,
            &mut self.longitude,
            &mut self.latitude,
            &mut self.proximity,
            &mut self.physical,
            &mut self.temporal,
            &mut self.beginning,
            &mut self.ending,
            &mut self.sort,
            &mut self.host,
            &mut self.partner,
            &mut self.cost,
            &mut self.venue_type,
            &mut self.min_age,
            &mut self.max_age,
        ] {
            if field.as_deref().map(str::is_empty).unwrap_or(false) {
                *field = None;
            }
        }
        self
    }

    /// Params adjusted for the API call: the `From`/`Until` date inputs
    /// (`YYYY-MM-DD`) are widened to the RFC3339 datetimes the API's
    /// `beginning`/`ending` (DateTime<FixedOffset>) require.
    fn for_api(&self) -> FindParams {
        fn widen(date: &Option<String>, time: &str) -> Option<String> {
            date.as_deref().map(|d| {
                if d.len() == 10 && d.as_bytes()[4] == b'-' {
                    format!("{d}T{time}+00:00")
                } else {
                    d.to_string()
                }
            })
        }
        let mut p = self.clone();
        p.beginning = widen(&self.beginning, "00:00:00");
        p.ending = widen(&self.ending, "23:59:59");
        p
    }

    /// `/find` URL for a given page, derived from the current query.
    fn page_url(&self, page: i64) -> String {
        let mut p = self.clone();
        p.page = Some(page);
        match serde_qs::to_string(&p) {
            Ok(qs) if !qs.is_empty() => format!("/find?{qs}"),
            _ => "/find".to_string(),
        }
    }

    /// Whether any filter (other than sort) is active — drives the visibility
    /// of "Clear Filters".
    fn has_any_filter(&self) -> bool {
        self.text.is_some()
            || self.near.is_some()
            || self.proximity.is_some()
            || self.physical.is_some()
            || self.temporal.is_some()
            || self.beginning.is_some()
            || self.ending.is_some()
            || self.host.is_some()
            || self.partner.is_some()
            || self.cost.is_some()
            || self.venue_type.is_some()
            || self.min_age.is_some()
            || self.max_age.is_some()
            || self.kids_only.is_some()
            || self.adults_only.is_some()
            || !self.topics.is_empty()
            || !self.descriptors.is_empty()
    }

    fn has_topic(&self, value: &str) -> bool {
        self.topics.iter().any(|t| t == value)
    }

    fn has_descriptor(&self, value: &str) -> bool {
        self.descriptors.iter().any(|d| d == value)
    }
}

#[derive(TemplateSimple)]
#[template(path = "partials/find_results.stpl")]
struct FindResultsView {
    cards: Vec<CardView>,
    total: i64,
    page_human: i64,
    last_human: i64,
    has_prev: bool,
    has_next: bool,
    prev_url: String,
    next_url: String,
    /// When true (HTMX requests), also emit the result-count line as an
    /// out-of-band swap so it updates above the quick-bar.
    oob: bool,
}

#[derive(TemplateSimple)]
#[template(path = "pages/find.stpl")]
struct FindPage {
    params: FindParams,
    topics_all: Vec<(String, String)>,
    descriptors_all: Vec<(String, String)>,
    partners_all: Vec<PartnerOption>,
    results_html: String,
    total: i64,
}

#[handler]
pub async fn find(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let is_hx = req.header("HX-Request").is_some();

    // Non-strict mode so percent-encoded array brackets (`topics%5B%5D=…`, as
    // browsers submit `name="topics[]"` form fields) are parsed as arrays.
    let raw_qs = req.uri().query().unwrap_or("");
    let params: FindParams = serde_qs::Config::new(5, false)
        .deserialize_str(raw_qs)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;
    let mut params = params.normalized();

    // Fresh landing (no query at all): default to 50 miles around the user's
    // IP-geolocated location, via the backend geolocate endpoint (which caches
    // in c_ip_coords). Any explicit criteria (a non-empty query, e.g. a
    // submitted form or "Distance: Anywhere") take precedence.
    if raw_qs.is_empty() {
        let path = match iplookup::client_ip(req) {
            Some(ip) => format!("/api/ui/finder/geolocate?ip={ip}"),
            None => "/api/ui/finder/geolocate".to_string(),
        };
        if let Ok(geo) = state.api.get_json::<GeolocateResponse>(&path, None).await {
            params.longitude = Some(geo.longitude.to_string());
            params.latitude = Some(geo.latitude.to_string());
            params.proximity = Some("80467".to_string()); // ~50 miles
            if !geo.near.is_empty() {
                params.near = Some(geo.near);
            }
        }
    }

    let api_qs =
        serde_qs::to_string(&params.for_api()).map_err(|e| AppError::BadRequest(e.to_string()))?;
    let api_path = if api_qs.is_empty() {
        "/api/ui/finder/search".to_string()
    } else {
        format!("/api/ui/finder/search?{api_qs}")
    };

    let resp = state.api.get(&api_path, token.as_deref()).await?;
    let results: SearchResults = json_or_err(resp).await?;

    let page_index = results.pagination.page_index;
    let last_page = results.pagination.last_page;
    let total = results.pagination.total;

    let view = FindResultsView {
        cards: results.matches.into_iter().map(CardView::from).collect(),
        total,
        page_human: page_index + 1,
        last_human: last_page + 1,
        has_prev: page_index > 0,
        has_next: page_index < last_page,
        prev_url: params.page_url(page_index - 1),
        next_url: params.page_url(page_index + 1),
        oob: is_hx,
    };

    // HTMX requests swap the results region and OOB-update the count line.
    if is_hx {
        return Ok(Render(view).into_response());
    }

    // Full page: also build the chrome and the facet option lists.
    let chrome = Chrome::build(
        &state.api,
        token.as_deref(),
        req.header("host"),
        req.uri().path().to_string(),
        state.config.domain.clone(),
    )
    .await;

    let topics_all = state
        .api
        .get_json::<Vec<(String, String)>>("/api/ui/finder/topics", None)
        .await
        .unwrap_or_default();
    let descriptors_all = state
        .api
        .get_json::<Vec<(String, String)>>("/api/ui/finder/descriptors", None)
        .await
        .unwrap_or_default();
    let partners_all = state
        .api
        .get_json::<Vec<PartnerOption>>("/api/ui/finder/partners", None)
        .await
        .unwrap_or_default();

    let results_html = view.render_once()?;

    Ok(page(
        chrome,
        "Find Opportunities | Science Near Me",
        FindPage {
            params,
            topics_all,
            descriptors_all,
            partners_all,
            results_html,
            total,
        },
    )?
    .into_response())
}
