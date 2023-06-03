#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub location: String,
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
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::new(
            "configuration.yaml",
            config::FileFormat::Yaml,
        ))
        .build()?;
    settings.try_deserialize::<Settings>()
}
