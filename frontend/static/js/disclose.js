
document.querySelectorAll("[data-disclosed]").forEach((disclosedElement) => {
  disclosedElement.style.display = "none";
});

// If the user has already disclosed this session there is no need to redo it.
discloseRedactedsInCache();

function discloseRedactedsInCache() {
  let disclosedItems = sessionStorage.getItem("disclosedItems");
  if (disclosedItems == null) return;

  disclosedItems = JSON.parse(disclosedItems);

  for (const [key, value] of Object.entries(disclosedItems)) {
    discloseRedacted(key, value);
  }
}

document.querySelectorAll("#disclose-btn").forEach((element) => {
  element.onclick = () => {
    verifyAction(async (token) => {
      discloseAllRedacted(token);
    });
  };
});

async function discloseAllRedacted(token) {
  const REDACTED_TERMS = await queryRedacted(token);

  if (REDACTED_TERMS == null) {
    document.querySelectorAll("[data-redacted]").forEach((element) => {
      element.innerHTML = "Error disclosing redacted term!";
    });

    return;
  }

  REDACTED_TERMS.forEach((value, key, _map) => {
    discloseRedacted(key, value);
  });
}

async function discloseRedacted(key, value) {
  document.querySelectorAll("[data-redacted]").forEach((redactedElement) => {
    redactedElement.style.display = "none";
  });

  document.querySelectorAll("[data-disclosed]").forEach((disclosedElement) => {
    disclosedElement.style.display = "flex";
  });

  let htmlElement = document.querySelector("html");

  substituteAttrsRecursive(htmlElement, key, value);
  substituteTextRecursive(htmlElement, key, value);

  // Initialize disclosed items session storage if needed.
  let sessionDisclosedItems = sessionStorage.getItem("disclosedItems");
  if (sessionDisclosedItems == null) {
    sessionDisclosedItems = new Object();
  } else {
    sessionDisclosedItems = JSON.parse(sessionDisclosedItems);
  }

  // Add disclosed term to session storage.
  sessionDisclosedItems[key] = value;
  sessionStorage.setItem("disclosedItems", JSON.stringify(sessionDisclosedItems));
}

function substituteAttrsRecursive(html, key, value) {
  for (let i = 0; i < html.children.length; i++) {
    let child = html.children.item(i);

    substituteAttrsRecursive(child, key, value);

    child.getAttributeNames().forEach((attributeName) => {
      let attributeValue = child.getAttribute(attributeName);
      if (attributeValue != "") {
        attributeValue = attributeValue.replace(`@@${key}@@`, value);
        child.setAttribute(attributeName, attributeValue);
      }
    });
  }
}

function substituteTextRecursive(html, key, value) {
  html.childNodes.forEach((child) => {
    substituteTextRecursive(child, key, value);

    // A node type of 3 is a Text node.
    if (child.nodeType == 3) {
      let textValue = child.textContent;
      textValue = textValue.replace(`@@${key}@@`, value);
      child.textContent = textValue;
    }
  });
}

async function queryRedacted(token) {
  const URL = "/api/disclose";

  return await fetch(`${URL}`, {
    method: "GET",
    headers: {
      "CF-Turnstile-Response": token,
    },
  }).then(async (response) => {
    if (response.status == 200) {
      let jsonData = await response.json();
      return new Map(Object.entries(jsonData));
    } else {
      console.error("Unexpected response from server!");
      return null;
    }
  }).catch(async (e) => {
    console.error(e, e.stack);
    return null;
  });
}
