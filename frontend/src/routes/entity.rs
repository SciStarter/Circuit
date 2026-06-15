use poem::web::cookie::CookieJar;
use poem::web::{Data, Form, Path, Redirect};
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::Deserialize;

use chrono::{DateTime, Duration, Utc};

use common::model::opportunity::{
    AnnotatedOpportunityExterior, LocationType, OpportunityExterior, ReviewStatus,
};
use common::model::person::PermitAction;

use crate::chrome::Chrome;
use crate::error::AppError;
use crate::render::{page, Render};
use crate::session::token_from_jar;
use crate::{markdown, opportunity, AppState};

/// Current viewer's involvement flags from `GET /api/ui/entity/:slug/me`.
#[derive(Debug, Default, Deserialize)]
struct MeFlags {
    #[serde(default)]
    like: bool,
    #[serde(default)]
    save: bool,
    #[serde(default)]
    didit: bool,
}

// --- Reviews (the `common` Review type is Serialize-only, so deserialize locally) ---

#[derive(Debug, Default, Deserialize)]
struct ReviewsResp {
    #[serde(default)]
    average: Option<f32>,
    #[serde(default)]
    reviews: Vec<ReviewItem>,
}

#[derive(Debug, Deserialize)]
struct ReviewItem {
    id: i32,
    #[serde(default)]
    username: Option<String>,
    rating: i16,
    comment: String,
    when: DateTime<Utc>,
}

/// The reviews section (list + add-review form), an HTMX island re-rendered when
/// a review is added.
#[derive(TemplateSimple)]
#[template(path = "partials/reviews.stpl")]
struct ReviewsView {
    slug: String,
    authenticated: bool,
    average: Option<f32>,
    reviews: Vec<ReviewRow>,
}

struct ReviewRow {
    id: i32,
    username: String,
    when_label: String,
    rating: i16,
    comment_html: String,
}

/// Fetch and build the reviews section for `slug`.
async fn reviews_section(state: &AppState, token: Option<&str>, slug: &str) -> ReviewsView {
    let resp: ReviewsResp = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}/reviews"), token)
        .await
        .unwrap_or_default();
    let reviews = resp
        .reviews
        .into_iter()
        .map(|r| ReviewRow {
            id: r.id,
            username: r.username.unwrap_or_else(|| "Anonymous".to_string()),
            when_label: r.when.format("%b %-d, %Y").to_string(),
            rating: r.rating,
            comment_html: markdown::to_html(&r.comment),
        })
        .collect();
    ReviewsView {
        slug: slug.to_string(),
        authenticated: token.is_some(),
        average: resp.average.filter(|a| a.is_finite()),
        reviews,
    }
}

/// The interactive action bar (Save / Like / Share / Visit Website / I Did This),
/// rendered into the page and re-rendered by the toggle handlers for HTMX swaps.
#[derive(TemplateSimple)]
#[template(path = "partials/opp_actions.stpl")]
struct OppActionsView {
    slug: String,
    authenticated: bool,
    did_save: bool,
    did_like: bool,
    did_didit: bool,
    likes: i64,
    weblink: Option<String>,
    public_url: String,
    twitter_url: String,
    facebook_url: String,
    linkedin_url: String,
    /// Upcoming dates with add-to-calendar links (empty = no calendar button).
    calendar: Vec<CalEntry>,
}

/// One upcoming occurrence with add-to-calendar links for each service.
struct CalEntry {
    label: String,
    google: String,
    outlook: String,
    office365: String,
    yahoo: String,
}

/// Build add-to-calendar links for each upcoming occurrence (UTC, so the links
/// are correct regardless of the viewer's timezone). Mirrors CalendarAdd.vue.
fn calendar_links(opp: &OpportunityExterior, now: DateTime<Utc>) -> Vec<CalEntry> {
    let starts = &opp.start_datetimes;
    let ends = &opp.end_datetimes;

    let mut pairs: Vec<(DateTime<Utc>, DateTime<Utc>)> = if !starts.is_empty()
        && starts.len() == ends.len()
    {
        starts
            .iter()
            .zip(ends.iter())
            .map(|(s, e)| (s.to_utc(), e.to_utc()))
            .collect()
    } else {
        // No matching ends: assume each runs one hour.
        starts
            .iter()
            .map(|s| (s.to_utc(), s.to_utc() + Duration::hours(1)))
            .collect()
    };

    pairs.retain(|(s, _)| *s > now);
    pairs.sort_by_key(|(s, _)| *s);
    pairs.truncate(5);

    let location = opp.location_name.clone();
    let description = opp
        .partner_opp_url
        .clone()
        .unwrap_or_default();

    pairs
        .into_iter()
        .map(|(begin, end)| cal_entry(&opp.title, &location, &description, begin, end))
        .collect()
}

