use pulldown_cmark::{html, Options, Parser};

/// Render Markdown to sanitized HTML. Opportunity text fields (short_desc,
/// description) are Markdown authored by partners, matching the old frontend's
/// client-side `vue-markdown` rendering. Output is sanitized with ammonia
/// since the source is user-supplied.
pub fn to_html(src: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);

    let parser = Parser::new_ext(src, options);
    let mut unsafe_html = String::new();
    html::push_html(&mut unsafe_html, parser);

    ammonia::clean(&unsafe_html)
}

#[cfg(test)]
mod tests {
    use super::to_html;

    #[test]
    fn renders_basic_markdown() {
        let html = to_html("Telescopes **on the lawn**. [Details](https://example.org)");
        assert!(html.contains("<strong>on the lawn</strong>"));
        assert!(html.contains("href=\"https://example.org\""));
    }

    #[test]
    fn sanitizes_dangerous_html() {
        let html = to_html("hello <script>alert(1)</script> world");
        assert!(!html.contains("<script>"));
        assert!(html.contains("hello"));
    }
}
