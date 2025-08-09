use axum::{
    extract::{Request, State},
    http::HeaderMap,
    middleware::Next,
    response::{IntoResponse, Response},
};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};

use crate::{middleware::error::Error, shared_state::SharedState};

use super::error::TurnstileApiResponseError;

#[derive(Serialize)]
struct TurnstileFormData<'a> {
    secret: &'a str,
    response: &'a str,
    remoteip: &'a str,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct SiteverifyResponse {
    success: bool,

    // The below fields are only passed upon success
    // or if passed explicitly by the client.
    challenge_ts: Option<String>,
    hostname: Option<String>,
    error_codes: Option<Vec<TurnstileApiResponseError>>,
    action: Option<String>,
    cdata: Option<String>,
}

const SITEVERIFY_URL: &str = "https://challenges.cloudflare.com/turnstile/v0/siteverify";

pub async fn turnstile(
    State(state): State<SharedState>,
    mut headers: HeaderMap,
    request: Request,
    next: Next,
) -> Response {
    trace!("Turnstile middleware called!");

    let debug_mode = state.config.debug_mode;

    if debug_mode && headers.get("CF-Connecting-IP").is_none() {
        info!("Debug mode detected -- appending artificial CF-Connecting-IP header");

        headers.append(
            "CF-Connecting-IP",
            axum::http::HeaderValue::from_str(&std::net::Ipv4Addr::LOCALHOST.to_string()).unwrap(),
        );
    }

    let Some(cf_connecting_ip) = headers.get("cf-connecting-ip") else {
        info!("Missing header CF-Connecting-IP");

        return (StatusCode::BAD_REQUEST, "Missing header: cf-connecting-ip").into_response();
    };

    let Ok(cf_connecting_ip) = cf_connecting_ip.to_str() else {
        info!("Header CF-Connecting-IP is malformed");

        return (StatusCode::BAD_REQUEST, "CF-Connecting-IP is malformed").into_response();
    };

    let Some(cf_response) = headers.get("cf-turnstile-response") else {
        info!("Missing header cf-turnstile-response for {cf_connecting_ip}");

        return (
            StatusCode::BAD_REQUEST,
            "Missing header: cf-turnstile-response",
        )
            .into_response();
    };

    let Ok(cf_response) = cf_response.to_str() else {
        return (StatusCode::BAD_REQUEST, "Malformed turnstile response").into_response();
    };

    let cf_secret_key = state.secrets.cf_turnstile_secret;

    trace!("Turnstile Middleware: \n\t - cf_response: {cf_response}\n\t - cf_connecting_ip: {cf_connecting_ip}\n\t - cf_secret_key: {cf_secret_key}");

    match siteverify(cf_connecting_ip, cf_response, &cf_secret_key).await {
        Err(err) => {
            return match err {
                Error::SiteverifyResponse(turnstile_error) => match turnstile_error {
                    TurnstileApiResponseError::InvalidInputResponse => (
                        StatusCode::TOO_MANY_REQUESTS,
                        "Expired or illegitimate siteverify token",
                    ),
                    TurnstileApiResponseError::BadRequest => (
                        StatusCode::TOO_MANY_REQUESTS,
                        "Siteverify request malformed",
                    ),
                    TurnstileApiResponseError::TimeoutOrDuplicate => (
                        StatusCode::TOO_MANY_REQUESTS,
                        "Siteverify token already redeemed",
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Unable to verify the human",
                    ),
                }
                .into_response(),
                _ => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Unable to verify the human",
                )
                    .into_response(),
            }
        }
        Ok(result) => result,
    };

    // Invoke next layer!
    next.run(request).await
}

/// Performs a CAPTCHA validation using Cloudflare Turnstile's siteverify API.
async fn siteverify(remote_ip: &str, response: &str, secret: &str) -> Result<bool, Error> {
    trace!("Performing Turnstile Siteverify check for {remote_ip} ({response})");

    let form = TurnstileFormData {
        secret,
        response,
        remoteip: remote_ip,
    };

    let client = Client::new();

    let response = client
        .post(SITEVERIFY_URL)
        .form(&form)
        .send()
        .await
        .map_err(Error::SiteverifyRequest)?;

    let response_text = response.text().await.map_err(Error::SiteverifyRead)?;

    let response_parsed = serde_json::from_str::<SiteverifyResponse>(&response_text)
        .map_err(Error::SiteverifyParse)?;

    if !response_parsed.success {
        info!(
            "Turnstile siteverify failed for {remote_ip} ({})",
            form.response
        );

        return Err(Error::SiteverifyResponse(
            response_parsed.error_codes.unwrap()[0],
        ));
    }

    info!(
        "Turnstile siteverify succeeded for {remote_ip} ({})",
        form.response
    );

    Ok(true)
}
