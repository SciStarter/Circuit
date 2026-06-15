// "See on map" island: lazily loads self-hosted MapLibre GL (+ OpenFreeMap
// tiles, no token) the first time the map is revealed, then renders the
// opportunity's point marker or polygon. Loads nothing on pages without a map.
(function () {
  "use strict";

  var STYLE = "https://tiles.openfreemap.org/styles/liberty";
  var TEAL = "#087a91";
  var loaded = false;

  function loadLib(cb) {
    if (window.maplibregl) {
      cb();
      return;
    }
    var css = document.createElement("link");
    css.rel = "stylesheet";
    css.href = "/static/css/maplibre-gl.css";
    document.head.appendChild(css);
    var s = document.createElement("script");
    s.src = "/static/js/maplibre-gl.js";
    s.onload = cb;
    document.head.appendChild(s);
  }

  // Flatten arbitrarily nested coordinate arrays to [lng,lat] pairs.
  function coordsOf(geometry) {
    var out = [];
    (function walk(a) {
      if (typeof a[0] === "number") out.push(a);
      else a.forEach(walk);
    })(geometry.coordinates);
    return out;
  }

  function boundsOf(coords) {
    var lons = coords.map(function (c) { return c[0]; });
    var lats = coords.map(function (c) { return c[1]; });
    return [
      [Math.min.apply(null, lons), Math.min.apply(null, lats)],
      [Math.max.apply(null, lons), Math.max.apply(null, lats)],
    ];
  }

  var map = null;

  function initMap(el) {
    var geo;
    try {
      geo = JSON.parse(el.getAttribute("data-geojson"));
    } catch (e) {
      return;
    }
    var coords = coordsOf(geo);
    if (!coords.length) return;

    map = new maplibregl.Map({
      container: el,
      style: STYLE,
      center: coords[0],
      zoom: 13,
    });
    // Bottom-right so the controls don't sit under the modal's close button.
    map.addControl(new maplibregl.NavigationControl(), "bottom-right");

    map.on("load", function () {
      if (geo.type === "Point") {
        new maplibregl.Marker({ color: TEAL }).setLngLat(coords[0]).addTo(map);
      } else {
        map.addSource("shape", {
          type: "geojson",
          data: { type: "Feature", geometry: geo, properties: {} },
        });
        map.addLayer({ id: "shape-fill", type: "fill", source: "shape", paint: { "fill-color": TEAL, "fill-opacity": 0.25 } });
        map.addLayer({ id: "shape-line", type: "line", source: "shape", paint: { "line-color": TEAL, "line-width": 2 } });
        map.fitBounds(boundsOf(coords), { padding: 30, maxZoom: 14 });
      }
    });
  }

  function openModal() {
    var modal = document.getElementById("opp-map-modal");
    if (!modal) return;
    modal.hidden = false;
    var el = document.getElementById("opp-map");
    if (!loaded) {
      loaded = true;
      loadLib(function () { initMap(el); });
    } else if (map) {
      // The container was display:none while closed; recompute its size.
      map.resize();
    }
  }

  function closeModal() {
    var modal = document.getElementById("opp-map-modal");
    if (modal) modal.hidden = true;
  }

  document.addEventListener("click", function (e) {
    if (e.target.closest("[data-toggle-map]")) {
      e.preventDefault();
      openModal();
      return;
    }
    // Close on the close button or a click on the backdrop itself.
    if (e.target.closest("[data-close-map]") || e.target.id === "opp-map-modal") {
      closeModal();
    }
  });

  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") closeModal();
  });
})();
