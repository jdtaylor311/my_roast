//! /src/main.rs
use my_roast::configuration::{self, DatabaseConfiguration};
use sqlx::{postgres::PgPoolOptions, PgPool};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let configuration =
        configuration::get_configuration().expect("Failed to retrieve configuration.");

    let pg_pool = get_connection_pool(configuration.database);

    sqlx::migrate!().run(&pg_pool).await?;

    Ok(())
}

fn get_connection_pool(database: DatabaseConfiguration) -> PgPool {
    PgPoolOptions::new().connect_lazy_with(database.with_db())
}
