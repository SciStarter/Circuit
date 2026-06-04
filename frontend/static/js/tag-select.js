// Tag-autocomplete island: enhances a checkbox-list facet (Topics / Activity
// Type) into a Buefy-style taginput — selected items show as removable chips,
// and a text field offers autocomplete suggestions. Progressive enhancement:
// it drives the underlying checkboxes (which still submit with the form and
// remain the no-JS fallback).
(function () {
  "use strict";

  function labelFor(cb) {
    var label = cb.closest("label");
    return label ? label.textContent.trim() : cb.value;
  }

  function enhance(fieldset) {
    var grid = fieldset.querySelector(".facet-options");
    if (!grid) return;
    var checkboxes = Array.prototype.slice.call(
      grid.querySelectorAll('input[type="checkbox"]')
    );
    if (!checkboxes.length) return;

    grid.hidden = true;
    fieldset.classList.add("tagselect-active");

    var box = document.createElement("div");
    box.className = "tagselect";
    var chips = document.createElement("div");
    chips.className = "tagselect-chips";
    var input = document.createElement("input");
    input.type = "text";
    input.className = "tagselect-input";
    input.autocomplete = "off";
    input.placeholder = fieldset.dataset.placeholder || "Type to filter…";
    var menu = document.createElement("div");
    menu.className = "tagselect-menu";
    menu.hidden = true;
    box.appendChild(chips);
    box.appendChild(input);
    box.appendChild(menu);
    fieldset.appendChild(box);

    function setChecked(cb, val) {
      cb.checked = val;
      // Fire change so the form's hx-trigger="change" re-runs the search.
      cb.dispatchEvent(new Event("change", { bubbles: true }));
      renderChips();
    }

    function renderChips() {
      chips.innerHTML = "";
      checkboxes
        .filter(function (cb) {
          return cb.checked;
        })
        .forEach(function (cb) {
          var chip = document.createElement("span");
          chip.className = "tagselect-chip";
          chip.appendChild(document.createTextNode(labelFor(cb)));
          var x = document.createElement("button");
          x.type = "button";
          x.className = "tagselect-x";
          x.setAttribute("aria-label", "Remove " + labelFor(cb));
          x.innerHTML = "&times;";
          x.addEventListener("click", function () {
            setChecked(cb, false);
          });
          chip.appendChild(x);
          chips.appendChild(chip);
        });
    }

    function renderMenu() {
      var q = input.value.trim().toLowerCase();
      var matches = checkboxes
        .filter(function (cb) {
          return !cb.checked && labelFor(cb).toLowerCase().indexOf(q) >= 0;
        })
        .slice(0, 8);
      menu.innerHTML = "";
      matches.forEach(function (cb) {
        var item = document.createElement("div");
        item.className = "tagselect-item";
        item.textContent = labelFor(cb);
        // mousedown (not click) so it fires before the input's blur.
        item.addEventListener("mousedown", function (e) {
          e.preventDefault();
          setChecked(cb, true);
          input.value = "";
          renderMenu();
          input.focus();
        });
        menu.appendChild(item);
      });
      menu.hidden = matches.length === 0;
    }

    input.addEventListener("input", renderMenu);
    input.addEventListener("focus", renderMenu);
    input.addEventListener("blur", function () {
      setTimeout(function () {
        menu.hidden = true;
      }, 150);
    });

    renderChips();
  }

  document.addEventListener("DOMContentLoaded", function () {
    document
      .querySelectorAll('[data-island="tag-select"]')
      .forEach(enhance);
  });
})();
