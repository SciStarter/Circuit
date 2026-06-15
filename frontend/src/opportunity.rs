//! Display helpers for `OpportunityExterior`, shared by the finder result cards
//! and the entity detail page so the two render opportunities consistently
//! (single source of truth for location/time/keyword formatting).

use common::model::opportunity::{Cost, LocationType, OpportunityExterior};

use crate::markdown;

/// A compact opportunity card, shared by the finder results and the home page's
/// "Here & Now" section (rendered via `partials/opportunity_card.stpl`).
pub struct CardView {
    pub slug: String,
    pub title: String,
    pub subtitle: String,
    pub short_desc_html: String,
    pub image: String,
    pub location_line: String,
    pub time_line: String,
    pub keywords_line: String,
}

impl CardView {
    pub fn from(opp: OpportunityExterior) -> CardView {
        // Cards always show an image, falling back to a thumbnail placeholder.
        let image = image_url(&opp)
            .unwrap_or("/static/img/no-image-thumb.jpg")
            .to_string();

        CardView {
            slug: opp.slug.clone(),
            title: opp.title.clone(),
            subtitle: opp.organization_name.clone(),
            short_desc_html: markdown::to_html(&opp.short_desc),
            image,
            location_line: location_line(&opp),
            time_line: time_line(&opp),
            keywords_line: keywords_line(&opp),
        }
    }
}

/// The opportunity's image URL, or `None` when it has none (callers choose a
/// fallback — cards use a thumbnail placeholder, the detail page omits the
/// image entirely).
pub fn image_url(opp: &OpportunityExterior) -> Option<&str> {
    let url = opp.image_url.trim();
    if url.is_empty() {
        None
    } else {
        Some(url)
    }
}

/// One-line location summary, mirroring OpportunityLocation.vue: "Online" for
/// online opportunities, "Anywhere" for the `any` location type, otherwise
/// "City, State" (falling back to the location name).
pub fn location_line(opp: &OpportunityExterior) -> String {
    if opp.is_online {
        return "Online".to_string();
    }
    if opp.location_type == LocationType::Any {
        return "Anywhere".to_string();
    }
    let mut parts = Vec::new();
    if !opp.address_city.trim().is_empty() {
        parts.push(opp.address_city.clone());
    }
    if !opp.address_state.trim().is_empty() {
        parts.push(opp.address_state.clone());
    }
    if !parts.is_empty() {
        parts.join(", ")
    } else if !opp.location_name.trim().is_empty() {
        opp.location_name.clone()
    } else {
        // A fixed-location opportunity with no resolved address yet.
        "See description for location".to_string()
    }
}

/// Full multi-line street address for a fixed-location (At/Near) opportunity:
/// location name, street, "City, State Zip", and country (omitting the US).
/// Empty for online / "anywhere" opportunities.
pub fn address_lines(opp: &OpportunityExterior) -> Vec<String> {
    if opp.is_online || opp.location_type == LocationType::Any {
        return Vec::new();
    }
    let mut lines = Vec::new();

    let name = opp.location_name.trim();
    if !name.is_empty() {
        lines.push(name.to_string());
    }
    let street = opp.address_street.trim();
    if !street.is_empty() {
        lines.push(street.to_string());
    }

    let (city, state, zip) = (
        opp.address_city.trim(),
        opp.address_state.trim(),
        opp.address_zip.trim(),
    );
    let mut city_state = String::new();
    if !city.is_empty() {
        city_state.push_str(city);
    }
    if !state.is_empty() {
        if !city_state.is_empty() {
            city_state.push_str(", ");
        }
        city_state.push_str(state);
    }
    if !zip.is_empty() {
        if !city_state.is_empty() {
            city_state.push(' ');
        }
        city_state.push_str(zip);
    }
    if !city_state.is_empty() {
        lines.push(city_state);
    }

    let country = opp.address_country.trim();
    if !country.is_empty() && !matches!(country, "USA" | "US" | "United States") {
        lines.push(country.to_string());
    }

    lines
}

/// Human-readable, comma-separated language list (best-effort code → name).
pub fn languages_line(opp: &OpportunityExterior) -> String {
    opp.languages
        .iter()
        .map(|code| language_name(code))
        .collect::<Vec<_>>()
        .join(", ")
}

fn language_name(code: &str) -> String {
    match code.split(['-', '_']).next().unwrap_or(code) {
        "en" => "English".to_string(),
        "es" => "Spanish".to_string(),
        "fr" => "French".to_string(),
        "de" => "German".to_string(),
        "zh" => "Chinese".to_string(),
        "ja" => "Japanese".to_string(),
        "pt" => "Portuguese".to_string(),
        other => other.to_uppercase(),
    }
}

/// One-line timing: the first start date, or "Available any time" for
/// on-demand opportunities (no scheduled dates).
pub fn time_line(opp: &OpportunityExterior) -> String {
    match opp.start_datetimes.first() {
        Some(dt) => dt.format("%b %-d, %Y").to_string(),
        None => "Available any time".to_string(),
    }
}

/// Comma-separated free-text keywords/tags (up to six, alphabetized). Used on
/// the compact finder cards.
pub fn keywords_line(opp: &OpportunityExterior) -> String {
    let mut tags: Vec<&String> = opp.tags.iter().collect();
    tags.sort();
    tags.into_iter()
        .take(6)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ")
}

/// The detail page's full keyword line, mirroring OpportunityKeywords.vue:
/// activity types (descriptors) + topics, rendered with human labels, followed
/// by the free-text tags — de-duplicated, and excluding `internal_*` tags.
pub fn keywords_full(opp: &OpportunityExterior) -> String {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();

    for code in opp.opp_descriptor.iter().map(serde_code) {
        if seen.insert(code.clone()) {
            out.push(humanize_facet(&code));
        }
    }
    for code in opp.opp_topics.iter().map(serde_code) {
        if seen.insert(code.clone()) {
            out.push(humanize_facet(&code));
        }
    }
    let mut tags: Vec<&String> = opp.tags.iter().collect();
    tags.sort();
    for tag in tags {
        if !tag.starts_with("internal_") && seen.insert(tag.clone()) {
            out.push(tag.clone());
        }
    }

    out.join(", ")
}

/// The serde wire code (snake_case) for an enum variant, e.g. `Topic` →
/// "astronomy_and_space".
fn serde_code<T: serde::Serialize>(value: T) -> String {
    serde_json::to_string(&value)
        .unwrap_or_default()
        .trim_matches('"')
        .to_string()
}

/// Turn a snake_case facet code into a display label: "astronomy_and_space" →
/// "Astronomy & Space", "citizen_science" → "Citizen Science".
fn humanize_facet(code: &str) -> String {
    code.split('_')
        .map(|word| {
            if word == "and" {
                "&".to_string()
            } else {
                let mut chars = word.chars();
                match chars.next() {
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    None => String::new(),
                }
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A short cost label, or `None` when the cost is unknown.
pub fn cost_label(opp: &OpportunityExterior) -> Option<&'static str> {
    match opp.cost {
        Cost::Free => Some("Free"),
        Cost::Cost => Some("Has a cost"),
        Cost::Unknown => None,
    }
}

/// An age-range label, shown only when the opportunity actually restricts ages
/// (the defaults are 0 and 999).
pub fn age_label(opp: &OpportunityExterior) -> Option<String> {
    let (min, max) = (opp.min_age, opp.max_age);
    match (min > 0, max < 999) {
        (false, false) => None,
        (true, false) => Some(format!("Ages {min}+")),
        (false, true) => Some(format!("Up to age {max}")),
        (true, true) => Some(format!("Ages {min}\u{2013}{max}")),
    }
}
