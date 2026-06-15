// Viewer-local opportunity time rendering, ported from the old
// OpportunityTime.vue. The server can't know the visitor's timezone, so it
// emits the raw ISO start/end datetimes in data attributes and this island
// formats them in the browser's locale/timezone. Without JS, the server's
// fallback summary line remains.
(function () {
  "use strict";

  var EARLIEST = new Date("0001-01-01");
  var LATEST = new Date("9999-01-01");
  var DT = { month: "long", day: "numeric", year: "numeric", hour: "numeric", minute: "2-digit" };
  var T = { hour: "numeric", minute: "2-digit" };

  function parseList(json) {
    try {
      return (JSON.parse(json) || []).map(function (s) { return new Date(s); });
    } catch (e) {
      return [];
    }
  }

  // Build [start, end] pairs, mirroring OpportunityTime.vue's `pairs`.
  function pairs(starts, ends, hasEnd) {
    if (ends.length === starts.length) {
      return starts.map(function (s, i) { return [s, ends[i]]; });
    } else if (starts.length === 1) {
      var dt = starts[0];
      return hasEnd
        ? [[dt, new Date(dt.getTime() + 3600000)]]
        : [[dt, LATEST]];
    } else if (starts.length === 0 && ends.length === 1) {
      return [[EARLIEST, ends[0]]];
    } else {
      return starts.map(function (s) { return [s, new Date(s.getTime() + 3600000)]; });
    }
  }

  function upcoming(all) {
    var now = new Date();
    var sorted = all.slice().sort(function (a, b) { return a[0] - b[0] || a[1] - b[1]; });
    var futureStart = sorted.filter(function (p) { return p[0] > now; });
    if (futureStart.length) return futureStart;
    var futureEnd = sorted.filter(function (p) { return p[0] < now && p[1] > now; });
    if (futureEnd.length) return futureEnd;
    return sorted.length ? sorted : [[EARLIEST, LATEST]];
  }

  function display(pair) {
    var now = new Date();
    var a = pair[0], b = pair[1];
    if (b < now) return "Finished " + b.toLocaleString([], DT);
    if (a < now && b >= LATEST) return "Ongoing";
    if (a < now && b > now) return "Ongoing through " + b.toLocaleString([], DT);
    if (a.getFullYear() === b.getFullYear() && a.getMonth() === b.getMonth() && a.getDate() === b.getDate()) {
      return a.toLocaleString([], DT) + " through " + b.toLocaleString([], T);
    }
    return a.toLocaleString([], DT);
  }

  function render(el) {
    var starts = parseList(el.getAttribute("data-starts"));
    var ends = parseList(el.getAttribute("data-ends"));
    var hasEnd = el.getAttribute("data-has-end") === "true";
    if (!starts.length && !ends.length) return; // on-demand: keep the fallback

    var up = upcoming(pairs(starts, ends, hasEnd));
    var soonest = up.slice(0, 5);
    var ul = document.createElement("ul");
    ul.className = "opportunity-time";
    soonest.forEach(function (p) {
      var li = document.createElement("li");
      li.textContent = display(p);
      ul.appendChild(li);
    });
    if (up.length > soonest.length) {
      var dots = document.createElement("li");
      dots.textContent = "…";
      ul.appendChild(dots);
      var last = document.createElement("li");
      last.textContent = display(up[up.length - 1]);
      ul.appendChild(last);
    }
    el.innerHTML = "";
    el.appendChild(ul);
  }

  document.addEventListener("DOMContentLoaded", function () {
    document.querySelectorAll('[data-island="opp-time"]').forEach(render);
  });
})();
