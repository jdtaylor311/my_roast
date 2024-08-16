//! src/routes/user.rs

use axum::response::IntoResponse;
pub(crate) use axum::{extract::State, routing::get, Router};

use crate::startup::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/users", get(get_users_handler))
}

pub async fn get_users_handler(State(state): State<AppState>) -> impl IntoResponse {
    "Hello world from handler"
}