fn cal_entry(
    title: &str,
    location: &str,
    description: &str,
    begin: DateTime<Utc>,
    end: DateTime<Utc>,
) -> CalEntry {
    let compact_b = begin.format("%Y%m%dT%H%M%S").to_string();
    let compact_e = end.format("%Y%m%dT%H%M%S").to_string();
    let iso_b = format!("{}+00:00", begin.format("%Y-%m-%dT%H:%M:%S"));
    let iso_e = format!("{}+00:00", end.format("%Y-%m-%dT%H:%M:%S"));

    #[derive(serde::Serialize)]
    struct Google<'a> {
        action: &'a str,
        dates: String,
        details: &'a str,
        location: &'a str,
        text: &'a str,
    }
    #[derive(serde::Serialize)]
    struct Outlook<'a> {
        body: &'a str,
        enddt: &'a str,
        location: &'a str,
        path: &'a str,
        rru: &'a str,
        startdt: &'a str,
        subject: &'a str,
    }
    #[derive(serde::Serialize)]
    struct Yahoo<'a> {
        v: &'a str,
        title: &'a str,
        st: String,
        et: String,
        desc: &'a str,
        in_loc: &'a str,
    }

    let google = format!(
        "https://calendar.google.com/calendar/render?{}",
        serde_qs::to_string(&Google {
            action: "TEMPLATE",
            dates: format!("{compact_b}Z/{compact_e}Z"),
            details: description,
            location,
            text: title,
        })
        .unwrap_or_default()
    );
    let outlook_q = |host: &str| {
        format!(
            "https://{host}/calendar/0/deeplink/compose?{}",
            serde_qs::to_string(&Outlook {
                body: description,
                enddt: &iso_e,
                location,
                path: "/calendar/action/compose",
                rru: "addevent",
                startdt: &iso_b,
                subject: title,
            })
            .unwrap_or_default()
        )
    };
    let yahoo = format!(
        "https://calendar.yahoo.com/?{}",
        serde_qs::to_string(&Yahoo {
            v: "60",
            title,
            st: format!("{compact_b}Z"),
            et: format!("{compact_e}Z"),
            desc: description,
            in_loc: location,
        })
        .unwrap_or_default()
    );

    CalEntry {
        label: begin.format("%b %-d, %Y, %-I:%M %p UTC").to_string(),
        google,
        outlook: outlook_q("outlook.live.com"),
        office365: outlook_q("outlook.office.com"),
        yahoo,
    }
}

/// Build the current action-bar state from the opportunity plus the viewer's
/// involvement flags and like count. Shared by the detail page (initial render)
/// and the toggle handlers (HTMX re-render), so the bar reflects the whole
/// involvement mode after any toggle (save/didit share one ordered mode).
async fn build_actions(
    state: &AppState,
    token: Option<&str>,
    slug: &str,
    opp: &OpportunityExterior,
) -> OppActionsView {
    let me: MeFlags = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}/me"), token)
        .await
        .unwrap_or_default();
    let likes: i64 = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}/likes"), token)
        .await
        .unwrap_or(0);

    let weblink = [opp.partner_opp_url.clone(), opp.organization_website.clone()]
        .into_iter()
        .flatten()
        .map(|s| s.trim().to_string())
        .find(|s| !s.is_empty());

    let public_url = format!("https://{}/{}", state.config.domain, slug);
    let hashtags = opp
        .opp_hashtags
        .iter()
        .map(|h| h.replace('#', ""))
        .collect::<Vec<_>>()
        .join(",");
    let tweet = format!("Check out this great science opportunity: {}", opp.title);

    // Build the social share query strings with serde_qs (already a dependency).
    #[derive(serde::Serialize)]
    struct TweetParams<'a> {
        url: &'a str,
        text: &'a str,
        via: &'a str,
        hashtags: &'a str,
    }
    #[derive(serde::Serialize)]
    struct FbParams<'a> {
        display: &'a str,
        u: &'a str,
    }
    #[derive(serde::Serialize)]
    struct UrlParam<'a> {
        url: &'a str,
    }

    let twitter_url = format!(
        "https://twitter.com/share?{}",
        serde_qs::to_string(&TweetParams {
            url: &public_url,
            text: &tweet,
            via: "science_near_me",
            hashtags: &hashtags,
        })
        .unwrap_or_default()
    );
    let facebook_url = format!(
        "https://www.facebook.com/sharer.php?{}",
        serde_qs::to_string(&FbParams {
            display: "page",
            u: &public_url,
        })
        .unwrap_or_default()
    );
    let linkedin_url = format!(
        "https://www.linkedin.com/sharing/share-offsite/?{}",
        serde_qs::to_string(&UrlParam { url: &public_url }).unwrap_or_default()
    );

    OppActionsView {
        slug: slug.to_string(),
        authenticated: token.is_some(),
        did_save: me.save,
        did_like: me.like,
        did_didit: me.didit,
        likes,
        weblink,
        public_url,
        twitter_url,
        facebook_url,
        linkedin_url,
        calendar: calendar_links(opp, Utc::now()),
    }
}

