use secrecy::{ExposeSecret, Secret};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

/// Possible runtime environment for our application
pub enum Environment {
    Development,
    Production,
}

///
impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Development => "development",
            Environment::Production => "production",
        }
    }
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

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
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
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let config_dir = base_path.join("config");

    let environment: Environment = std::env::var("RUST_ENV")
        .unwrap_or_else(|_| "development".into())
        .try_into()
        .expect("Failed to parse RUST_ENV");
    let environment_filename = format!("{}.yml", environment.as_str());
    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.yml")))
        .add_source(config::File::from(config_dir.join(&environment_filename)))
        // .add_source(config::File::with_name("src/config/config.yml"))
        .build()?;

    settings.try_deserialize::<Settings>()
}

impl TryFrom<String> for Environment {
    type Error = String;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "development" => Ok(Self::Development),
            "production" => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. Ue either `development` or `production`",
                other
            )),
        }
    }
}
