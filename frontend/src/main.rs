mod api;
mod chrome;
mod config;
mod error;
mod iplookup;
mod markdown;
mod opportunity;
mod proxy;
mod render;
mod routes;
mod session;

use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::CookieJarManager;
use poem::{get, handler, post, EndpointExt, Route, Server};

use crate::api::ApiClient;
use crate::config::Config;

/// Shared application state, injected into handlers via `Data<&AppState>`.
#[derive(Clone)]
pub struct AppState {
    pub config: Config,
    pub api: ApiClient,
}

#[handler]
fn healthz() -> &'static str {
    "ok"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".into()),
        )
        .init();

    let config = Config::from_env();
    tracing::info!(port = config.port, api = %config.api_base_url, "starting frontend");

    let api = ApiClient::new(config.api_base_url.clone());
    let state = AppState {
        config: config.clone(),
        api,
    };

    let app = Route::new()
        .at("/healthz", get(healthz))
        .at("/", get(routes::home::home))
        .at("/find", get(routes::finder::find))
        .at(
            "/login",
            get(routes::auth::login_form).post(routes::auth::login_submit),
        )
        .at(
            "/signup",
            get(routes::auth::signup_form).post(routes::auth::signup_submit),
        )
        .at(
            "/login-scistarter",
            get(routes::auth::scistarter_form).post(routes::auth::scistarter_submit),
        )
        .at(
            "/forgot",
            get(routes::auth::forgot_form).post(routes::auth::forgot_submit),
        )
        .at("/account", get(routes::auth::account))
        .at("/logout", post(routes::auth::logout))
        .at("/api/*path", proxy::api_proxy)
        // Interactive action-bar toggles (HTMX) on a detail page.
        .at("/:slug/like", post(routes::entity::toggle_like))
        .at("/:slug/save", post(routes::entity::toggle_save))
        .at("/:slug/didit", post(routes::entity::toggle_didit))
        .at("/:slug/reviews", post(routes::entity::add_review))
        .at("/:slug/report-review", post(routes::entity::report_review))
        // Owner/editor management actions on a detail page.
        .at("/:slug/status", post(routes::entity::owner_set_status))
        .at("/:slug/visibility", post(routes::entity::owner_set_visibility))
        // Entity detail by slug. Registered last; Poem matches static segments
        // (`/find`, `/login`, …) before this dynamic one.
        .at("/:slug", get(routes::entity::entity_detail))
        .nest("/static", StaticFilesEndpoint::new("static"))
        .with(CookieJarManager::new())
        .data(state);

    Server::new(TcpListener::bind(("0.0.0.0", config.port)))
        .run(app)
        .await
}
