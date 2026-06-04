// Location typeahead for the finder's "Near" field. As the user types, fetches
// place suggestions from the backend geocoder proxy (GET
// /api/ui/finder/geosuggest, OpenCage server-side — no client geosearch key
// needed) and shows a dropdown; selecting one fills the hidden
// longitude/latitude and triggers the finder's HTMX re-search.
(function () {
  "use strict";

  function debounce(fn, ms) {
    var t;
    return function () {
      var args = arguments, self = this;
      clearTimeout(t);
      t = setTimeout(function () { fn.apply(self, args); }, ms);
    };
  }

  function wire(island) {
    var near = island.querySelector('input[name="near"]');
    var lon = island.querySelector('input[name="longitude"]');
    var lat = island.querySelector('input[name="latitude"]');
    var prox = island.querySelector('[name="proximity"]'); // select (finder) or hidden (header)
    if (!near || !lon || !lat) return;

    var host = near.parentNode;
    if (getComputedStyle(host).position === "static") host.style.position = "relative";

    var menu = document.createElement("div");
    menu.className = "place-menu";
    menu.hidden = true;
    host.appendChild(menu);

    function hide() {
      menu.hidden = true;
      menu.innerHTML = "";
    }

    function choose(p) {
      near.value = p.near;
      lon.value = p.longitude;
      lat.value = p.latitude;
      if (prox) prox.value = "80467"; // selecting a place snaps distance to 50 miles
      hide();
      near.dispatchEvent(new Event("change", { bubbles: true }));
    }

    var search = debounce(function () {
      var q = near.value.trim();
      if (q.length < 3) { hide(); return; }
      if (window.snmActivity) window.snmActivity(true);
      fetch("/api/ui/finder/geosuggest?q=" + encodeURIComponent(q))
        .then(function (r) { return r.ok ? r.json() : null; })
        .then(function (d) {
          var places = (d && d.places) || [];
          if (!places.length) { hide(); return; }
          menu.innerHTML = "";
          places.forEach(function (p) {
            var item = document.createElement("div");
            item.className = "place-item";
            item.textContent = p.near;
            // mousedown (not click) so it fires before the input's blur.
            item.addEventListener("mousedown", function (e) {
              e.preventDefault();
              choose(p);
            });
            menu.appendChild(item);
          });
          menu.hidden = false;
        })
        .catch(hide)
        .finally(function () { if (window.snmActivity) window.snmActivity(false); });
    }, 300);

    near.addEventListener("input", function () {
      // Typing invalidates any previously chosen coordinates.
      lon.value = "";
      lat.value = "";
      if (near.value.trim() === "") {
        // Cleared location snaps distance to "Anywhere" (0 = no proximity
        // filter even if stale coords linger) and re-searches.
        if (prox) prox.value = "0";
        hide();
        near.dispatchEvent(new Event("change", { bubbles: true }));
      } else {
        search();
      }
    });
    near.addEventListener("focus", function () {
      if (near.value.trim().length >= 3 && menu.children.length) menu.hidden = false;
    });
    near.addEventListener("blur", function () {
      setTimeout(hide, 150);
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    document.querySelectorAll('[data-island="lookup-place"]').forEach(wire);
  });
})();
