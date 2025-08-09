let cfResponseToken = null;

function verifyAction(action) {
  if (cfResponseToken == null) {
    let cfSiteKey = document.documentElement.getAttribute("data-cf-sitekey");

    // Turnstile has not yet rendered.
    let turnstileDialog = document.getElementById("cf-turnstile-dialog");
    turnstileDialog.showModal();

    turnstile.render("#cf-turnstile-widget", {
      sitekey: cfSiteKey,
      callback: async function(token) {
        cfResponseToken = token;

        action(token);

        // Wait a bit so the user has time to see the verify result.
        await new Promise(resolve => setTimeout(resolve, 2000));
        turnstileDialog.close();
      },
    });
  } else if (turnstile.isExpired(cfResponseToken)) {
    // Turnstile token is expired - reset.
    turnstile.reset();

    action(cfResponseToken);
  }

  // We assume token was consumed in the action, so we reset it.
  turnstile.reset();
}
