use poem::web::cookie::{Cookie, CookieJar, SameSite};
use serde::Deserialize;

use crate::api::ApiClient;

/// Name of the session cookie. Mirrors the backend (`backend/src/ui/auth.rs`):
/// the `__Host-` prefix (which requires Secure + path=/ + no Domain) is used
/// in release builds, plain `token` in debug. The frontend and API share an
/// origin, so the same cookie is visible to both.
#[cfg(not(debug_assertions))]
pub const TOKEN_COOKIE: &str = "__Host-token";
#[cfg(debug_assertions)]
pub const TOKEN_COOKIE: &str = "token";

/// 90 days, matching the backend session lifetime.
const SESSION_DAYS: i64 = 90;

/// The current user as surfaced by `GET /api/ui/auth/me`. Rendered into the
/// nav chrome; `None` means anonymous.
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub uid: String,
    pub token: String,
    pub username: Option<String>,
    pub image_url: Option<String>,
    pub num_partners: i64,
    pub reports_pending: i64,
}

/// Shape of the `/api/ui/auth/me` (and login/signup) JSON payload. All
/// identity fields are optional because the anonymous response is just
/// `{"authenticated": false}`.
#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub authenticated: bool,
    pub uid: Option<String>,
    pub token: Option<String>,
    pub username: Option<String>,
    pub image_url: Option<String>,
    #[serde(default)]
    pub num_partners: i64,
    #[serde(default)]
    pub reports_pending: i64,
}

impl MeResponse {
    pub fn into_user(self) -> Option<CurrentUser> {
        match (self.authenticated, self.uid, self.token) {
            (true, Some(uid), Some(token)) => Some(CurrentUser {
                uid,
                token,
                username: self.username,
                image_url: self.image_url,
                num_partners: self.num_partners,
                reports_pending: self.reports_pending,
            }),
            _ => None,
        }
    }
}

/// Read the session token from the request's cookie jar, if present.
pub fn token_from_jar(jar: &CookieJar) -> Option<String> {
    jar.get(TOKEN_COOKIE).map(|c| c.value_str().to_string())
}

/// Resolve the current user for this request by forwarding the session token
/// to the API. Returns `None` for anonymous sessions or on any failure (the
/// chrome simply renders logged-out).
pub async fn current_user(api: &ApiClient, token: Option<&str>) -> Option<CurrentUser> {
    let token = token?;
    let me: MeResponse = api.get_json("/api/ui/auth/me", Some(token)).await.ok()?;
    me.into_user()
}

/// Set the session cookie on the response, mirroring the backend's cookie
/// attributes so it is interchangeable with the API-set cookie.
pub fn set_token_cookie(jar: &CookieJar, token: String) {
    let mut cookie = Cookie::new_with_str(TOKEN_COOKIE, token);
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_max_age(std::time::Duration::from_secs((SESSION_DAYS * 24 * 3600) as u64));
    if cfg!(not(debug_assertions)) {
        cookie.set_secure(true);
        cookie.set_same_site(SameSite::None);
    } else {
        cookie.set_same_site(SameSite::Lax);
    }
    jar.add(cookie);
}

/// Clear the session cookie (logout).
pub fn clear_token_cookie(jar: &CookieJar) {
    jar.remove(TOKEN_COOKIE);
}
