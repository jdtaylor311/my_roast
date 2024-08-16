//! src/database.rs
use sqlx::{postgres::PgDatabaseError, PgPool};

use crate::configuration::DatabaseConfiguration;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub fn new(config: &DatabaseConfiguration) -> Result<Database, PgDatabaseError> {
        let options = DatabaseConfiguration::with_db(config);
        let pool = PgPool::connect_lazy_with(options);
        Ok(Self { pool })
    }
}
