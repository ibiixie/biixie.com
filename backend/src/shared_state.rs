use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::routes;

use crate::secrets::Secrets;

#[derive(Clone)]
pub struct SharedState {
    pub config: Config,
    pub secrets: Secrets,
    pub service_state: Arc<Mutex<ServiceState>>,
}

impl SharedState {
    pub fn new() -> Self {
        let config = Config::from_env();
        let secrets = Secrets::from_file();
        let service_state = Arc::new(Mutex::new(ServiceState::init(&secrets)));

        Self {
            config,
            secrets,
            service_state,
        }
    }

    pub async fn get(&self) -> Result<tokio::sync::MutexGuard<'_, ServiceState>, ()> {
        let state_mutex_timeout = self.config.state_mutex_timeout;

        let waiter =
            tokio::time::timeout(std::time::Duration::from_secs(state_mutex_timeout), async {
                (self.service_state).lock().await
            })
            .await;

        waiter.map_err(|_| {})
    }
}

#[derive(Clone)]
pub struct ServiceState {
    pub disclose_api_state: routes::api::disclose::DiscloseServiceState,
}

impl ServiceState {
    fn init(secrets: &Secrets) -> Self {
        Self {
            disclose_api_state: routes::api::disclose::DiscloseServiceState::load(secrets),
        }
    }
}
