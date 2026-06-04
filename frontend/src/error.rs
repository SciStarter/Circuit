use poem::{http::StatusCode, IntoResponse, Response};

/// Errors that can occur while handling a request. Each maps to an HTTP
/// status and a rendered error page. Handlers return `Result<_, AppError>`
/// and Poem turns the error into a response via `IntoResponse`.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("upstream API request failed: {0}")]
    Api(#[from] reqwest::Error),

    #[error("upstream API returned status {status}")]
    ApiStatus { status: StatusCode, body: String },

    #[error("template rendering failed: {0}")]
    Render(#[from] sailfish::RenderError),

    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("not found")]
    NotFound,
}

impl AppError {
    pub fn status(&self) -> StatusCode {
        match self {
            AppError::Api(_) | AppError::Render(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::ApiStatus { status, .. } => *status,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    /// Build the rendered error response. Shared by `IntoResponse` and
    /// `ResponseError`.
    fn render(&self) -> Response {
        let status = self.status();
        if status.is_server_error() {
            tracing::error!("request failed: {self}");
        } else {
            tracing::debug!("request rejected: {self}");
        }

        // A minimal standalone error page. Phase 8 replaces this with a
        // themed error template matching the old 404/error pages.
        let body = format!(
            "<!doctype html><html><head><meta charset=\"utf-8\"><title>{code} {reason}</title></head>\
             <body style=\"font-family:sans-serif;padding:2rem\"><h1>{code} {reason}</h1></body></html>",
            code = status.as_u16(),
            reason = status.canonical_reason().unwrap_or("Error"),
        );

        Response::builder()
            .status(status)
            .content_type("text/html; charset=utf-8")
            .body(body)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        self.render()
    }
}

impl poem::error::ResponseError for AppError {
    fn status(&self) -> StatusCode {
        AppError::status(self)
    }

    fn as_response(&self) -> Response {
        self.render()
    }
}
