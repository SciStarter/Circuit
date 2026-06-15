use poem::http::StatusCode;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::AppError;

/// Thin async client for the backend JSON API. The frontend is a pure API
/// consumer: every handler that needs data calls through here. The caller's
/// session token (read from the incoming request cookie) is forwarded as a
/// `Bearer` header, which the API accepts in place of the cookie (see
/// `request_person` in `backend/src/ui/mod.rs`).
#[derive(Clone)]
pub struct ApiClient {
    client: reqwest::Client,
    base: String,
}

impl ApiClient {
    pub fn new(base: impl Into<String>) -> Self {
        ApiClient {
            client: reqwest::Client::builder()
                .build()
                .expect("failed to build HTTP client"),
            base: base.into(),
        }
    }

    fn url(&self, path: &str) -> String {
        format!("{}{}", self.base, path)
    }

    /// The API base URL (no trailing slash).
    pub fn base(&self) -> &str {
        &self.base
    }

    /// The underlying HTTP client, for the `/api/*` reverse proxy.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// GET `path`, forwarding the session token if present. Returns the raw
    /// response so callers can inspect status, headers, or body as needed.
    pub async fn get(&self, path: &str, token: Option<&str>) -> Result<reqwest::Response, AppError> {
        let mut req = self.client.get(self.url(path));
        if let Some(t) = token {
            req = req.bearer_auth(t);
        }
        Ok(req.send().await?)
    }

    /// GET `path` and deserialize a successful JSON response into `T`.
    pub async fn get_json<T: DeserializeOwned>(
        &self,
        path: &str,
        token: Option<&str>,
    ) -> Result<T, AppError> {
        let resp = self.get(path, token).await?;
        json_or_err(resp).await
    }

    /// POST `body` as JSON to `path`, forwarding the session token if present.
    pub async fn post_json<B: Serialize + ?Sized>(
        &self,
        path: &str,
        token: Option<&str>,
        body: &B,
    ) -> Result<reqwest::Response, AppError> {
        let mut req = self.client.post(self.url(path)).json(body);
        if let Some(t) = token {
            req = req.bearer_auth(t);
        }
        Ok(req.send().await?)
    }

    /// DELETE `path`, forwarding the session token if present.
    pub async fn delete(
        &self,
        path: &str,
        token: Option<&str>,
    ) -> Result<reqwest::Response, AppError> {
        let mut req = self.client.delete(self.url(path));
        if let Some(t) = token {
            req = req.bearer_auth(t);
        }
        Ok(req.send().await?)
    }

    /// PUT `body` as JSON to `path`, forwarding the session token if present.
    pub async fn put_json<B: Serialize + ?Sized>(
        &self,
        path: &str,
        token: Option<&str>,
        body: &B,
    ) -> Result<reqwest::Response, AppError> {
        let mut req = self.client.put(self.url(path)).json(body);
        if let Some(t) = token {
            req = req.bearer_auth(t);
        }
        Ok(req.send().await?)
    }
}

/// Deserialize a successful response into `T`, or surface the upstream status
/// and body as an `AppError`.
pub async fn json_or_err<T: DeserializeOwned>(resp: reqwest::Response) -> Result<T, AppError> {
    let status = resp.status();
    if status.is_success() {
        Ok(resp.json::<T>().await?)
    } else {
        let body = resp.text().await.unwrap_or_default();
        Err(AppError::ApiStatus {
            status: StatusCode::from_u16(status.as_u16()).unwrap_or(StatusCode::BAD_GATEWAY),
            body,
        })
    }
}
