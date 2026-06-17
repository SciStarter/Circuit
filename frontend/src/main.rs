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
        // Authenticated account area.
        .at("/my/saved", get(routes::my::saved_opportunities))
        .at("/my/saved/remove-old", post(routes::my::remove_old_saved))
        .at("/my/saved/:uid/remove", post(routes::my::remove_saved))
        .at("/my/science", get(routes::my::science))
        .at("/my/science/report", post(routes::my::report_involvement))
        .at(
            "/my/profile",
            get(routes::my::profile_settings).post(routes::my::save_profile),
        )
        .at("/my/profile/delete", post(routes::my::delete_account))
        .at("/my/goals", get(routes::my::goals_page))
        .at("/my/goals/set", post(routes::my::set_goal))
        .at("/my/goals/:id/status", post(routes::my::update_goal_status))
        .at("/my/goals/:id/delete", post(routes::my::delete_goal))
        .at(
            "/my/submit-opportunity",
            get(routes::opp_form::new_opportunity).post(routes::opp_form::save_new),
        )
        .at(
            "/my/opportunity/:uid",
            get(routes::opp_form::edit_opportunity).post(routes::opp_form::save_existing),
        )
        .at("/my/opportunities", get(routes::my::opportunities))
        .at(
            "/my/opportunities/export",
            get(routes::my::export_opportunities),
        )
        .at(
            "/my/opportunities/:uid/trash",
            post(routes::my::trash_opportunity),
        )
        .at("/my/data-overview", get(routes::data::data_overview))
        .at("/my/hosts-explorer", get(routes::data::hosts_explorer))
        .at(
            "/my/opportunity-data-explorer",
            get(routes::data::opportunity_data_explorer),
        )
        .at(
            "/my/snm-data-overview",
            get(routes::data::snm_data_overview),
        )
        .at("/my/organization", get(routes::my::organization))
        .at("/my/organization/:uid", post(routes::my::save_organization))
        .at(
            "/my/organization/:uid/member",
            post(routes::my::manage_org_member),
        )
        .at(
            "/my/organization/:uid/invite",
            post(routes::my::invite_managers),
        )
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
