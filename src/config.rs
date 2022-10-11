use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub adapter: String,
    pub username: String,
    pub password: Secret<String>,
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub test_db_prefix: String,
}

impl DatabaseSettings {
    pub fn connection_url(&self) -> Secret<String> {
        Secret::new(
            format!(
                "{}://{}:{}@{}:{}/{}",
                self.adapter,
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port,
                self.db_name
            ), // .to_string(),
        )
    }

    pub fn connection_url_nodb(&self) -> Secret<String> {
        Secret::new(
            format!(
                "{}://{}:{}@{}:{}/postgres",
                self.adapter,
                self.username,
                self.password.expose_secret(),
                self.host,
                self.port
            ), // .to_string(),
        )
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("src/config/config.yml"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
