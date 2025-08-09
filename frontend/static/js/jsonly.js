// Unhide elements that should only be shown if JS is enabled.
document.querySelectorAll('.jsonly').forEach((elem) => {
  elem.style.display = "flex";
});
