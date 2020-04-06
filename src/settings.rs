use config::{Config, ConfigError, File};
use std::env;
use std::path::Path;

#[derive(Debug, Deserialize, Default)]
struct Hue {
    ip: String,
    username: String,
}

/// Configuration holding Phiplips Hue API information and debug option.
#[derive(Debug, Deserialize, Default)]
pub struct Settings {
    debug: bool,
    hue: Hue,
}

impl Settings {
    /// Load configuration from environment or ~/.huerc configuration file.
    ///
    /// Environment variables have a priority over configuration file.
    pub fn new(config_file: Option<&str>) -> Result<Self, ConfigError> {
        let mut s = Config::new();

        // defaults
        s.set_default("debug", false)?;

        // file config
        // XXX: really?
        let home_path = env::var("HOME").unwrap();
        let file_name = home_path + "/.huerc.ini";
        let file = config_file.unwrap_or(&file_name);
        s.merge(File::from(Path::new(file)).required(false))?;

        // environment variables
        if let (Ok(username), Ok(ip)) = (env::var("HUE_USERNAME"), env::var("HUE_IP")) {
            s.set("hue.username", username)?;
            s.set("hue.ip", ip)?;
        }

        // panic if we don't have config
        s.get_str("hue.ip").expect("Missing Hue IP configuration!");
        s.get_str("hue.username")
            .expect("Missing Hue Username configuration!");

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

#[cfg(test)]
mod tests {

    use super::*;

    static NONEXISTING_CONFIG: &'static str = "/tmp/file-that-doest-exists";

    // TODO: thread safety and env variables

    #[test]
    #[should_panic]
    fn missing_config() {
        Settings::new(Some(NONEXISTING_CONFIG)).unwrap();
    }

    #[test]
    fn env_config() {
        env::set_var("HUE_IP", "10.10.10.10");
        env::set_var("HUE_USERNAME", "HueGrant");

        let s = Settings::new(Some(NONEXISTING_CONFIG)).unwrap();
        assert_eq!(s.ip(), "10.10.10.10");
        assert_eq!(s.username(), "HueGrant");

        env::remove_var("HUE_IP");
        env::remove_var("HUE_USERNAME");
    }

    #[test]
    fn file_config() {
        //TODO: mock a file
    }

    #[test]
    fn file_and_env_config() {
        //TODO: mock a file
    }
}
