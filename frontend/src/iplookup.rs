/// Best-effort client IP from proxy headers (ingress sets `X-Forwarded-For`).
/// Returns only a publicly routable address; loopback/private addresses (e.g.
/// local dev) yield `None` so the backend falls back to caller-IP geolocation.
/// The actual geolocation + caching lives in the backend
/// (`GET /api/ui/finder/geolocate`).
pub fn client_ip(req: &poem::Request) -> Option<String> {
    let candidate = req
        .header("x-forwarded-for")
        .and_then(|v| v.split(',').next())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .or_else(|| req.header("x-real-ip").map(str::trim))
        .filter(|s| !s.is_empty())?;

    routable(candidate).then(|| candidate.to_string())
}

/// Rough check that an address isn't loopback or in a common private range.
fn routable(ip: &str) -> bool {
    !(ip.starts_with("127.")
        || ip == "::1"
        || ip.starts_with("10.")
        || ip.starts_with("192.168.")
        || ip.starts_with("169.254.")
        || ip.starts_with("fc")
        || ip.starts_with("fd"))
}
