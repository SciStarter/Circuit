//! The "Data Insights" dashboards (`/my/data-overview`, …). These render the
//! batch-compiled analytics from `/api/ui/organization/analytics`. The compiled
//! payload is already largely Chart.js-shaped (pie configs, time-series arrays,
//! device/state metric objects), so the handlers shape it into complete
//! Chart.js configs (embedded as `<canvas data-chart=…>` JSON for the charts.js
//! island) and into metric tables with comparison bars.

use poem::web::cookie::CookieJar;
use poem::web::{Data, Query};
use poem::{handler, IntoResponse, Request, Response};
use sailfish::TemplateSimple;
use serde::Deserialize;
use serde_json::{json, Value};

use crate::error::AppError;
use crate::render::page;
use crate::routes::my::require_user;
use crate::AppState;

/// The default reporting window (the analytics API keys rows by these enum
/// *names*, whose discriminants are 0/0).
const DEFAULT_PERIOD: &str = "This Month";
const DEFAULT_STATUS: &str = "Live and Closed";

/// The six audience metric columns, in display order.
const METRIC_COLUMNS: &[&str] = &[
    "Unique Users",
    "New Users",
    "Returning Users",
    "Total Pageviews",
    "Unique Pageviews",
    "Avg. Time",
];

/// Line-series colours (SNM palette).
const SERIES_COLORS: &[&str] = &["#268699", "#7cb4bf", "#f2c04b", "#991b08"];

#[derive(Debug, Deserialize)]
struct OrgOpt {
    uid: String,
    name: String,
}

// --- small Value helpers ---

fn f(v: &Value, key: &str) -> f64 {
    v.get(key).and_then(Value::as_f64).unwrap_or(0.0)
}

/// Format a metric for display: "Avg. Time" is seconds → m:ss, others integers.
fn fmt_metric(col: &str, v: f64) -> String {
    if col == "Avg. Time" {
        let secs = v.round() as i64;
        format!("{}:{:02}", secs / 60, secs % 60)
    } else {
        format!("{}", v.round() as i64)
    }
}

/// One cell of a metric table: formatted value + a 0–100 bar percentage.
struct Cell {
    value: String,
    pct: i64,
}

/// A labelled row of metric cells (a device, a state, a referral source…).
struct MetricRow {
    label: String,
    cells: Vec<Cell>,
}

/// Build a metric row from a metrics object against the `max` object.
fn metric_row(label: &str, m: &Value, max: &Value) -> MetricRow {
    let cells = METRIC_COLUMNS
        .iter()
        .map(|&col| {
            let val = f(m, col);
            let mx = f(max, col).max(1.0);
            Cell {
                value: fmt_metric(col, val),
                pct: ((val / mx) * 100.0).clamp(0.0, 100.0) as i64,
            }
        })
        .collect();
    MetricRow {
        label: label.to_string(),
        cells,
    }
}

/// Build a Chart.js line config from a time-series array (`[{date, <series>…}]`).
fn line_config(chart: &Value, series: &[&str]) -> String {
    let points = chart.as_array().cloned().unwrap_or_default();
    let labels: Vec<String> = points
        .iter()
        .map(|p| {
            p.get("date")
                .and_then(Value::as_str)
                .map(|d| if d.len() >= 10 { d[..10].to_string() } else { d.to_string() })
                .unwrap_or_default()
        })
        .collect();
    let datasets: Vec<Value> = series
        .iter()
        .enumerate()
        .map(|(i, &name)| {
            let color = SERIES_COLORS[i % SERIES_COLORS.len()];
            let data: Vec<f64> = points.iter().map(|p| f(p, name)).collect();
            json!({
                "label": name,
                "data": data,
                "borderColor": color,
                "backgroundColor": color,
                "tension": 0.3,
                "fill": false,
            })
        })
        .collect();
    json!({
        "type": "line",
        "data": { "labels": labels, "datasets": datasets },
        "options": { "plugins": { "legend": { "position": "bottom" } } }
    })
    .to_string()
}

/// Wrap a pre-built Chart.js pie/doughnut `data` object into a full config.
fn pie_config(data: &Value, doughnut: bool) -> String {
    json!({
        "type": if doughnut { "doughnut" } else { "pie" },
        "data": data,
        "options": { "plugins": { "legend": { "position": "bottom" } } }
    })
    .to_string()
}

