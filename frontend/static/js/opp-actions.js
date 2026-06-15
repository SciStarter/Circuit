// Action-bar enhancements on the entity detail page:
//  - fire best-effort analytics beacons (register-interest / shared / calendar)
//    via the /api proxy when a link is clicked, mirroring the old app;
//  - copy the public link to the clipboard for the "Copy link" share option.
// All links work without this script; the beacons are fire-and-forget.
(function () {
  "use strict";

  function slugOf(el) {
    var bar = el.closest("[data-slug]");
    return bar && bar.getAttribute("data-slug");
  }

  function beacon(slug, spec) {
    // spec is "interest" or "shared:facebook" / "calendar:google".
    var parts = spec.split(":");
    var kind = parts[0];
    var network = parts[1];
    var url = "/api/ui/entity/" + encodeURIComponent(slug) + "/" + kind;
    fetch(url, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: network ? JSON.stringify({ network: network }) : "{}",
      keepalive: true,
    }).catch(function () {});
  }

  document.addEventListener("click", function (e) {
    var copy = e.target.closest("[data-copy]");
    if (copy) {
      var text = copy.getAttribute("data-copy");
      if (navigator.clipboard) {
        navigator.clipboard.writeText(text).catch(function () {});
      }
      var prev = copy.textContent;
      copy.textContent = "Copied!";
      setTimeout(function () {
        copy.textContent = prev;
      }, 1500);
    }

    var b = e.target.closest("[data-beacon]");
    if (b) {
      var slug = slugOf(b);
      if (slug) beacon(slug, b.getAttribute("data-beacon"));
    }
  });
})();
