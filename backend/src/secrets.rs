use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct Secrets {
    pub cf_turnstile_secret: String,
    pub redacted_terms: String,
}

impl Secrets {
    pub fn from_file() -> Self {
        Self {
            cf_turnstile_secret: Self::read("cf_turnstile_secret"),
            redacted_terms: Self::read("redacted_terms"),
        }
    }

    fn read(key: &str) -> String {
        let secrets_path = PathBuf::from(
            std::env::var("SECRETS_PATH").unwrap_or_else(|_| "/run/secrets/".to_string()),
        );

        std::fs::read_to_string(secrets_path.join(key))
            .unwrap_or_else(|_| panic!("missing secret '{key}'"))
    }
}
