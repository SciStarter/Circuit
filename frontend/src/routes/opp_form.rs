//! Add / edit opportunity form (`/my/submit-opportunity`, `/my/opportunity/:uid`).
//!
//! The opportunity is round-tripped through `GET`/`POST`/`PUT
//! /api/ui/opportunity[/:uid]`: we fetch the canonical record (a blank template
//! for new opportunities, the stored one for edits) as an untyped `Value`,
//! overlay the submitted fields onto it, and write it back — so the many fields
//! the form doesn't surface (extra_data, partner bookkeeping, …) survive intact.
//! A typed `OppView` of the same record drives rendering.
//!
//! Per the agreed design this is a single page with CSS section tabs (Basic /
//! Required / Additional) and one set of save actions; physical-location
//! geometry is captured as a point via an address typeahead island.

use poem::web::cookie::CookieJar;
use poem::web::{Data, Path, Redirect};
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::chrome::Chrome;
use crate::error::AppError;
use crate::render::page;
use crate::routes::my::require_user;
use crate::AppState;

/// The eight public-engagement-of-science domains (code, label).
const PES_DOMAINS: &[(&str, &str)] = &[
    ("citizen_science", "Citizen Science"),
    ("live_science", "Live Science"),
    ("museum_or_science_center", "Museum or Science Center"),
    ("maker", "Maker"),
    ("policy", "Science Policy"),
    ("out_of_school_time_program", "Out of School Time Program"),
    ("formal_education", "Formal Education"),
    ("science_communications", "Science Communications"),
];

/// Popular keyword suggestions (matching the old form's `mostUsed`).
const MOST_USED_TAGS: &[&str] = &[
    "museum",
    "astronomy",
    "afterschool",
    "library",
    "kids",
    "citizen science",
    "nature",
];

#[derive(Debug, Default, Deserialize)]
struct SocialHandles {
    #[serde(default)]
    twitter: Option<String>,
    #[serde(default)]
    instagram: Option<String>,
    #[serde(default)]
    facebook: Option<String>,
}

impl SocialHandles {
    fn twitter(&self) -> &str {
        self.twitter.as_deref().unwrap_or("")
    }
    fn instagram(&self) -> &str {
        self.instagram.as_deref().unwrap_or("")
    }
    fn facebook(&self) -> &str {
        self.facebook.as_deref().unwrap_or("")
    }
}

/// A typed projection of the opportunity for rendering the form. Only the
/// fields the form edits are read; everything else stays on the round-tripped
/// `Value`.
#[derive(Debug, Deserialize)]
struct OppView {
    #[serde(default)]
    uid: String,
    #[serde(default)]
    slug: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    organization_name: String,
    #[serde(default)]
    contact_name: String,
    #[serde(default)]
    contact_email: String,
    #[serde(default)]
    contact_phone: String,
    #[serde(default)]
    partner: String,
    #[serde(default)]
    is_online: bool,
    #[serde(default)]
    location_type: String,
    #[serde(default)]
    location_name: String,
    #[serde(default)]
    partner_opp_url: Option<String>,
    #[serde(default)]
    organization_website: Option<String>,
    #[serde(default)]
    timezone: Option<String>,
    #[serde(default)]
    start_datetimes: Vec<String>,
    #[serde(default)]
    end_datetimes: Vec<String>,
    #[serde(default)]
    has_end: bool,
    #[serde(default)]
    recurrence: String,
    #[serde(default)]
    end_recurrence: Option<String>,
    #[serde(default)]
    short_desc: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    image_url: String,
    #[serde(default)]
    pes_domain: String,
    #[serde(default)]
    opp_descriptor: Vec<String>,
    #[serde(default)]
    cost: String,
    #[serde(default)]
    min_age: i64,
    #[serde(default)]
    max_age: i64,
    #[serde(default)]
    opp_venue: Vec<String>,
    #[serde(default)]
    opp_topics: Vec<String>,
    #[serde(default)]
    ticket_required: bool,
    #[serde(default)]
    tags: Vec<String>,
    #[serde(default)]
    opp_hashtags: Vec<String>,
    #[serde(default)]
    opp_social_handles: SocialHandles,
    #[serde(default)]
    location_point: Option<Value>,
}

