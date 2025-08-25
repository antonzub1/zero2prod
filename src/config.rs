use config::{Config, File, FileFormat};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub database_name: String,
    pub host: String,
    pub port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = Config::builder().add_source(
        File::new("config.yaml", FileFormat::Yaml)
    )
        .build()?;
    settings.try_deserialize::<Settings>()
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name
        )
    }
}
