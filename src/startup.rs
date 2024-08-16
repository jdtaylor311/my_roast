//! src/startup.rs

use axum::Router;

use crate::{
    configuration::{self, Configuration},
    database::Database,
    routes::*,
};

#[derive(Clone)]
pub struct AppState {
    database: Database,
    config: Configuration,
}

pub async fn create_app() -> axum::Router {
    let config = configuration::get_configuration();
    let database = Database::new(&config.as_ref().unwrap().database);
    let state = AppState {
        database: database.unwrap(),
        config: config.unwrap(),
    };

    Router::new()
        .merge(users::router())
        .with_state(state)
        .merge(health_check::router())
}
