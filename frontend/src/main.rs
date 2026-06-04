mod api;
mod chrome;
mod config;
mod error;
mod iplookup;
mod markdown;
mod proxy;
mod render;
mod routes;
mod session;

use poem::endpoint::StaticFilesEndpoint;
use poem::listener::TcpListener;
use poem::middleware::CookieJarManager;
use poem::web::Redirect;
use poem::{get, handler, post, EndpointExt, IntoResponse, Route, Server};

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

/// The home page is built in Phase 3; until then, send the root to the finder.
#[handler]
fn home() -> impl IntoResponse {
    Redirect::see_other("/find")
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
        .at("/", get(home))
        .at("/find", get(routes::finder::find))
        .at(
            "/login",
            get(routes::auth::login_form).post(routes::auth::login_submit),
        )
        .at("/logout", post(routes::auth::logout))
        .at("/api/*path", proxy::api_proxy)
        .nest("/static", StaticFilesEndpoint::new("static"))
        .with(CookieJarManager::new())
        .data(state);

    Server::new(TcpListener::bind(("0.0.0.0", config.port)))
        .run(app)
        .await
}