/// One engagement comparison row: this org vs the network mean/median.
struct EngBar {
    label: String,
    self_val: i64,
    mean_val: i64,
    median_val: i64,
    self_pct: i64,
    mean_pct: i64,
    median_pct: i64,
}

/// The analytics sections shared by the org overview and the per-opportunity
/// explorer — both render the same engagement / traffic / technology / audience
/// from a compiled kind=0 `data` object.
#[derive(Default)]
struct AnalyticsSections {
    eng_total_views: i64,
    eng_total_clicks: i64,
    eng_line: String,
    eng_bars: Vec<EngBar>,
    traffic_line: String,
    traffic_pie: String,
    traffic_has_pie: bool,
    traffic_rows: Vec<MetricRow>,
    tech_doughnut: String,
    tech_rows: Vec<MetricRow>,
    states_rows: Vec<MetricRow>,
}

/// Shape a compiled kind=0 analytics `data` object into the renderable sections.
fn build_sections(data: &Value) -> AnalyticsSections {
    // Engagement: line + this-vs-network comparison bars.
    let eng = &data["engagement"]["data"];
    let eng_line = line_config(&eng["chart"], &["Views", "Unique", "Clicks to Website"]);
    let bars = &eng["bars"];
    let (eng_self, eng_mean, eng_median) = (&bars["self"], &bars["mean"], &bars["median"]);
    let eng_cols: Vec<String> = eng["columns"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_str().map(String::from)).collect())
        .unwrap_or_default();
    let eng_bars = eng_cols
        .iter()
        .map(|col| {
            let s = f(eng_self, col);
            let me = f(eng_mean, col);
            let md = f(eng_median, col);
            let mx = s.max(me).max(md).max(1.0);
            EngBar {
                label: col.clone(),
                self_val: s as i64,
                mean_val: me as i64,
                median_val: md as i64,
                self_pct: ((s / mx) * 100.0) as i64,
                mean_pct: ((me / mx) * 100.0) as i64,
                median_pct: ((md / mx) * 100.0) as i64,
            }
        })
        .collect();

    // Traffic: line + referral pie + source table.
    let traffic = &data["traffic"]["data"];
    let traffic_line = line_config(&traffic["chart"], &["Views", "Unique", "Clicks to Website"]);
    let pie = &traffic["pie"];
    let traffic_has_pie = pie
        .get("datasets")
        .and_then(|d| d.get(0))
        .and_then(|d| d.get("data"))
        .and_then(Value::as_array)
        .map(|a| !a.is_empty())
        .unwrap_or(false);
    let traffic_max = &traffic["max"];
    let traffic_rows = traffic["table"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .map(|r| metric_row(r.get("name").and_then(Value::as_str).unwrap_or(""), r, traffic_max))
                .collect()
        })
        .unwrap_or_default();

    // Technology: device doughnut + table.
    let tech = &data["technology"]["data"];
    let tech_max = &tech["max"];
    let tech_rows: Vec<MetricRow> = ["mobile", "tablet", "desktop"]
        .iter()
        .map(|d| metric_row(&capitalize(d), &tech[*d], tech_max))
        .collect();
    let tech_doughnut = pie_config(
        &json!({
            "labels": ["Mobile", "Tablet", "Desktop"],
            "datasets": [{
                "data": [
                    f(&tech["mobile"], "Unique Users"),
                    f(&tech["tablet"], "Unique Users"),
                    f(&tech["desktop"], "Unique Users"),
                ],
                "backgroundColor": ["#268699", "#f2c04b", "#7cb4bf"],
                "hoverOffset": 4,
            }],
        }),
        true,
    );

    // Audience: top states by unique users.
    let states = &data["states"]["data"];
    let states_max = &states["max"];
    let mut states_rows: Vec<MetricRow> = states["states"]
        .as_object()
        .map(|m| {
            m.iter()
                .filter(|(k, _)| !k.is_empty())
                .map(|(k, v)| metric_row(k, v, states_max))
                .collect()
        })
        .unwrap_or_default();
    states_rows.sort_by(|a, b| b.cells[0].pct.cmp(&a.cells[0].pct));
    states_rows.truncate(15);

    AnalyticsSections {
        eng_total_views: f(eng_self, "Views") as i64,
        eng_total_clicks: f(eng_self, "Clicks to Website") as i64,
        eng_line,
        eng_bars,
        traffic_line,
        traffic_pie: pie_config(pie, false),
        traffic_has_pie,
        traffic_rows,
        tech_doughnut,
        tech_rows,
        states_rows,
    }
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_data_overview.stpl")]
struct DataOverviewPage {
    orgs: Vec<OrgOpt>,
    selected_uid: String,
    selected_name: String,
    has_data: bool,
    metric_columns: Vec<String>,
    sections: AnalyticsSections,
    searches: Vec<(String, i64)>,
    searches_max: i64,
}

