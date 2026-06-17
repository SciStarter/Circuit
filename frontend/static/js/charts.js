// Generic Chart.js island. Any `<canvas data-chart='<json>'>` on the page is
// hydrated into a chart; the JSON is a complete Chart.js config (type + data +
// options), built server-side. Chart.js is self-hosted and lazy-loaded only
// when a chart is actually present, so pages without charts ship no chart JS.
(function () {
  "use strict";

  function loadLib(cb) {
    if (window.Chart) { cb(); return; }
    if (window.__chartLoading) {
      window.__chartLoading.push(cb);
      return;
    }
    window.__chartLoading = [cb];
    var s = document.createElement("script");
    s.src = "/static/js/chart.umd.js";
    s.onload = function () {
      var cbs = window.__chartLoading || [];
      window.__chartLoading = null;
      cbs.forEach(function (fn) { fn(); });
    };
    document.head.appendChild(s);
  }

  function instantiate(el) {
    if (el.__charted) return;
    el.__charted = true;
    var cfg;
    try {
      cfg = JSON.parse(el.getAttribute("data-chart"));
    } catch (e) {
      return;
    }
    // Sensible shared defaults; the server config can override.
    cfg.options = cfg.options || {};
    if (cfg.options.responsive === undefined) cfg.options.responsive = true;
    if (cfg.options.maintainAspectRatio === undefined) {
      cfg.options.maintainAspectRatio = false;
    }
    new window.Chart(el, cfg);
  }

  // Load the self-hosted treemap controller and register it (some builds
  // auto-register; the explicit call is a harmless safety net), then continue.
  function withTreemap(cb) {
    if (window.__treemapReady) { cb(); return; }
    var s = document.createElement("script");
    s.src = "/static/js/chartjs-chart-treemap.umd.js";
    s.onload = function () {
      try {
        if (window.ChartTreemap) {
          window.Chart.register(
            window.ChartTreemap.TreemapController,
            window.ChartTreemap.TreemapElement
          );
        }
      } catch (e) { /* already registered */ }
      window.__treemapReady = true;
      cb();
    };
    document.head.appendChild(s);
  }

  function draw() {
    var els = document.querySelectorAll("canvas[data-chart]");
    if (!els.length) return;
    var needsTreemap = Array.prototype.some.call(els, function (el) {
      return /"type"\s*:\s*"treemap"/.test(el.getAttribute("data-chart") || "");
    });
    loadLib(function () {
      var go = function () {
        els.forEach(instantiate);
      };
      if (needsTreemap) withTreemap(go);
      else go();
    });
  }

  document.addEventListener("DOMContentLoaded", draw);
})();
