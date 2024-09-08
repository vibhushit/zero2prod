//! src/configuration.rs

#[derive(serde::Deserialize)]
pub struct Settings{
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings{
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

pub fn get_configuration() -> Result<Settings, config::ConfigError> {
    //Initialise our configuration reader
    let mut settings = config::Config::default();

    //Add configurations values from a file named configurations
    //It will look for any top-level file with an extension
    //that `config` kknows how to parse: yaml, json, etc.
    settings.merge(config::File::with_name("configuration"))?;

    //Try to convert the configuration value it read into
    //our Settings type
    settings.try_into()
}