#[derive(Debug, Default, Deserialize)]
struct OverviewQuery {
    #[serde(default)]
    org: String,
}

#[handler]
pub async fn data_overview(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<OverviewQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };

    let orgs: Vec<OrgOpt> = state
        .api
        .get_json("/api/ui/organization/all", Some(&token))
        .await
        .unwrap_or_default();

    if orgs.is_empty() {
        return Ok(page(
            chrome,
            "Your Data Overview | Science Near Me",
            empty_overview(),
        )?
        .into_response());
    }

    let selected = orgs
        .iter()
        .find(|o| o.uid == q.org)
        .unwrap_or(&orgs[0]);
    let selected_uid = selected.uid.clone();
    let selected_name = selected.name.clone();

    // Fetch the org's compiled analytics for the default window.
    let path = format!(
        "/api/ui/organization/analytics?about={}&kind=0&period={}&status={}",
        selected_uid,
        crate::routes::my::urlencode(DEFAULT_PERIOD),
        crate::routes::my::urlencode(DEFAULT_STATUS),
    );
    let data: Value = state
        .api
        .get_json(&path, Some(&token))
        .await
        .unwrap_or(Value::Null);
    let has_data = data.get("engagement").is_some();
    let sections = build_sections(&data);

    // User searches (local metro searches) — org-level only.
    let searches: Vec<(String, i64)> = state
        .api
        .get_json::<Vec<(String, i64)>>("/api/ui/finder/metro-searches", Some(&token))
        .await
        .unwrap_or_default();
    let searches_max = searches.first().map(|(_, n)| *n).unwrap_or(1).max(1);

    Ok(page(
        chrome,
        "Your Data Overview | Science Near Me",
        DataOverviewPage {
            orgs,
            selected_uid,
            selected_name,
            has_data,
            metric_columns: METRIC_COLUMNS.iter().map(|s| s.to_string()).collect(),
            sections,
            searches,
            searches_max,
        },
    )?
    .into_response())
}

// ---------------------------------------------------------------------------
// /my/hosts-explorer — host comparison (analytics kind=1)
// ---------------------------------------------------------------------------

/// A host's row: opportunity counts + engagement, each with a bar percentage.
struct HostRow {
    name: String,
    total: i64,
    live: i64,
    total_pct: i64,
    live_pct: i64,
    views: i64,
    exits: i64,
    didits: i64,
    saves: i64,
    likes: i64,
    shares: i64,
    calendar_adds: i64,
    views_pct: i64,
    exits_pct: i64,
    didits_pct: i64,
    saves_pct: i64,
    likes_pct: i64,
    shares_pct: i64,
    calendar_pct: i64,
}

