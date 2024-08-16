//! /src/main.rs
use std::ops::BitAnd;

use my_roast::startup::create_app;
use my_roast::{
    configuration::{self, DatabaseConfiguration},
    startup::{self},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration =
        configuration::get_configuration().expect("Failed to retrieve configuration.");

    let pg_pool = get_connection_pool(&configuration.database);

    sqlx::migrate!().run(&pg_pool).await?;
    let server = create_app().await;

    let bind_string = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );
    let listener = TcpListener::bind(bind_string).await.unwrap();

    axum::serve(listener, server).await;

    Ok(())
}

fn get_connection_pool(database: &DatabaseConfiguration) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(database.with_db())
}
