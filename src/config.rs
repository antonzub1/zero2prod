use config::{Config, Environment, File};
use secrecy::{ExposeSecret, SecretString};
use serde_aux::field_attributes::deserialize_number_from_string;
use serde::Deserialize;
use sqlx::postgres::{PgConnectOptions, PgSslMode};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: SecretString,
    pub database_name: String,
    pub host: String,
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub port: u16,
    pub require_ssl: bool
}

#[derive(Debug, Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with="deserialize_number_from_string")]
    pub host: String,
    pub port: u16,
}

pub enum Env {
    Local,
    Production
}

impl Env {
    pub fn as_str(&self) -> &str {
        match self {
            Env::Local => "local",
            Env::Production => "production",
        }
    }
}

impl TryFrom<String> for Env {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "local" => Ok(Env::Local),
            "production" => Ok(Env::Production),
            other => Err(
                format!("\"{}\"is not a supported environment. Use \"local\" or \"production\".", other)
            )
        }    
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    let env: Env = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");

    let env = format!("{}.yaml", env.as_str());

    let settings = Config::builder()
        .add_source(File::from(config_dir.join("base.yaml")))
        .add_source(File::from(config_dir.join(env)))
        .add_source(Environment::with_prefix("APP").prefix_separator("_").separator("__"))
        .build()?;
    let settings = settings.try_deserialize::<Settings>()?;
    tracing::debug!("Running configuration: {:?}", settings);
    Ok(settings)
}

impl DatabaseSettings {
    pub fn connection_options(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Prefer
        };
        PgConnectOptions::new()
            .host(&self.host)
            .port(self.port)
            .username(&self.username)
            .password(&self.password.expose_secret())
            .database(&self.database_name)
            .ssl_mode(ssl_mode)
    }
}
