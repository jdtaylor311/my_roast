//! src/configuration.rs
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::postgres::{PgConnectOptions, PgSslMode};
use std::env;
use std::path::Path;
pub fn get_configuration() -> Result<Configuration, config::ConfigError> {
    //Set base path to configurations
    let base_path = env::current_dir().expect("Cannot obtain current directory");
    let config_path = base_path.join(Path::new("configurations"));

    //get env
    let environment: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to get APP_ENVIRONMENT variable.");

    let environment_file = format!("{}.yaml", environment.as_str());
    println!("The environment_file is {}", environment_file);

    let settings = config::Config::builder()
        .add_source(config::File::from(config_path.join("base.yaml")))
        .add_source(config::File::from(config_path.join(environment_file)))
        .build()?;

    settings.try_deserialize::<Configuration>()
}

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    pub database: DatabaseConfiguration,
    pub application: ApplicationConfiguration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfiguration {
    host: String,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    port: u16,
    username: String,
    password: String,
    database: String,
    ssl_enabled: bool,
}
impl DatabaseConfiguration {
    pub fn with_db(&self) -> PgConnectOptions {
        self.without_db().database(&self.database)
    }
    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_option = match &self.ssl_enabled {
            true => PgSslMode::Require,
            _ => PgSslMode::Prefer,
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password)
            .ssl_mode(ssl_option)
    }
}
#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationConfiguration {
    pub host: String,
    pub port: u16,
}

enum Environment {
    Local,
    Development,
    Staging,
    Production,
}
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Development => "dev",
            Environment::Staging => "staging",
            Environment::Production => "production",
        }
    }
}
impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Environment::Local),
            "dev" => Ok(Environment::Development),
            "staging" => Ok(Environment::Staging),
            "production" => Ok(Environment::Production),
            other => Err(format!(
                "{} is not supported environment. Please use local, dev, staging, or production.",
                other
            )),
        }
    }
}
