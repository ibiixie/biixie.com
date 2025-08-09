#[derive(Debug, Clone)]
pub struct Config {
    pub debug_mode: bool,
    pub backend_port: u32,
    pub state_mutex_timeout: u64,
    pub request_body_limit: usize,
    pub rate_limit_count: u64,
    pub rate_limit_duration: u64,
    pub request_timeout: u64,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            debug_mode: Self::env("DEBUG").unwrap_or(false),
            backend_port: Self::env("BACKEND_PORT").unwrap_or(3000),
            state_mutex_timeout: Self::env("STATE_MUTEX_TIMEOUT").unwrap_or(30),
            request_body_limit: Self::env("REQUEST_BODY_LIMIT").unwrap_or(3072),
            rate_limit_count: Self::env("RATE_LIMIT_COUNT").unwrap_or(100),
            rate_limit_duration: Self::env("RATE_LIMIT_DURATION").unwrap_or(10 * 60),
            request_timeout: Self::env("REQUEST_TIMEOUT").unwrap_or(30),
        }
    }

    fn env<T: std::str::FromStr>(key: &str) -> Option<T> {
        let Ok(value) = std::env::var(key) else {
            return None;
        };

        let Ok(parsed) = value.parse::<T>() else {
            return None;
        };

        Some(parsed)
    }
}
