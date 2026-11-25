(function () {
  var bar = document.getElementById("topbar");
  var toggle = document.getElementById("nav-toggle");
  if (!bar || !toggle) return;
  function siteT(key, english) {
    return window.siteT ? window.siteT(key, english) : english;
  }
  function setOpen(open) {
    bar.classList.toggle("nav-open", open);
    toggle.setAttribute("aria-expanded", open ? "true" : "false");
    toggle.textContent = open ? siteT("close", "Close") : siteT("menu", "Menu");
  }
  toggle.addEventListener("click", function () {
    setOpen(!bar.classList.contains("nav-open"));
  });
  document.addEventListener("keydown", function (e) {
    if (e.key === "Escape") setOpen(false);
  });
  window.addEventListener("site:i18n", function () {
    setOpen(bar.classList.contains("nav-open"));
  });
})();
