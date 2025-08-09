window.addEventListener('load', function () {
    document.querySelectorAll("nav a").forEach(element => {
        let elementText = element.getAttribute("href");
        let currentPageText = window.location.pathname;

        if (currentPageText.length > 1 && currentPageText.endsWith('/')) {
            currentPageText = currentPageText.substring(0, currentPageText.length - 1);
        }
    
        if (elementText == currentPageText) {
            element.classList.add("active");
        } else {
            element.classList.remove("active");
        }
    });
});
