// Age-range island: adds a slider in front of each age number input (min/max),
// mirroring the old Buefy slider-plus-direct-input. Progressive enhancement —
// the number inputs remain and are the no-JS fallback and submitted value.
(function () {
  "use strict";

  function enhance(fieldset) {
    ["min_age", "max_age"].forEach(function (name) {
      var num = fieldset.querySelector('input[name="' + name + '"]');
      if (!num) return;

      var range = document.createElement("input");
      range.type = "range";
      range.min = "0";
      range.max = "120";
      range.step = "1";
      range.className = "age-slider";
      range.value = num.value || "0";
      num.parentNode.insertBefore(range, num);

      range.addEventListener("input", function () {
        num.value = range.value;
      });
      range.addEventListener("change", function () {
        num.value = range.value;
        // Fire change so the form's hx-trigger="change" re-runs the search.
        num.dispatchEvent(new Event("change", { bubbles: true }));
      });
      num.addEventListener("input", function () {
        range.value = num.value || "0";
      });
    });
  }

  document.addEventListener("DOMContentLoaded", function () {
    document.querySelectorAll('[data-island="age-range"]').forEach(enhance);
  });
})();