fn pct(v: f64, max: f64) -> i64 {
    ((v / max.max(1.0)) * 100.0).clamp(0.0, 100.0) as i64
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_hosts_explorer.stpl")]
struct HostsPage {
    orgs: Vec<OrgOpt>,
    selected_uid: String,
    selected_name: String,
    has_data: bool,
    total_hosts: i64,
    total_opportunities: i64,
    treemap: String,
    hosts: Vec<HostRow>,
}

#[handler]
pub async fn hosts_explorer(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<OverviewQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };

    let orgs: Vec<OrgOpt> = state
        .api
        .get_json("/api/ui/organization/all", Some(&token))
        .await
        .unwrap_or_default();

    let mut view = HostsPage {
        orgs,
        selected_uid: String::new(),
        selected_name: String::new(),
        has_data: false,
        total_hosts: 0,
        total_opportunities: 0,
        treemap: String::new(),
        hosts: Vec::new(),
    };

    if view.orgs.is_empty() {
        return Ok(page(chrome, "Hosts Explorer | Science Near Me", view)?.into_response());
    }

    let selected = view
        .orgs
        .iter()
        .find(|o| o.uid == q.org)
        .unwrap_or(&view.orgs[0]);
    view.selected_uid = selected.uid.clone();
    view.selected_name = selected.name.clone();

    let path = format!(
        "/api/ui/organization/analytics?about={}&kind=1&period={}&status={}",
        view.selected_uid,
        crate::routes::my::urlencode(DEFAULT_PERIOD),
        crate::routes::my::urlencode(DEFAULT_STATUS),
    );
    let resp: Value = state
        .api
        .get_json(&path, Some(&token))
        .await
        .unwrap_or(Value::Null);
    let data = &resp["data"];
    view.has_data = data.get("hosts").is_some();
    view.total_hosts = data.get("total_hosts").and_then(Value::as_i64).unwrap_or(0);
    view.total_opportunities = data
        .get("total_opportunities")
        .and_then(Value::as_i64)
        .unwrap_or(0);

    let max = &data["max"];
    let empty = Vec::new();
    let hosts = data["hosts"].as_array().unwrap_or(&empty);
    view.hosts = hosts
        .iter()
        .map(|h| HostRow {
            name: h.get("name").and_then(Value::as_str).unwrap_or("").to_string(),
            total: f(h, "total") as i64,
            live: f(h, "live") as i64,
            total_pct: pct(f(h, "total"), f(max, "total")),
            live_pct: pct(f(h, "live"), f(max, "live")),
            views: f(h, "views") as i64,
            exits: f(h, "opportunity_exits") as i64,
            didits: f(h, "didits") as i64,
            saves: f(h, "saves") as i64,
            likes: f(h, "likes") as i64,
            shares: f(h, "shares") as i64,
            calendar_adds: f(h, "calendar_adds") as i64,
            views_pct: pct(f(h, "views"), f(max, "views")),
            exits_pct: pct(f(h, "opportunity_exits"), f(max, "opportunity_exits")),
            didits_pct: pct(f(h, "didits"), f(max, "didits")),
            saves_pct: pct(f(h, "saves"), f(max, "saves")),
            likes_pct: pct(f(h, "likes"), f(max, "likes")),
            shares_pct: pct(f(h, "shares"), f(max, "shares")),
            calendar_pct: pct(f(h, "calendar_adds"), f(max, "calendar_adds")),
        })
        .collect();
    view.hosts.sort_by(|a, b| b.total.cmp(&a.total));

    // Treemap of hosts sized by total opportunities (chartjs-chart-treemap).
    if view.total_hosts > 0 {
        view.treemap = json!({
            "type": "treemap",
            "data": { "datasets": [{
                "tree": hosts,
                "key": "total",
                "groups": ["name"],
                "backgroundColor": "#268699",
                "borderColor": "#ffffff",
                "borderWidth": 1,
                "spacing": 1,
                "labels": { "display": true, "color": "#ffffff", "font": { "size": 11 } },
            }]},
            "options": { "plugins": { "legend": { "display": false } } }
        })
        .to_string();
    }

    Ok(page(chrome, "Hosts Explorer | Science Near Me", view)?.into_response())
}

// ---------------------------------------------------------------------------
// /my/snm-data-overview — public, SNM-wide dashboard
// ---------------------------------------------------------------------------

#[derive(Debug, Default, Deserialize)]
struct OppsOverview {
    #[serde(default)]
    total: i64,
    #[serde(default)]
    active: i64,
    #[serde(default)]
    inactive: i64,
    #[serde(default)]
    in_person: i64,
    #[serde(default)]
    online: i64,
    #[serde(default)]
    global: i64,
    #[serde(default)]
    regional: i64,
    #[serde(default)]
    at_point: i64,
    #[serde(default)]
    attribute: std::collections::HashMap<String, AttrGroup>,
    #[serde(default)]
    providers: Providers,
}

