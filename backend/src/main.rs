#![allow(clippy::cargo_common_metadata)]
#![allow(clippy::multiple_crate_versions)]

mod config;
mod middleware;
mod routes;
mod secrets;
mod shared_state;

#[macro_use]
extern crate log;

use std::str::FromStr;

#[tokio::main]
async fn main() {
    let log_level = std::env::var("BACKEND_LOG_LEVEL").unwrap_or("Info".into());

    simplelog::TermLogger::init(
        log::LevelFilter::from_str(&log_level)
            .expect("BACKEND_LOG_LEVEL exists in environment but is malformed"),
        simplelog::Config::default(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
    .unwrap();

    info!("Starting backend server");
    info!("Log level set to {}", log::max_level());

    let state: shared_state::SharedState = shared_state::SharedState::new();
    let port = state.config.backend_port;
    let router = routes::setup_routes(state);
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .unwrap();

    axum::serve(listener, router)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    info!("Stopped backend server");
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C shutdown handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install termination signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    };
}
 
