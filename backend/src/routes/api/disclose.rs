use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{secrets::Secrets, shared_state::SharedState};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DiscloseServiceState {
    redacted_terms: HashMap<String, String>,
}

impl DiscloseServiceState {
    /// Load secrets from a plaintext file in a "KEY=VALUE\n" format.
    pub fn load(secrets: &Secrets) -> Self {
        Self {
            redacted_terms: Self::load_kvp(&secrets.redacted_terms),
        }
    }

    fn load_kvp(redacted_terms: &str) -> HashMap<String, String> {
        let content = redacted_terms.trim().to_owned();
        let lines: Vec<String> = content.lines().map(std::convert::Into::into).collect();
        let pairs: Vec<(String, String)> = lines
            .iter()
            .map(|v| {
                let split: Vec<&str> = v.split('=').collect();
                let key = split[0];
                let value = split[1];

                (key.to_string(), value.to_string())
            })
            .collect();

        dbg!(&pairs);
        pairs.into_iter().collect()
    }
}

#[axum::debug_handler]
pub async fn handler(State(state): State<SharedState>) -> Json<HashMap<String, String>> {
    info!("Serving request for redacted terms");

    let Ok(shared_state) = state.get().await else {
        error!("Mutex lock acquisition timed out while processing disclose request");

        return Json(HashMap::new());
    };

    let kvps = &shared_state.disclose_api_state.redacted_terms;

    Json(kvps.clone())
}
