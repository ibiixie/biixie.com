use serde::{Deserialize, Serialize};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TurnstileApiResponseError {
    MissingInputSecret,
    InvalidInputSecret,
    MissingInputResponse,
    InvalidInputResponse,
    BadRequest,
    TimeoutOrDuplicate,
    InternalError,
}

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    #[allow(dead_code)]
    SiteverifyRequest(reqwest::Error),
    #[allow(dead_code)]
    SiteverifyRead(reqwest::Error),
    #[allow(dead_code)]
    SiteverifyParse(serde_json::Error),
    SiteverifyResponse(TurnstileApiResponseError),
}