/// A "Nearby & Similar" sidebar entry.
struct RelatedView {
    slug: String,
    title: String,
    location_line: String,
    time_line: String,
}

impl RelatedView {
    fn from(opp: &OpportunityExterior) -> RelatedView {
        RelatedView {
            slug: opp.slug.clone(),
            title: opp.title.clone(),
            location_line: opportunity::location_line(opp),
            time_line: opportunity::time_line(opp),
        }
    }
}

#[derive(TemplateSimple)]
#[template(path = "pages/entity.stpl")]
struct EntityPage {
    title: String,
    subtitle: String,
    image: Option<String>,
    image_credit: String,
    short_desc_html: String,
    description_html: String,
    location_line: String,
    address_lines: Vec<String>,
    /// Raw GeoJSON geometry (point or polygon) for the "see on map" island.
    map_geojson: Option<String>,
    time_line: String,
    /// JSON arrays of ISO start/end datetimes, consumed by the opp-time island
    /// to render viewer-local times (matching OpportunityTime.vue).
    starts_json: String,
    ends_json: String,
    has_end: bool,
    keywords_line: String,
    tags: Vec<String>,
    weblink: Option<String>,
    cost_label: Option<String>,
    age_label: Option<String>,
    min_age: Option<i16>,
    max_age: Option<i16>,
    ticket_required: bool,
    languages_line: String,
    // Host (organization) and provider (partner) attributions.
    organization_name: String,
    organization_website: Option<String>,
    organization_logo_url: Option<String>,
    partner_name: String,
    partner_website: Option<String>,
    partner_logo_url: Option<String>,
    related: Vec<RelatedView>,
    /// Pre-rendered interactive action bar (HTMX island).
    actions_html: String,
    /// Pre-rendered reviews section (HTMX island).
    reviews_html: String,
    /// Whether the visitor arrived from the finder, for the breadcrumb.
    from_search: bool,
    // Owner/editor management bar (shown when the viewer is authorized).
    slug: String,
    uid: String,
    show_owner_bar: bool,
    can_manage: bool,
    withdrawn: bool,
    /// Publish-state for the bar: "declined" | "live" | "rejected" | "draft" | "pending".
    status_kind: &'static str,
}

/// Non-empty trimmed string, else `None` — for optional URL/logo fields.
fn some_nonempty(s: Option<String>) -> Option<String> {
    s.map(|v| v.trim().to_string()).filter(|v| !v.is_empty())
}

