use std::env;

/// Runtime configuration for the SSR frontend, read once from the
/// environment at startup. The frontend owns no database; it only needs to
/// know how to reach the API service and a few values that surface in
/// rendered pages.
#[derive(Clone, Debug)]
pub struct Config {
    /// Port the Poem server listens on.
    pub port: u16,
    /// Base URL used for server-side calls to the API (no trailing slash).
    pub api_base_url: String,
    /// Public site domain (used for canonical/OG URLs).
    pub domain: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            port: env::var("FRONTEND_PORT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(3000),
            api_base_url: api_base_url(),
            domain: env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string()),
        }
    }
}

/// Resolve the API base URL. An explicit `API_BASE_URL` wins; otherwise we
/// build one from the Kubernetes service-discovery variables the cluster
/// injects (mirroring the old Nuxt config), falling back to localhost.
fn api_base_url() -> String {
    if let Ok(url) = env::var("API_BASE_URL") {
        return url.trim_end_matches('/').to_string();
    }

    let host = env::var("CIRCUIT_API_SERVICE_SERVICE_HOST")
        .or_else(|_| env::var("CIRCUIT_API_SERVICE_BETA_SERVICE_HOST"))
        .unwrap_or_else(|_| "localhost".to_string());
    let port = env::var("CIRCUIT_API_SERVICE_SERVICE_PORT")
        .or_else(|_| env::var("CIRCUIT_API_SERVICE_BETA_SERVICE_PORT"))
        .unwrap_or_else(|_| "8000".to_string());

    format!("http://{host}:{port}")
}
