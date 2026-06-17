// Interactivity for the add/edit opportunity form. Section tabs and the
// location/timing reveals are pure CSS (:has); this handles the pieces that
// need scripting: the address typeahead (point capture), repeatable time
// periods, image upload, tag suggestions, and the facet filter boxes. All
// degrade to plain inputs when JS is unavailable.
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

  // --- Address typeahead -> location_name + point_lng/point_lat ---
  function wirePlace(box) {
    var name = box.querySelector('input[name="location_name"]');
    var lng = box.querySelector('input[name="point_lng"]');
    var lat = box.querySelector('input[name="point_lat"]');
    if (!name || !lng || !lat) return;

    var host = name.parentNode;
    if (getComputedStyle(host).position === "static") host.style.position = "relative";
    var menu = document.createElement("div");
    menu.className = "place-menu";
    menu.hidden = true;
    host.appendChild(menu);

    function hide() { menu.hidden = true; menu.innerHTML = ""; }

    var search = debounce(function () {
      var q = name.value.trim();
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
            item.addEventListener("mousedown", function (e) {
              e.preventDefault();
              name.value = p.near;
              lng.value = p.longitude;
              lat.value = p.latitude;
              hide();
            });
            menu.appendChild(item);
          });
          menu.hidden = false;
        })
        .catch(hide)
        .finally(function () { if (window.snmActivity) window.snmActivity(false); });
    }, 300);

    name.addEventListener("input", function () {
      // Typing a fresh query invalidates the previously chosen point.
      lng.value = "";
      lat.value = "";
      search();
    });
    name.addEventListener("blur", function () { setTimeout(hide, 150); });
  }

  // --- Repeatable time periods ---
  function wirePeriods(form) {
    var list = form.querySelector(".period-list");
    var add = form.querySelector(".add-period");
    if (!list || !add) return;

    add.addEventListener("click", function () {
      var rows = list.querySelectorAll(".period-row");
      var clone = rows[rows.length - 1].cloneNode(true);
      clone.querySelectorAll("input").forEach(function (i) { i.value = ""; });
      list.appendChild(clone);
    });

    list.addEventListener("click", function (e) {
      var btn = e.target.closest(".remove-period");
      if (!btn) return;
      var rows = list.querySelectorAll(".period-row");
      if (rows.length > 1) btn.closest(".period-row").remove();
      else btn.closest(".period-row").querySelectorAll("input").forEach(function (i) { i.value = ""; });
    });
  }

  // --- Image upload ---
  function wireUpload(form) {
    var file = form.querySelector("#image_upload");
    var url = form.querySelector("#image_url");
    var preview = form.querySelector(".image-preview");
    var status = form.querySelector("#upload_status");
    if (!file || !url) return;

    file.addEventListener("change", function () {
      if (!file.files || !file.files[0]) return;
      var data = new FormData();
      data.append("file", file.files[0]);
      if (status) { status.hidden = false; status.textContent = "Uploading…"; }
      if (window.snmActivity) window.snmActivity(true);
      fetch("/api/upload", { method: "POST", body: data })
        .then(function (r) { return r.ok ? r.json() : null; })
        .then(function (d) {
          var u = Array.isArray(d) ? d[0] : (d && d.url);
          if (u) {
            url.value = u;
            if (preview) preview.src = u;
            if (status) status.textContent = "Uploaded";
          } else if (status) {
            status.textContent = "Upload failed";
          }
        })
        .catch(function () { if (status) status.textContent = "Upload failed"; })
        .finally(function () { if (window.snmActivity) window.snmActivity(false); });
    });

    // Keep the preview in sync when the URL is typed/pasted.
    if (preview) {
      url.addEventListener("input", function () {
        if (url.value.trim()) preview.src = url.value.trim();
      });
    }
  }

  // --- Tag suggestion chips ---
  function wireTags(form) {
    var input = form.querySelector("#tags-input");
    if (!input) return;
    form.querySelectorAll(".tag-chip").forEach(function (chip) {
      chip.addEventListener("click", function () {
        var tag = chip.getAttribute("data-tag");
        var have = input.value.split(",").map(function (t) { return t.trim(); }).filter(Boolean);
        if (have.indexOf(tag) < 0) {
          have.push(tag);
          input.value = have.join(", ");
        }
      });
    });
  }

  // --- Facet filter boxes ---
  function wireFilters(form) {
    form.querySelectorAll(".facet-filter").forEach(function (box) {
      var target = form.querySelector(box.getAttribute("data-target"));
      if (!target) return;
      box.addEventListener("input", function () {
        var q = box.value.trim().toLowerCase();
        target.querySelectorAll("[data-label]").forEach(function (label) {
          label.style.display = label.getAttribute("data-label").indexOf(q) >= 0 ? "" : "none";
        });
      });
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    var form = document.querySelector(".opp-form");
    if (!form) return;
    form.querySelectorAll("[data-place-lookup]").forEach(wirePlace);
    wirePeriods(form);
    wireUpload(form);
    wireTags(form);
    wireFilters(form);
  });
})();