/// Opportunity (entity) detail page: `GET /<slug>`.
///
/// Fetches `/api/ui/entity/<slug>`; the API 404s for missing or unpublished
/// entities (when the viewer isn't authorized), which propagates to our 404
/// page via `json_or_err`. Read-only for now — the interactive actions
/// (save / like / "I did this" / calendar / share / reviews) are a follow-up
/// slice built as HTMX/JS islands.
#[handler]
pub async fn entity_detail(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);

    let annotated: AnnotatedOpportunityExterior = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}"), token.as_deref())
        .await?;
    let AnnotatedOpportunityExterior {
        exterior: opp,
        accepted,
        withdrawn,
        authorized,
        review_status,
        current: _,
    } = annotated;

    // Owner/editor management bar state.
    let show_owner_bar = authorized != PermitAction::Nothing;
    let can_manage = authorized == PermitAction::Manage;
    let status_kind = if !accepted {
        "declined"
    } else {
        match review_status {
            ReviewStatus::NotRequired | ReviewStatus::Publish => "live",
            ReviewStatus::Reject => "rejected",
            ReviewStatus::Draft => "draft",
            ReviewStatus::Pending => "pending",
        }
    };

    // "Nearby & Similar" — best-effort; an empty list just hides the section.
    let related: Vec<RelatedView> = state
        .api
        .get_json::<Vec<OpportunityExterior>>(
            &format!("/api/ui/entity/{slug}/recommended"),
            token.as_deref(),
        )
        .await
        .unwrap_or_default()
        .iter()
        .take(5)
        .map(RelatedView::from)
        .collect();

    let from_search = req
        .header("referer")
        .map(|r| r.contains("/find"))
        .unwrap_or(false);

    let weblink = [opp.partner_opp_url.as_deref(), opp.organization_website.as_deref()]
        .into_iter()
        .flatten()
        .map(str::trim)
        .find(|s| !s.is_empty())
        .map(str::to_string);

    let chrome = Chrome::build(
        &state.api,
        token.as_deref(),
        req.header("host"),
        req.uri().path().to_string(),
        state.config.domain.clone(),
    )
    .await;

    let actions_html = build_actions(&state, token.as_deref(), &slug, &opp)
        .await
        .render_once()
        .map_err(AppError::Render)?;
    let reviews_html = reviews_section(&state, token.as_deref(), &slug)
        .await
        .render_once()
        .map_err(AppError::Render)?;

    let page_title = format!("{} - Science Near Me", opp.title);
    let view = EntityPage {
        title: opp.title.clone(),
        subtitle: opp.organization_name.clone(),
        image: opportunity::image_url(&opp).map(str::to_string),
        image_credit: opp.image_credit.clone(),
        short_desc_html: markdown::to_html(&opp.short_desc),
        description_html: markdown::to_html(&opp.description),
        location_line: opportunity::location_line(&opp),
        address_lines: opportunity::address_lines(&opp),
        map_geojson: if !opp.is_online && opp.location_type != LocationType::Any {
            opp.location_point
                .as_ref()
                .or(opp.location_polygon.as_ref())
                .map(|v| v.to_string())
        } else {
            None
        },
        time_line: opportunity::time_line(&opp),
        starts_json: serde_json::to_string(&opp.start_datetimes).unwrap_or_else(|_| "[]".into()),
        ends_json: serde_json::to_string(&opp.end_datetimes).unwrap_or_else(|_| "[]".into()),
        has_end: opp.has_end,
        keywords_line: opportunity::keywords_full(&opp),
        tags: {
            let mut t: Vec<String> = opp.tags.iter().cloned().collect();
            t.sort();
            t
        },
        weblink,
        cost_label: opportunity::cost_label(&opp).map(str::to_string),
        age_label: opportunity::age_label(&opp),
        min_age: (opp.min_age > 0).then_some(opp.min_age),
        max_age: (opp.max_age < 999).then_some(opp.max_age),
        ticket_required: opp.ticket_required,
        languages_line: opportunity::languages_line(&opp),
        organization_name: opp.organization_name.clone(),
        organization_website: some_nonempty(opp.organization_website.clone()),
        organization_logo_url: some_nonempty(opp.organization_logo_url.clone()),
        partner_name: opp.partner_name.clone(),
        partner_website: some_nonempty(opp.partner_website.clone()),
        partner_logo_url: some_nonempty(opp.partner_logo_url.clone()),
        related,
        actions_html,
        reviews_html,
        from_search,
        uid: opp.uid.to_string(),
        slug: slug.clone(),
        show_owner_bar,
        can_manage,
        withdrawn,
        status_kind,
    };

    Ok(page(chrome, page_title, view)?.into_response())
}

/// Toggle a like (allowed anonymously), returning the re-rendered action bar.
#[handler]
pub async fn toggle_like(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let path = format!("/api/ui/entity/{slug}/likes");
    let me: MeFlags = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}/me"), token.as_deref())
        .await
        .unwrap_or_default();
    if me.like {
        let _ = state.api.delete(&path, token.as_deref()).await?;
    } else {
        let _ = state
            .api
            .post_json(&path, token.as_deref(), &serde_json::json!({}))
            .await?;
    }
    let opp: OpportunityExterior = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}"), token.as_deref())
        .await?;
    let view = build_actions(&state, token.as_deref(), &slug, &opp).await;
    Ok(Render(view).into_response())
}

