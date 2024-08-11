//! src/configuration.rs

#[derive(Debug)]
pub struct Configuration {
    database: DatabaseConfiguration,
}

#[derive(Debug)]
pub struct DatabaseConfiguration {
    host: String,
    port: u16,
    username: String,
    password: String,
    database: String,
}

impl Configuration {
    pub fn get_configuration() -> Configuration {
        todo!();
    }
}