impl OppView {
    /// "online" | "physical" | "both" | "" — mirrors the old computed `location`.
    fn location_mode(&self) -> &str {
        let physical = self.location_type == "at" || self.location_type == "near";
        match (self.is_online, physical) {
            (true, true) => "both",
            (true, false) => "online",
            (false, true) => "physical",
            (false, false) => "",
        }
    }

    /// "time" when explicit start times exist, else "ongoing".
    fn when_mode(&self) -> &str {
        if self.start_datetimes.is_empty() {
            "ongoing"
        } else {
            "time"
        }
    }

    fn timezone(&self) -> &str {
        self.timezone.as_deref().unwrap_or("")
    }
    fn partner_opp_url(&self) -> &str {
        self.partner_opp_url.as_deref().unwrap_or("")
    }
    fn organization_website(&self) -> &str {
        self.organization_website.as_deref().unwrap_or("")
    }
    fn end_recurrence_date(&self) -> &str {
        // Stored RFC3339; the date input wants YYYY-MM-DD.
        self.end_recurrence
            .as_deref()
            .map(|s| if s.len() >= 10 { &s[..10] } else { s })
            .unwrap_or("")
    }

    fn has_descriptor(&self, code: &str) -> bool {
        self.opp_descriptor.iter().any(|d| d == code)
    }
    fn has_topic(&self, code: &str) -> bool {
        self.opp_topics.iter().any(|t| t == code)
    }
    fn has_venue(&self, code: &str) -> bool {
        self.opp_venue.iter().any(|v| v == code)
    }
    fn domain_is(&self, code: &str) -> bool {
        self.pes_domain == code
    }
    fn has_minimum(&self) -> bool {
        self.min_age > 0
    }
    fn has_maximum(&self) -> bool {
        self.max_age < 999
    }

    fn tags_csv(&self) -> String {
        self.tags.join(", ")
    }
    fn hashtags_csv(&self) -> String {
        self.opp_hashtags.join(", ")
    }

    fn point_lng(&self) -> String {
        self.coord(0)
    }
    fn point_lat(&self) -> String {
        self.coord(1)
    }
    fn coord(&self, i: usize) -> String {
        self.location_point
            .as_ref()
            .and_then(|p| p.get("coordinates"))
            .and_then(|c| c.get(i))
            .and_then(|n| n.as_f64())
            .map(|n| n.to_string())
            .unwrap_or_default()
    }

    /// (start, end) pairs as `YYYY-MM-DDTHH:MM` local strings for the
    /// datetime-local inputs of the "set time" mode.
    fn periods(&self) -> Vec<(String, String)> {
        let n = self.start_datetimes.len().max(self.end_datetimes.len());
        (0..n)
            .map(|i| {
                (
                    local_dt(self.start_datetimes.get(i)),
                    local_dt(self.end_datetimes.get(i)),
                )
            })
            .collect()
    }

    /// The ongoing-mode end date (`YYYY-MM-DD`) if one is set.
    fn ongoing_end(&self) -> String {
        if self.has_end {
            self.end_datetimes
                .first()
                .map(|s| if s.len() >= 10 { s[..10].to_string() } else { s.clone() })
                .unwrap_or_default()
        } else {
            String::new()
        }
    }
}

/// RFC3339 → `YYYY-MM-DDTHH:MM` (the value form of `<input type=datetime-local>`).
fn local_dt(s: Option<&String>) -> String {
    s.map(|s| if s.len() >= 16 { s[..16].to_string() } else { s.clone() })
        .unwrap_or_default()
}

/// A partner the user may file an opportunity under.
#[derive(Debug, Deserialize, Clone)]
struct PartnerOpt {
    uid: String,
    name: String,
    #[serde(default)]
    url: Option<String>,
    #[serde(default)]
    image_url: Option<String>,
}

