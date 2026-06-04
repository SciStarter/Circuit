use crate::api::ApiClient;
use crate::session::{current_user, CurrentUser};

/// Per-request context for the global chrome (header, nav, footer). Built once
/// per request and rendered into every page via the layout. Mirrors the data
/// the old `layouts/default.vue` derived from the Vuex `user` state.
pub struct Chrome {
    pub user: Option<CurrentUser>,
    /// Whether this is the beta host (old app keyed off `host` containing
    /// "beta."); surfaces the beta styling/labels.
    pub beta: bool,
    /// Public site domain, for canonical/OG URLs.
    pub domain: String,
    /// Request path, used to highlight the active nav item.
    pub path: String,
}

impl Chrome {
    /// Build the chrome for a request: resolve the current user from the
    /// session token and capture host-derived flags.
    pub async fn build(
        api: &ApiClient,
        token: Option<&str>,
        host: Option<&str>,
        path: String,
        domain: String,
    ) -> Chrome {
        Chrome {
            user: current_user(api, token).await,
            beta: host.map(|h| h.contains("beta.")).unwrap_or(false),
            domain,
            path,
        }
    }

    /// Whether the current request path matches `p` (for nav highlighting).
    pub fn is_active(&self, p: &str) -> bool {
        self.path == p
    }

    pub fn authenticated(&self) -> bool {
        self.user.is_some()
    }

    /// An "owner" can manage opportunities (belongs to at least one partner).
    pub fn owner(&self) -> bool {
        self.user.as_ref().map_or(false, |u| u.num_partners > 0)
    }

    pub fn username(&self) -> &str {
        self.user
            .as_ref()
            .and_then(|u| u.username.as_deref())
            .unwrap_or("")
    }

    pub fn reports_pending(&self) -> i64 {
        self.user.as_ref().map_or(0, |u| u.reports_pending)
    }
}
