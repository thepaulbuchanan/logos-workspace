pub mod verification;
pub mod cursor;

use axum::{routing::post, Router};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use crate::config::AppState;

pub fn build_application_router(shared_state: Arc<AppState>) -> Router {
    let cors_policy = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([axum::http::Method::POST])
        .allow_headers([axum::http::HeaderName::from_static("content-type")]);

    Router::new()
        .route("/api/verify", post(verification::handle_web_verification))
        .route("/api/cursor", post(cursor::handle_cursor_sync))
        .layer(cors_policy)
        .with_state(shared_state)
}