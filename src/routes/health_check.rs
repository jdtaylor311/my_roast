//! src/routes/health_check.rs

use axum::{routing::get, Router};

pub fn router() -> Router {
    Router::new().route("/health_check", get(|| async { "OK" }))
}
