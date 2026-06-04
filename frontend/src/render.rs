use poem::{IntoResponse, Response};
use sailfish::TemplateSimple;

use crate::chrome::Chrome;
use crate::error::AppError;

/// Wraps a Sailfish template so it can be returned directly from a Poem
/// handler. Mirrors the backend's template -> Response convenience
/// (see `backend/src/v1/manage/mod.rs`): a handler ends with
/// `Ok(Page { .. }.render())`. We derive `TemplateSimple` (Sailfish 0.11's
/// trait that destructures struct fields into template scope, so templates
/// reference fields by bare name).
pub struct Render<T: TemplateSimple>(pub T);

impl<T: TemplateSimple + Send> IntoResponse for Render<T> {
    fn into_response(self) -> Response {
        match self.0.render_once() {
            Ok(body) => Response::builder()
                .content_type("text/html; charset=utf-8")
                .body(body),
            Err(e) => AppError::Render(e).into_response(),
        }
    }
}

/// The global chrome wrapper. Page content is rendered to a string and
/// embedded here, so page templates stay decoupled from the header/nav/footer
/// (single source of chrome).
#[derive(TemplateSimple)]
#[template(path = "layouts/default.stpl")]
pub struct DefaultLayout {
    pub chrome: Chrome,
    pub title: String,
    pub body: String,
}

/// Render `content` and wrap it in the default chrome.
pub fn page(
    chrome: Chrome,
    title: impl Into<String>,
    content: impl TemplateSimple,
) -> Result<Render<DefaultLayout>, AppError> {
    let body = content.render_once()?;
    Ok(Render(DefaultLayout {
        chrome,
        title: title.into(),
        body,
    }))
}
