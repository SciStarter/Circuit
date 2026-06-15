use chrono::Utc;
use poem::web::cookie::CookieJar;
use poem::web::Data;
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::{Deserialize, Serialize};

use common::model::OpportunityExterior;

use crate::chrome::Chrome;
use crate::error::AppError;
use crate::iplookup;
use crate::opportunity::CardView;
use crate::render::page;
use crate::session::token_from_jar;
use crate::AppState;

/// Response from the backend geolocate endpoint (`/api/ui/finder/geolocate`).
#[derive(Debug, Deserialize)]
struct GeolocateResponse {
    longitude: f32,
    latitude: f32,
    #[serde(default)]
    near: String,
}

#[derive(Debug, Deserialize)]
struct SearchResults {
    #[serde(default)]
    matches: Vec<OpportunityExterior>,
}

/// One "What would you like to do?" intent card.
struct IntentCard {
    title: &'static str,
    description: &'static str,
    image: &'static str,
    image2x: &'static str,
    link: String,
}

#[derive(TemplateSimple)]
#[template(path = "pages/home.stpl")]
struct HomePage {
    city: String,
    here_and_now_link: String,
    intents: Vec<IntentCard>,
    here_and_now: Vec<CardView>,
    /// Activity types (descriptors): (code, label).
    topics: Vec<(String, String)>,
    authenticated: bool,
    // Search-bar prefill (from IP geolocation).
    search_near: String,
    search_longitude: String,
    search_latitude: String,
}

/// Location-scoped "Here & Now" query, also used to build the see-more link.
#[derive(Serialize)]
struct HereQuery<'a> {
    beginning: &'a str,
    near: &'a str,
    longitude: f32,
    latitude: f32,
    sort: &'a str,
}

#[handler]
pub async fn home(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    let token = token_from_jar(jar);

    // Best-effort IP geolocation (same backend endpoint the finder uses).
    let geo: Option<GeolocateResponse> = {
        let path = match iplookup::client_ip(req) {
            Some(ip) => format!("/api/ui/finder/geolocate?ip={ip}"),
            None => "/api/ui/finder/geolocate".to_string(),
        };
        state.api.get_json::<GeolocateResponse>(&path, None).await.ok()
    };
    let located = geo.as_ref().filter(|g| !g.near.trim().is_empty());

    let city = located
        .and_then(|g| g.near.split(',').next())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "you".to_string());

    let now = Utc::now().to_rfc3339();

    // "Here & Now" search query string and the see-more link.
    let (here_query, here_and_now_link) = match located {
        Some(g) => {
            let q = serde_qs::to_string(&HereQuery {
                beginning: &now,
                near: &g.near,
                longitude: g.longitude,
                latitude: g.latitude,
                sort: "closest",
            })
            .unwrap_or_default();
            (q.clone(), format!("/find?{q}"))
        }
        None => {
            let q = format!("beginning={}&sort=closest", urlencode(&now));
            (q, "/find?sort=closest".to_string())
        }
    };

    // Here & Now cards (best-effort; empty section if it fails).
    let here_and_now: Vec<CardView> = state
        .api
        .get_json::<SearchResults>(&format!("/api/ui/finder/search?{here_query}"), token.as_deref())
        .await
        .map(|r| r.matches.into_iter().take(6).map(CardView::from).collect())
        .unwrap_or_default();

    // Activity types for the "By Activity Type" section.
    let topics: Vec<(String, String)> = state
        .api
        .get_json("/api/ui/finder/descriptors", None)
        .await
        .unwrap_or_default();

    let intents = intent_cards(&here_and_now_link);

    let chrome = Chrome::build(
        &state.api,
        token.as_deref(),
        req.header("host"),
        req.uri().path().to_string(),
        state.config.domain.clone(),
    )
    .await;
    let authenticated = chrome.authenticated();

    let view = HomePage {
        city,
        here_and_now_link,
        intents,
        here_and_now,
        topics,
        authenticated,
        search_near: located.map(|g| g.near.clone()).unwrap_or_default(),
        search_longitude: located.map(|g| g.longitude.to_string()).unwrap_or_default(),
        search_latitude: located.map(|g| g.latitude.to_string()).unwrap_or_default(),
    };

    Ok(page(chrome, "Science Near Me", view)?.into_response())
}

/// The six intent cards, each linking into the finder with preset filters.
fn intent_cards(base: &str) -> Vec<IntentCard> {
    let mk = |suffix: &str| format!("{base}&{suffix}");
    vec![
        IntentCard {
            title: "Listen, Learn, Discuss, Inform",
            description: "Participate in live dialogues about current science and society issues",
            image: "/static/img/learn-discuss.jpg",
            image2x: "/static/img/learn-discuss@2x.jpg",
            link: mk("physical=in-person-or-online&text=forum"),
        },
        IntentCard {
            title: "Create or Build",
            description: "Be creative and do something hands-on",
            image: "/static/img/create-build.jpg",
            image2x: "/static/img/create-build@2x.jpg",
            link: mk("physical=in-person-or-online&text=maker"),
        },
        IntentCard {
            title: "Explore Earth and Space",
            description: "Feed your curiosity with an expert guide",
            image: "/static/img/explore-space.jpg",
            image2x: "/static/img/explore-space@2x.jpg",
            link: mk("physical=in-person-or-online&descriptors[]=star_party"),
        },
        IntentCard {
            title: "Celebrate Science",
            description: "Go to a science festival",
            image: "/static/img/celebrate-science.jpg",
            image2x: "/static/img/celebrate-science@2x.jpg",
            link: mk("physical=in-person-or-online&text=festival"),
        },
        IntentCard {
            title: "Make a Difference",
            description: "Participate in science or serve your community",
            image: "/static/img/make-difference.jpg",
            image2x: "/static/img/make-difference@2x.jpg",
            link: mk("physical=in-person-or-online&descriptors[]=citizen_science"),
        },
        IntentCard {
            title: "For Kids",
            description: "Where kids can explore, learn, and get excited about science",
            image: "/static/img/for-kids.jpg",
            image2x: "/static/img/for-kids@2x.jpg",
            link: mk("physical=in-person-or-online&max_age=13"),
        },
    ]
}

/// Minimal percent-encoding for a single query value (the RFC3339 timestamp).
fn urlencode(value: &str) -> String {
    value
        .bytes()
        .map(|b| match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                (b as char).to_string()
            }
            _ => format!("%{b:02X}"),
        })
        .collect()
}
