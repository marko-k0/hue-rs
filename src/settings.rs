use config::{Config, ConfigError, File, Environment};

#[derive(Debug, Deserialize)]
struct Hue {
    ip: String,
    username: String,
}

/// Configuration holding Phiplips Hue API information and debug option.
#[derive(Debug, Deserialize)]
pub struct Settings {
    debug: bool,
    hue: Hue,
}

impl Settings {
    /// Load configuration from environment or ~/.huerc configuration file.
    ///
    /// Environment variables have a priority over configuration file.
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();

        s.merge(File::with_name("~/.huerc").required(false));
        s.merge(Environment::with_prefix("HUE"))?;

        s.try_into()
    }

    /// Get IP address of Philips Hue bridge.
    pub fn ip(&self) -> &str {
        &self.hue.ip
    }

    /// Get username that is used to talk with the API.
    pub fn username(&self) -> &str {
        &self.hue.username
    }
}
