pub mod api;

use crate::shared_state::SharedState;
use axum::routing::get;
use axum::BoxError;
use axum::{error_handling::HandleErrorLayer, Router};
use reqwest::StatusCode;
use std::time::Duration;
use tower::buffer::BufferLayer;
use tower::limit::RateLimitLayer;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::limit::RequestBodyLimitLayer;
use tower_http::timeout::TimeoutLayer;

pub fn setup_routes(state: SharedState) -> Router {
    Router::new()
        .layer(axum::middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::turnstile,
        ))
        .layer(
            ServiceBuilder::new()
                .layer(TimeoutLayer::new(Duration::from_secs(
                    state.config.request_timeout,
                )))
                .layer(HandleErrorLayer::new(|err: BoxError| async move {
                    error!("Error ocurred in HandleErrorLayer: {err}");
                    (StatusCode::INTERNAL_SERVER_ERROR, "Something went wrong!")
                }))
                .layer(BufferLayer::new(1024))
                .layer(RateLimitLayer::new(
                    state.config.rate_limit_count,
                    Duration::from_secs(state.config.rate_limit_duration * 60),
                ))
                .layer(RequestBodyLimitLayer::new(state.config.request_body_limit))
                .layer(CorsLayer::permissive()),
        )
        .route("/api/disclose", get(api::disclose::handler))
        .with_state(state)
}