#[derive(Debug, Default, Deserialize)]
struct AttrGroup {
    max: AttrMax,
    #[serde(default)]
    rows: Vec<AttrRowRaw>,
}
#[derive(Debug, Default, Deserialize)]
struct AttrMax {
    // Null when the group is empty (backend `max()` → None).
    #[serde(default)]
    all: Option<i64>,
    #[serde(default)]
    current: Option<i64>,
}
#[derive(Debug, Default, Deserialize)]
struct AttrRowRaw {
    #[serde(default)]
    label: String,
    #[serde(default)]
    all: i64,
    #[serde(default)]
    current: i64,
}
#[derive(Debug, Default, Deserialize)]
struct Providers {
    #[serde(default)]
    max: Option<i64>,
    #[serde(default)]
    rows: Vec<ProviderRowRaw>,
}
#[derive(Debug, Default, Deserialize)]
struct ProviderRowRaw {
    #[serde(default)]
    label: String,
    #[serde(default)]
    value: i64,
}

/// A row with an absolute count + a bar percentage.
struct CountBar {
    label: String,
    value: i64,
    pct: i64,
}

/// An attribute group's table (all + current columns).
struct AttrTable {
    title: String,
    rows: Vec<AttrRowView>,
}
struct AttrRowView {
    label: String,
    all: i64,
    current: i64,
    all_pct: i64,
    current_pct: i64,
}

/// A demographic proportion row (0–1 proportion → percent label + bar).
struct PropRow {
    label: String,
    value: String,
    pct: i64,
}

