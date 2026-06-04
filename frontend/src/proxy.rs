use poem::http::{header, HeaderMap, StatusCode};
use poem::web::Data;
use poem::{handler, Body, Request, Response};

use crate::error::AppError;
use crate::AppState;

/// Reverse-proxy `/api/*` to the backend API service.
///
/// In production the ingress routes `/api/*` straight to the API (so this is
/// never hit). Locally there is no ingress, so the frontend forwards these
/// requests itself — which is what makes client-side `fetch('/api/...')` (the
/// location typeahead, etc.) work in development.
#[handler]
pub async fn api_proxy(
    state: Data<&AppState>,
    req: &Request,
    body: Body,
) -> Result<Response, AppError> {
    let path_and_query = req
        .uri()
        .path_and_query()
        .map(|pq| pq.as_str())
        .unwrap_or_else(|| req.uri().path());
    let url = format!("{}{}", state.api.base(), path_and_query);

    let bytes = body.into_bytes().await.map_err(|e| AppError::BadRequest(e.to_string()))?;

    // Forward the request headers except Host (reqwest sets it from the URL).
    let mut headers = HeaderMap::new();
    for (name, value) in req.headers() {
        if name != header::HOST {
            headers.insert(name.clone(), value.clone());
        }
    }

    let upstream = state
        .api
        .client()
        .request(req.method().clone(), &url)
        .headers(headers)
        .body(bytes.to_vec())
        .send()
        .await?;

    // Relay status, content-type, and any Set-Cookie headers back to the client.
    let status = StatusCode::from_u16(upstream.status().as_u16()).unwrap_or(StatusCode::BAD_GATEWAY);
    let mut builder = Response::builder().status(status);
    if let Some(ct) = upstream.headers().get(header::CONTENT_TYPE) {
        builder = builder.header(header::CONTENT_TYPE, ct.clone());
    }
    for cookie in upstream.headers().get_all(header::SET_COOKIE) {
        builder = builder.header(header::SET_COOKIE, cookie.clone());
    }

    let payload = upstream.bytes().await?;
    Ok(builder.body(payload.to_vec()))
}
