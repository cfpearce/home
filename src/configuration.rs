use secrecy::Secret;

const LOCAL: &str = "local";
const PRODUCTION: &str = "production";

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub location: String,
    pub password: Secret<String>,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!("sqlite://{}", self.location)
    }

    pub fn connection_temporary(&self) -> String {
        "".to_string()
    }
}

#[derive(serde::Deserialize)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application: ApplicationSettings,
}

pub enum Environment {
    Local,
    Production,
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => LOCAL,
            Environment::Production => PRODUCTION,
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            LOCAL => Ok(Self::Local),
            PRODUCTION => Ok(Self::Production),
            other => Err(format!(
                "{} is not a supported environment. \
                Use either 'local' or 'production'.",
                other
            )),
        }
    }
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir()
        .expect("Failed to determine the current directory");
    let configuration_directory = base_path.join("configuration");

    let environmnet: Environment = std::env::var("APP_ENVIRONMENT")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENVIRONMENT");

    let environment_filename = format!("{}.yaml", environmnet.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(
            configuration_directory.join("base.yaml"),
        ))
        .add_source(config::File::from(
            configuration_directory.join(environment_filename),
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