struct SnmStat {
    label: String,
    value: i64,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_snm_data_overview.stpl")]
struct SnmPage {
    visits_total: i64,
    // opportunities
    opp_total: i64,
    opp_breakdown: Vec<CountBar>,
    attributes: Vec<AttrTable>,
    providers: Vec<CountBar>,
    // engagement
    stats: Vec<SnmStat>,
    searches: Vec<CountBar>,
    // demographics
    has_demographics: bool,
    gender_doughnut: String,
    female_pct: String,
    male_pct: String,
    age_rows: Vec<PropRow>,
    ethnicity_rows: Vec<PropRow>,
    education_rows: Vec<PropRow>,
    income_rows: Vec<PropRow>,
    children_rows: Vec<PropRow>,
}

/// Proportion (0–1) → a `PropRow` against an ordered (key, label) list.
fn prop_rows(group: &Value, items: &[(&str, &str)]) -> Vec<PropRow> {
    items
        .iter()
        .map(|(key, label)| {
            let p = group.get(*key).map(|v| f(v, "proportion")).unwrap_or(0.0);
            PropRow {
                label: label.to_string(),
                value: format!("{:.1}%", p * 100.0),
                pct: (p * 100.0).clamp(0.0, 100.0) as i64,
            }
        })
        .collect()
}

#[handler]
pub async fn snm_data_overview(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
) -> Result<Response, AppError> {
    // Public page, but build the chrome (logged-in or not).
    let token = crate::session::token_from_jar(jar);
    let chrome = crate::chrome::Chrome::for_request(&state, jar, req).await;

    let visits_total = state
        .api
        .get_json::<Value>("/api/ui/organization/sitedata", None)
        .await
        .ok()
        .and_then(|v| v.get("visits_total").and_then(Value::as_i64))
        .unwrap_or(0);

    let opps: OppsOverview = state
        .api
        .get_json("/api/ui/organization/opps-overview", token.as_deref())
        .await
        .unwrap_or_default();

    // SNM-wide compiled analytics (all-time).
    let report: Value = state
        .api
        .get_json(
            &format!(
                "/api/ui/organization/analytics?about=00000000-0000-0000-0000-000000000000&kind=0&period={}&status={}",
                crate::routes::my::urlencode("All Time"),
                crate::routes::my::urlencode(DEFAULT_STATUS),
            ),
            None,
        )
        .await
        .unwrap_or(Value::Null);

    // Opportunity breakdown bars (all relative to total).
    let total = opps.total;
    let opp_breakdown = vec![
        ("Active", opps.active),
        ("Inactive", opps.inactive),
        ("In person", opps.in_person),
        ("Online", opps.online),
        ("Global", opps.global),
        ("Regional", opps.regional),
        ("At a point", opps.at_point),
    ]
    .into_iter()
    .map(|(label, value)| CountBar {
        label: label.to_string(),
        value,
        pct: pct(value as f64, total as f64),
    })
    .collect();

    // Attribute tables in a stable order.
    let attributes = [("activity", "Activity"), ("domain", "Domain"), ("indoor", "Venue")]
        .iter()
        .filter_map(|(key, title)| {
            opps.attribute.get(*key).map(|g| AttrTable {
                title: title.to_string(),
                rows: g
                    .rows
                    .iter()
                    .map(|r| AttrRowView {
                        label: humanize(&r.label),
                        all: r.all,
                        current: r.current,
                        all_pct: pct(r.all as f64, g.max.all.unwrap_or(0) as f64),
                        current_pct: pct(r.current as f64, g.max.current.unwrap_or(0) as f64),
                    })
                    .collect(),
            })
        })
        .collect();

    let providers = opps
        .providers
        .rows
        .iter()
        .map(|r| CountBar {
            label: r.label.clone(),
            value: r.value,
            pct: pct(r.value as f64, opps.providers.max.unwrap_or(0) as f64),
        })
        .collect();

    // Engagement stats.
    let st = &report["engagement"]["data"]["stats"];
    let stats = [
        ("Unique visitors", "unique_visitors"),
        ("Accounts", "accounts"),
        ("Opportunity views", "opportunity_views"),
        ("Unique opportunity views", "opportunity_unique"),
        ("Clicks to website", "opportunity_exits"),
        ("\"I did this\"", "didits"),
        ("Saves", "saves"),
        ("Likes", "likes"),
        ("Shares", "shares"),
        ("Calendar adds", "calendar_adds"),
    ]
    .iter()
    .map(|(label, key)| SnmStat {
        label: label.to_string(),
        value: f(st, key) as i64,
    })
    .collect();

    let search_max = report["engagement"]["data"]["search_max"].as_f64().unwrap_or(1.0);
    let searches = report["engagement"]["data"]["searches"]
        .as_array()
        .map(|rows| {
            rows.iter()
                .map(|r| CountBar {
                    label: r.get("phrase").and_then(Value::as_str).unwrap_or("").to_string(),
                    value: f(r, "searches") as i64,
                    pct: pct(f(r, "searches"), search_max),
                })
                .collect()
        })
        .unwrap_or_default();

    // Demographics.
    let demo = &report["demographics"];
    let has_demographics = demo.is_object();
    let female = f(&demo["sex"]["female"], "proportion");
    let male = f(&demo["sex"]["male"], "proportion");
    let gender_doughnut = pie_config(
        &json!({
            "labels": ["Female", "Male"],
            "datasets": [{ "data": [female, male], "backgroundColor": ["#268699", "#7cb4bf"], "hoverOffset": 4 }],
        }),
        true,
    );
    let age_order = [
        "18-20", "21-24", "25-29", "30-34", "35-39", "40-44", "45-49", "50-54", "55-59",
        "60-64", "65+",
    ];
    let age = &demo["age"];
    let age_rows = age_order
        .iter()
        .filter(|a| age.get(**a).is_some())
        .map(|a| {
            let p = f(&age[*a], "proportion");
            PropRow {
                label: a.to_string(),
                value: format!("{:.1}%", p * 100.0),
                pct: (p * 100.0).clamp(0.0, 100.0) as i64,
            }
        })
        .collect();

    Ok(page(
        chrome,
        "SNM Data Overview | Science Near Me",
        SnmPage {
            visits_total,
            opp_total: total,
            opp_breakdown,
            attributes,
            providers,
            stats,
            searches,
            has_demographics,
            gender_doughnut,
            female_pct: format!("{:.0}%", female * 100.0),
            male_pct: format!("{:.0}%", male * 100.0),
            age_rows,
            ethnicity_rows: prop_rows(
                &demo["ethnicity"],
                &[
                    ("Cauc.", "Caucasian"),
                    ("Hisp", "Hispanic"),
                    ("Afr. Am.", "African American"),
                    ("Asian", "Asian"),
                    ("Other", "Other"),
                ],
            ),
            education_rows: prop_rows(
                &demo["education"],
                &[("No College", "No College"), ("College", "College"), ("Grad. Sch.", "Graduate School")],
            ),
            income_rows: prop_rows(
                &demo["income"],
                &[("$0-50k", "$0–50k"), ("$50-100k", "$50–100k"), ("$100-150k", "$100–150k"), ("$150k+", "$150k+")],
            ),
            children_rows: prop_rows(
                &demo["children"],
                &[("Some Children under 17", "Some children under 17"), ("No Children under 17", "No children under 17")],
            ),
        },
    )?
    .into_response())
}

/// Turn an enum code like "citizen_science" into "Citizen Science".
fn humanize(code: &str) -> String {
    code.split(['_', '-'])
        .map(capitalize)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => String::new(),
    }
}

