// Global chrome behaviors: header menu/search toggles and the cookie notice.
// Progressive enhancement — all links/forms work without this script.
(function () {
  "use strict";

  // Mark that JS is active so CSS can hide no-JS-only controls (e.g. the
  // <summary> disclosure, since the quick-bar "More Filters" button toggles it).
  document.documentElement.classList.add("js");

  // Drive the shared HTMX activity spinner for non-HTMX work (e.g. the location
  // typeahead's plain fetch). Reference-counted, and uses its own `.active`
  // class so it doesn't collide with HTMX's `htmx-request` toggling.
  var activity = 0;
  window.snmActivity = function (on) {
    activity = Math.max(0, activity + (on ? 1 : -1));
    var spinner = document.getElementById("htmx-spinner");
    if (spinner) spinner.classList.toggle("active", activity > 0);
  };

  // Toggle the panel (search or mobile menu) named by data-toggle on click.
  function togglePanel(name) {
    var panel = document.querySelector('[data-panel="' + name + '"]');
    if (!panel) return;
    panel.classList.toggle("toggled");
    var btn = document.querySelector('[data-toggle="' + name + '"]');
    if (btn && btn.hasAttribute("aria-expanded")) {
      btn.setAttribute("aria-expanded", panel.classList.contains("toggled"));
    }
    if (name === "search" && panel.classList.contains("toggled")) {
      var first = panel.querySelector('input[name="text"]');
      if (first) first.focus();
    }
  }

  document.addEventListener("click", function (e) {
    var toggle = e.target.closest("[data-toggle]");
    if (toggle) {
      // The search toggle is also a real link to /find; only hijack it when
      // JS is available so the panel opens in place.
      if (toggle.dataset.toggle === "search") e.preventDefault();
      togglePanel(toggle.dataset.toggle);
      return;
    }

    // Links that act as POST actions (e.g. Log Out).
    var action = e.target.closest('a[data-method="post"]');
    if (action) {
      e.preventDefault();
      var form = document.createElement("form");
      form.method = "post";
      form.action = action.getAttribute("href");
      document.body.appendChild(form);
      form.submit();
      return;
    }

    if (e.target.closest("[data-cookie-consent]")) {
      window.localStorage.setItem("cookie-consent", "true");
      var notice = document.querySelector("[data-cookie-notice]");
      if (notice) notice.hidden = true;
    }

    // "More Filters" button (and the close button / backdrop) toggles the
    // slide-in filters drawer.
    if (e.target.closest("[data-toggle-facets]")) {
      var panel = document.getElementById("more-filters");
      var backdrop = document.querySelector(".more-filters-backdrop");
      if (panel) {
        var open = panel.classList.toggle("shown");
        if (backdrop) backdrop.hidden = !open;
      }
    }
  });

  // Quick-filter toggles: clicking the already-selected option clears it
  // (back to "everything"/"any time"), matching the old toggle behavior.
  document.addEventListener("mousedown", function (e) {
    var lbl = e.target.closest("label.qf-toggle");
    if (lbl) lbl._wasChecked = lbl.querySelector("input").checked;
  });
  document.addEventListener("click", function (e) {
    var lbl = e.target.closest("label.qf-toggle");
    if (!lbl || !lbl._wasChecked) return;
    var input = lbl.querySelector("input");
    input.checked = false;
    input.dispatchEvent(new Event("change", { bubbles: true }));
  });

  // Reveal the cookie notice unless already consented.
  document.addEventListener("DOMContentLoaded", function () {
    if (!window.localStorage.getItem("cookie-consent")) {
      var notice = document.querySelector("[data-cookie-notice]");
      if (notice) notice.hidden = false;
    }
  });
})();
