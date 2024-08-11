//! /src/main.rs
use my_roast::configuration;
use sqlx::migrate::Migrator;
use sqlx::postgres::PgConnectOptions;
use sqlx::PgPool;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration = configuration::Configuration::get_configuration();
    let connection_options = PgConnectOptions::new()
        .host("")
        .database("postgres")
        .port(5432)
        .username("")
        .password("");

    let pool = PgPool::connect_with(connection_options).await?;
    let migrator = Migrator::new(Path::new("./migrations")).await?;
    migrator.run(&pool).await?;

    Ok(())
}