/// The empty-state page (user manages no organizations).
fn empty_overview() -> DataOverviewPage {
    DataOverviewPage {
        orgs: Vec::new(),
        selected_uid: String::new(),
        selected_name: String::new(),
        has_data: false,
        metric_columns: METRIC_COLUMNS.iter().map(|s| s.to_string()).collect(),
        sections: AnalyticsSections::default(),
        searches: Vec::new(),
        searches_max: 1,
    }
}

// ---------------------------------------------------------------------------
// /my/opportunity-data-explorer — per-opportunity analytics (kind=0)
//
// Same shape as the org overview, but keyed by an opportunity uid; the selector
// lists the user's own opportunities.
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
struct OppOpt {
    uid: String,
    title: String,
}

#[derive(Debug, Deserialize)]
struct OppSearch {
    matches: Vec<OppOpt>,
}

#[derive(TemplateSimple)]
#[template(path = "pages/my_opportunity_data_explorer.stpl")]
struct OppExplorerPage {
    opps: Vec<OppOpt>,
    selected_uid: String,
    selected_title: String,
    has_data: bool,
    metric_columns: Vec<String>,
    sections: AnalyticsSections,
}

#[handler]
pub async fn opportunity_data_explorer(
    state: Data<&AppState>,
    jar: &CookieJar,
    req: &Request,
    Query(q): Query<OverviewQuery>,
) -> Result<Response, AppError> {
    let (chrome, token) = match require_user(&state, jar, req).await {
        Ok(v) => v,
        Err(redirect) => return Ok(redirect),
    };

    // The user's opportunities (live + non-current), alphabetical.
    let mut opps: Vec<OppOpt> = Vec::new();
    for current in ["true", "false"] {
        if let Ok(r) = state
            .api
            .get_json::<OppSearch>(
                &format!("/api/ui/finder/search?mine=true&current={current}&sort=alphabetical&per_page=500"),
                Some(&token),
            )
            .await
        {
            opps.extend(r.matches);
        }
    }

    let metric_columns = METRIC_COLUMNS.iter().map(|s| s.to_string()).collect();
    if opps.is_empty() {
        return Ok(page(
            chrome,
            "Opportunity Data Explorer | Science Near Me",
            OppExplorerPage {
                opps,
                selected_uid: String::new(),
                selected_title: String::new(),
                has_data: false,
                metric_columns,
                sections: AnalyticsSections::default(),
            },
        )?
        .into_response());
    }

    let selected = opps.iter().find(|o| o.uid == q.org).unwrap_or(&opps[0]);
    let selected_uid = selected.uid.clone();
    let selected_title = selected.title.clone();

    let path = format!(
        "/api/ui/organization/analytics?about={}&kind=0&period={}&status={}",
        selected_uid,
        crate::routes::my::urlencode(DEFAULT_PERIOD),
        crate::routes::my::urlencode(DEFAULT_STATUS),
    );
    let data: Value = state
        .api
        .get_json(&path, Some(&token))
        .await
        .unwrap_or(Value::Null);
    let has_data = data.get("engagement").is_some();
    let sections = build_sections(&data);

    Ok(page(
        chrome,
        "Opportunity Data Explorer | Science Near Me",
        OppExplorerPage {
            opps,
            selected_uid,
            selected_title,
            has_data,
            metric_columns,
            sections,
        },
    )?
    .into_response())
}