/// Toggle a save (requires auth), returning the re-rendered action bar.
#[handler]
pub async fn toggle_save(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let path = format!("/api/ui/entity/{slug}/saves");
    let me: MeFlags = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}/me"), token.as_deref())
        .await
        .unwrap_or_default();
    if me.save {
        let _ = state.api.delete(&path, token.as_deref()).await?;
    } else {
        let _ = state
            .api
            .post_json(&path, token.as_deref(), &serde_json::json!({}))
            .await?;
    }
    let opp: OpportunityExterior = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}"), token.as_deref())
        .await?;
    let view = build_actions(&state, token.as_deref(), &slug, &opp).await;
    Ok(Render(view).into_response())
}

/// Toggle "I did this" (requires auth), returning the re-rendered action bar.
#[handler]
pub async fn toggle_didit(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let path = format!("/api/ui/entity/{slug}/didit");
    let me: MeFlags = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}/me"), token.as_deref())
        .await
        .unwrap_or_default();
    if me.didit {
        let _ = state.api.delete(&path, token.as_deref()).await?;
    } else {
        let _ = state
            .api
            .post_json(&path, token.as_deref(), &serde_json::json!({}))
            .await?;
    }
    let opp: OpportunityExterior = state
        .api
        .get_json(&format!("/api/ui/entity/{slug}"), token.as_deref())
        .await?;
    let view = build_actions(&state, token.as_deref(), &slug, &opp).await;
    Ok(Render(view).into_response())
}

#[derive(Debug, Deserialize)]
pub struct AddReviewInput {
    rating: i16,
    #[serde(default)]
    comment: String,
}

/// Add a review (requires auth), returning the re-rendered reviews section.
#[handler]
pub async fn add_review(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
    Form(input): Form<AddReviewInput>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let _ = state
        .api
        .post_json(
            &format!("/api/ui/entity/{slug}/reviews"),
            token.as_deref(),
            &serde_json::json!({ "rating": input.rating, "comment": input.comment }),
        )
        .await?;
    let view = reviews_section(&state, token.as_deref(), &slug).await;
    Ok(Render(view).into_response())
}

#[derive(Debug, Deserialize)]
pub struct ReportReviewInput {
    id: i32,
}

/// Report a review for moderation. Best-effort; returns a confirmation snippet
/// that replaces the Report button.
#[handler]
pub async fn report_review(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
    Form(input): Form<ReportReviewInput>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let _ = state
        .api
        .post_json(
            &format!("/api/ui/entity/{slug}/report-review"),
            token.as_deref(),
            &serde_json::json!({ "id": input.id }),
        )
        .await;
    Ok(Response::builder()
        .content_type("text/html; charset=utf-8")
        .body(r#"<span class="reported">Reported &#10003;</span>"#))
}

#[derive(Debug, Deserialize)]
pub struct StatusInput {
    /// One of "publish" | "reject" | "draft" (a ReviewStatus wire value).
    status: String,
}

/// Owner/manager publish-state change: `POST /<slug>/status`. Forwards to the
/// API's `PUT /api/ui/entity/<slug>/status`, then redirects back to the page.
#[handler]
pub async fn owner_set_status(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
    Form(input): Form<StatusInput>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let _ = state
        .api
        .put_json(
            &format!("/api/ui/entity/{slug}/status"),
            token.as_deref(),
            &serde_json::json!({ "status": input.status }),
        )
        .await?;
    Ok(Redirect::see_other(format!("/{slug}")).into_response())
}

#[derive(Debug, Deserialize)]
pub struct VisibilityInput {
    /// "true" to hide (withdraw), "false" to unhide.
    withdrawn: String,
}

/// Owner Hide/Unhide: `POST /<slug>/visibility`. Mirrors the old app — fetch the
/// entity, flip `withdrawn`, and PUT it back — then redirect to the page.
#[handler]
pub async fn owner_set_visibility(
    state: Data<&AppState>,
    jar: &CookieJar,
    Path(slug): Path<String>,
    Form(input): Form<VisibilityInput>,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);
    let resp = state
        .api
        .get(&format!("/api/ui/entity/{slug}"), token.as_deref())
        .await?;
    let mut entity: serde_json::Value = crate::api::json_or_err(resp).await?;
    entity["withdrawn"] = serde_json::Value::Bool(input.withdrawn == "true");
    let _ = state
        .api
        .put_json(&format!("/api/ui/entity/{slug}"), token.as_deref(), &entity)
        .await?;
    Ok(Redirect::see_other(format!("/{slug}")).into_response())
}
