let prefersDark = (window.matchMedia('(prefers-color-scheme: dark)').matches);
setTheme(prefersDark ? 'dark' : 'light');

function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
}

function getTheme() {
    let theme = document.documentElement.getAttribute('data-theme');
    return theme == null ? 'dark' : theme;
}