#[derive(TemplateSimple)]
#[template(path = "pages/opportunity_form.stpl")]
struct FormPage {
    edit_mode: bool,
    opp: OppView,
    partners: Vec<PartnerOpt>,
    descriptors: Vec<(String, String)>,
    topics: Vec<(String, String)>,
    timezones: Vec<String>,
    pes_domains: Vec<(String, String)>,
    most_used: Vec<String>,
}

/// Build and render the form for the given opportunity `Value`.
async fn render_form(
    state: &AppState,
    chrome: Chrome,
    token: &str,
    edit_mode: bool,
    opp_value: Value,
) -> Result<Response, AppError> {
    let opp: OppView = serde_json::from_value(opp_value)
        .map_err(|e| AppError::BadRequest(format!("opportunity shape: {e}")))?;

    let descriptors = state
        .api
        .get_json::<Vec<(String, String)>>("/api/ui/finder/descriptors", None)
        .await
        .unwrap_or_default();
    let topics = state
        .api
        .get_json::<Vec<(String, String)>>("/api/ui/finder/topics", None)
        .await
        .unwrap_or_default();
    let timezones = state
        .api
        .get_json::<Vec<String>>("/api/ui/timezone", Some(token))
        .await
        .unwrap_or_default();
    let partners = state
        .api
        .get_json::<Vec<PartnerOpt>>("/api/ui/profile/partners", Some(token))
        .await
        .unwrap_or_default();

    let title = if edit_mode {
        "Edit Opportunity | Science Near Me"
    } else {
        "Add an Opportunity | Science Near Me"
    };

    Ok(page(
        chrome,
        title,
        FormPage {
            edit_mode,
            opp,
            partners,
            descriptors,
            topics,
            timezones,
            pes_domains: PES_DOMAINS
                .iter()
                .map(|(c, l)| (c.to_string(), l.to_string()))
                .collect(),
            most_used: MOST_USED_TAGS.iter().map(|s| s.to_string()).collect(),
        },
    )?
    .into_response())
}

#[handler]
pub async fn new_opportunity(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let blank: Value = state
        .api
        .get_json("/api/ui/opportunity/", Some(&token))
        .await?;
    render_form(&state, chrome, &token, false, blank).await
}

#[handler]
pub async fn edit_opportunity(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Path(uid): Path<String>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    let opp: Value = state
        .api
        .get_json(&format!("/api/ui/opportunity/{uid}"), Some(&token))
        .await?;
    render_form(&state, chrome, &token, true, opp).await
}

// ---------------------------------------------------------------------------
// Saving
// ---------------------------------------------------------------------------

/// The submitted form. Array fields use `name="x[]"`, so the body is parsed with
/// `serde_qs` (non-strict). Checkbox booleans are `Option<String>` (present ==
/// checked); the action button is `action`.
#[derive(Debug, Default, Deserialize)]
struct SaveForm {
    #[serde(default)]
    action: String,

    #[serde(default)]
    title: String,
    #[serde(default)]
    organization_name: String,
    #[serde(default)]
    contact_name: String,
    #[serde(default)]
    contact_email: String,
    #[serde(default)]
    contact_phone: String,
    #[serde(default)]
    partner: String,

    #[serde(default)]
    location_mode: String,
    #[serde(default)]
    partner_opp_url: String,
    #[serde(default)]
    location_name: String,
    #[serde(default)]
    point_lng: String,
    #[serde(default)]
    point_lat: String,

    #[serde(default)]
    when_mode: String,
    #[serde(default)]
    timezone: String,
    #[serde(default)]
    ongoing_has_end: Option<String>,
    #[serde(default)]
    ongoing_end: String,
    #[serde(default)]
    start_dt: Vec<String>,
    #[serde(default)]
    end_dt: Vec<String>,
    #[serde(default)]
    recurrence: String,
    #[serde(default)]
    end_recurrence: String,

    #[serde(default)]
    organization_website: String,

