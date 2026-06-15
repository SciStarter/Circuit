// Offline participation ("I did this" while logged out), ported from the old
// store's local_state / sync_local_to_server:
//  - anonymous "I did this" records the slug in localStorage and shows an
//    explainer prompting account creation;
//  - on the next authenticated page load, those records are POSTed to the
//    server and cleared (merge-on-login).
(function () {
  "use strict";

  function getLocal() {
    try {
      return JSON.parse(localStorage.getItem("local_state") || "{}");
    } catch (e) {
      return {};
    }
  }
  function setLocal(o) {
    localStorage.setItem("local_state", JSON.stringify(o));
  }
  function authenticated() {
    var p = document.getElementById("page");
    return !!p && p.classList.contains("authenticated");
  }

  function markDone(btn) {
    var icon = btn.querySelector(".oa-icon");
    if (icon) icon.classList.add("marked");
    var label = btn.querySelector(".oa-label");
    if (label) label.textContent = "You did this";
  }

  // Merge locally-recorded "I did this" into the account, then clear them.
  async function sync() {
    if (!authenticated()) return;
    var local = getLocal();
    var didit = local.didit || [];
    if (!didit.length) return;
    for (var i = 0; i < didit.length; i++) {
      try {
        await fetch("/api/ui/entity/" + encodeURIComponent(didit[i]) + "/didit", {
          method: "POST",
          headers: { "Content-Type": "application/json" },
          body: "{}",
        });
      } catch (e) {
        return; // leave the queue intact and retry on a later load
      }
    }
    local.didit = [];
    setLocal(local);
  }

  document.addEventListener("DOMContentLoaded", function () {
    sync();
    if (!authenticated()) {
      var didit = getLocal().didit || [];
      document.querySelectorAll("[data-offline-didit]").forEach(function (btn) {
        if (didit.indexOf(btn.getAttribute("data-offline-didit")) !== -1) markDone(btn);
      });
    }
  });

  document.addEventListener("click", function (e) {
    if (e.target.closest("[data-close-didit]")) {
      var m = document.getElementById("didit-explainer");
      if (m) m.hidden = true;
      return;
    }

    var btn = e.target.closest("[data-offline-didit]");
    if (!btn) return;
    e.preventDefault();
    var slug = btn.getAttribute("data-offline-didit");
    var local = getLocal();
    local.didit = local.didit || [];
    if (local.didit.indexOf(slug) === -1) local.didit.push(slug);
    setLocal(local);
    markDone(btn);
    var modal = document.getElementById("didit-explainer");
    if (modal) modal.hidden = false;
  });
})();
