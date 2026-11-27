(function () {
  var bar = document.getElementById("topbar");
  var toggle = document.getElementById("nav-toggle");
  if (!bar || !toggle) return;
  function t(key, english) {
    var fn = window.siteT;
    if (typeof fn === "function") return fn(key, english);
    return english;
  }
  function setOpen(open) {
    bar.classList.toggle("nav-open", open);
    toggle.setAttribute("aria-expanded", open ? "true" : "false");
    toggle.textContent = open ? t("close", "Close") : t("menu", "Menu");
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
