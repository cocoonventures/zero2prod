#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct DatabaseSettings {
    pub adapter: String,
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub db_name: String,
    pub test_db_prefix: String,
}

impl DatabaseSettings {
    pub fn connection_url(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.adapter, self.username, self.password, self.host, self.port, self.db_name
        )
    }

    pub fn connection_url_nodb(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/postgres",
            self.adapter, self.username, self.password, self.host, self.port
        )
    }
}

pub fn get_config() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::with_name("src/config/config.yml"))
        .build()?;

    settings.try_deserialize::<Settings>()
}