    #[serde(default)]
    short_desc: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    image_url: String,
    #[serde(default)]
    pes_domain: String,
    #[serde(default)]
    opp_descriptor: Vec<String>,
    #[serde(default)]
    cost: String,
    #[serde(default)]
    has_minimum: Option<String>,
    #[serde(default)]
    min_age: Option<i64>,
    #[serde(default)]
    has_maximum: Option<String>,
    #[serde(default)]
    max_age: Option<i64>,
    #[serde(default)]
    opp_venue: Vec<String>,
    #[serde(default)]
    opp_topics: Vec<String>,
    #[serde(default)]
    ticket_required: Option<String>,
    #[serde(default)]
    tags: String,

    #[serde(default)]
    opp_hashtags: String,
    #[serde(default)]
    social_twitter: String,
    #[serde(default)]
    social_instagram: String,
    #[serde(default)]
    social_facebook: String,
}

/// Split a comma-separated tag field into trimmed, non-empty entries.
fn split_tags(s: &str) -> Vec<String> {
    s.split(',')
        .map(|t| t.trim())
        .filter(|t| !t.is_empty())
        .map(|t| t.to_string())
        .collect()
}

/// The timezone offset (e.g. "-07:00") for `tz` on `date` (YYYY-MM-DD), via the
/// backend. Falls back to UTC when no timezone is selected or lookup fails.
async fn offset_for(state: &AppState, token: &str, tz: &str, date: &str) -> String {
    #[derive(Deserialize)]
    struct Zone {
        offset: String,
    }
    if tz.is_empty() || date.len() < 10 {
        return "+00:00".to_string();
    }
    let path = format!(
        "/api/ui/timezone?name={}&date={}",
        crate::routes::my::urlencode(tz),
        &date[..10]
    );
    match state.api.get_json::<Zone>(&path, Some(token)).await {
        Ok(z) => z.offset,
        Err(_) => "+00:00".to_string(),
    }
}

/// Turn a `datetime-local` value (`YYYY-MM-DDTHH:MM`) into an RFC3339 string with
/// the timezone offset for its date.
async fn build_datetime(state: &AppState, token: &str, tz: &str, local: &str) -> Option<String> {
    let local = local.trim();
    if local.len() < 16 {
        return None;
    }
    let offset = offset_for(state, token, tz, &local[..10]).await;
    Some(format!("{}:00{}", &local[..16], offset))
}

#[handler]
pub async fn save_new(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    body: String,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    save(&state, &chrome, &token, None, &body).await
}

#[handler]
pub async fn save_existing(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Path(uid): Path<String>,
    body: String,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };
    save(&state, &chrome, &token, Some(uid), &body).await
}

/// Shared save: merge the submitted form onto the canonical record and write it
/// back (POST to create, PUT to update), then redirect per the chosen action.
async fn save(
    state: &AppState,
    _chrome: &Chrome,
    token: &str,
    uid: Option<String>,
    body: &str,
) -> Result<Response, AppError> {
    let form: SaveForm = serde_qs::Config::new(5, false)
        .deserialize_str(body)
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Base record: the existing opportunity (edit) or a fresh blank (create),
    // so unmanaged fields are preserved.
    let base_path = match &uid {
        Some(uid) => format!("/api/ui/opportunity/{uid}"),
        None => "/api/ui/opportunity/".to_string(),
    };
    let mut opp: Value = state.api.get_json(&base_path, Some(token)).await?;
    let prior_location_type = opp
        .get("location_type")
        .and_then(|v| v.as_str())
        .unwrap_or("near")
        .to_string();

    let obj = opp
        .as_object_mut()
        .ok_or_else(|| AppError::BadRequest("opportunity is not an object".into()))?;

    // --- Basic information ---
    obj.insert("title".into(), json!(form.title));
    obj.insert("organization_name".into(), json!(form.organization_name));
    obj.insert("contact_name".into(), json!(form.contact_name));
    obj.insert("contact_email".into(), json!(form.contact_email));
    obj.insert("contact_phone".into(), json!(form.contact_phone));

    // Partner: copy the chosen partner's display fields too.
    if !form.partner.is_empty() {
        obj.insert("partner".into(), json!(form.partner));
        if let Ok(partners) = state
            .api
            .get_json::<Vec<PartnerOpt>>("/api/ui/profile/partners", Some(token))
            .await
        {
            if let Some(p) = partners.into_iter().find(|p| p.uid == form.partner) {
                obj.insert("partner_name".into(), json!(p.name));
                obj.insert("partner_url".into(), json!(p.url));
                obj.insert("partner_logo_url".into(), json!(p.image_url));
            }
        }
    }

    // --- Location ---
    match form.location_mode.as_str() {
        "online" => {
            obj.insert("is_online".into(), json!(true));
            obj.insert("location_type".into(), json!("any"));
            obj.insert("location_name".into(), json!(""));
            obj.insert("location_point".into(), Value::Null);
            obj.insert("location_polygon".into(), Value::Null);
            obj.insert("partner_opp_url".into(), json!(form.partner_opp_url));
        }
        "physical" | "both" => {
            obj.insert("is_online".into(), json!(form.location_mode == "both"));
            // Keep an existing precise "at"; otherwise proximity "near".
            let lt = if prior_location_type == "at" { "at" } else { "near" };
            obj.insert("location_type".into(), json!(lt));
            obj.insert("location_name".into(), json!(form.location_name));
            if let (Ok(lng), Ok(lat)) =
                (form.point_lng.trim().parse::<f64>(), form.point_lat.trim().parse::<f64>())
            {
                obj.insert(
                    "location_point".into(),
                    json!({ "type": "Point", "coordinates": [lng, lat] }),
                );
                obj.insert("location_polygon".into(), Value::Null);
            }
            if form.location_mode == "both" {
                obj.insert("partner_opp_url".into(), json!(form.partner_opp_url));
            }
        }
        _ => {}
    }

    // --- Timing ---
    obj.insert("timezone".into(), json!(empty_to_null(&form.timezone)));
    if form.when_mode == "time" {
        let mut starts = Vec::new();
        let mut ends = Vec::new();
        let n = form.start_dt.len().max(form.end_dt.len());
        for i in 0..n {
            let s = form.start_dt.get(i).map(String::as_str).unwrap_or("");
            let e = form.end_dt.get(i).map(String::as_str).unwrap_or("");
            if let Some(start) = build_datetime(state, token, &form.timezone, s).await {
                // Only keep a period that has a start; the end is optional.
                let end = build_datetime(state, token, &form.timezone, e).await;
                starts.push(json!(start));
                ends.push(json!(end.unwrap_or_else(|| start.clone())));
            }
        }
        obj.insert("has_end".into(), json!(!ends.is_empty()));
        obj.insert("start_datetimes".into(), json!(starts));
        obj.insert("end_datetimes".into(), json!(ends));
        obj.insert("recurrence".into(), json!(empty_or(&form.recurrence, "once")));
        obj.insert(
            "end_recurrence".into(),
            match build_datetime(state, token, &form.timezone, &format!("{}T00:00", form.end_recurrence)).await {
                Some(dt) if !form.end_recurrence.is_empty() => json!(dt),
                _ => Value::Null,
            },
        );
    } else {
        // Ongoing: no start times; an optional end date.
        obj.insert("start_datetimes".into(), json!([] as [Value; 0]));
        obj.insert("recurrence".into(), json!("once"));
        if form.ongoing_has_end.is_some() && !form.ongoing_end.is_empty() {
            let end = build_datetime(
                state,
                token,
                &form.timezone,
                &format!("{}T23:00", form.ongoing_end),
            )
            .await;
            obj.insert("has_end".into(), json!(true));
            obj.insert("end_datetimes".into(), json!([end]));
        } else {
            obj.insert("has_end".into(), json!(false));
            obj.insert("end_datetimes".into(), json!([] as [Value; 0]));
        }
    }

    // --- Learn more ---
    obj.insert(
        "organization_website".into(),
        json!(empty_to_null(&form.organization_website)),
    );

    // --- Required fields ---
    obj.insert("short_desc".into(), json!(form.short_desc));
    obj.insert("description".into(), json!(form.description));
    obj.insert("image_url".into(), json!(form.image_url));
    obj.insert("pes_domain".into(), json!(empty_or(&form.pes_domain, "unspecified")));
    obj.insert("opp_descriptor".into(), json!(form.opp_descriptor));
    obj.insert("cost".into(), json!(empty_or(&form.cost, "free")));
    obj.insert(
        "min_age".into(),
        json!(if form.has_minimum.is_some() { form.min_age.unwrap_or(0).max(0) } else { 0 }),
    );
    obj.insert(
        "max_age".into(),
        json!(if form.has_maximum.is_some() { form.max_age.unwrap_or(999) } else { 999 }),
    );
    obj.insert("opp_venue".into(), json!(form.opp_venue));
    obj.insert("opp_topics".into(), json!(form.opp_topics));
    obj.insert("ticket_required".into(), json!(form.ticket_required.is_some()));
    obj.insert("tags".into(), json!(split_tags(&form.tags)));

    // --- Additional ---
    obj.insert("opp_hashtags".into(), json!(split_tags(&form.opp_hashtags)));
    // The model's social handles are a string map; include only the handles
    // that were actually provided (a null value would be rejected).
    let mut handles = serde_json::Map::new();
    for (key, val) in [
        ("twitter", &form.social_twitter),
        ("instagram", &form.social_instagram),
        ("facebook", &form.social_facebook),
    ] {
        let v = val.trim();
        if !v.is_empty() {
            handles.insert(key.to_string(), json!(v));
        }
    }
    obj.insert("opp_social_handles".into(), Value::Object(handles));

    // --- Publication intent ---
    if form.action == "publish" {
        obj.insert("withdrawn".into(), json!(false));
        if obj.get("review_status").and_then(|v| v.as_str()) == Some("draft") {
            obj.insert("review_status".into(), json!("pending"));
        }
    }

    // Write back: PUT for an existing opportunity, POST to create a new one.
    let resp = match &uid {
        Some(uid) => {
            state
                .api
                .put_json(&format!("/api/ui/opportunity/{uid}"), Some(token), &opp)
                .await?
        }
        None => state.api.post_json("/api/ui/opportunity/", Some(token), &opp).await?,
    };
    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::ApiStatus {
            status: poem::http::StatusCode::from_u16(status.as_u16())
                .unwrap_or(poem::http::StatusCode::BAD_GATEWAY),
            body,
        });
    }
    let saved: Value = resp.json().await.map_err(AppError::from)?;
    let saved_uid = saved.get("uid").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let saved_slug = saved.get("slug").and_then(|v| v.as_str()).unwrap_or("").to_string();
    let review = saved.get("review_status").and_then(|v| v.as_str()).unwrap_or("");

    // Redirect per action.
    let dest = match form.action.as_str() {
        // Stay in the editor.
        "continue" => format!("/my/opportunity/{saved_uid}"),
        // View the public page (or fall back to the editor if no slug yet).
        "view" => {
            if saved_slug.is_empty() {
                format!("/my/opportunity/{saved_uid}")
            } else {
                format!("/{saved_slug}")
            }
        }
        // Publish: pending review goes to the listing; already-publishable to the page.
        "publish" => {
            if review == "pending" || saved_slug.is_empty() {
                "/my/opportunities".to_string()
            } else {
                format!("/{saved_slug}")
            }
        }
        _ => format!("/my/opportunity/{saved_uid}"),
    };
    Ok(Redirect::see_other(dest).into_response())
}

/// `""` → JSON null, otherwise the string.
pub(crate) fn empty_to_null(s: &str) -> Value {
    if s.trim().is_empty() {
        Value::Null
    } else {
        json!(s)
    }
}

/// `value` if non-empty, else `fallback`.
pub(crate) fn empty_or(value: &str, fallback: &str) -> String {
    if value.trim().is_empty() {
        fallback.to_string()
    } else {
        value.to_string()
    }
}
