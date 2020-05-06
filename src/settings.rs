use std::env;
use std::error::Error;
use std::fs::read_to_string;
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
    /// Load configuration from environment or ~/.huerc.ini configuration file.
    ///
    /// Environment variables have a priority over configuration file.
    pub fn new(config_file: Option<&str>) -> Result<Self, Box<dyn Error>> {
        // configuration file
        let default_config_file = env::var("HOME")? + "/.huerc.toml";
        let config = match config_file {
            Some(file_name) => read_to_string(Path::new(file_name))
                .expect(format!("File {} doesn't exists", file_name).as_ref()),
            None => read_to_string(Path::new(&default_config_file)).unwrap_or("".to_owned()),
        };
        let mut settings = toml::from_str(&config).unwrap_or(Settings::default());

        // environment variables
        if let (Ok(username), Ok(ip)) = (env::var("HUE_USERNAME"), env::var("HUE_IP")) {
            settings.hue.ip = ip;
            settings.hue.username = username;
        }
        if let Ok(debug) = env::var("DEBUG") {
            settings.debug = debug.parse().unwrap_or(false);
        }

        if settings.hue.ip.is_empty() || settings.hue.username.is_empty() {
            panic!("Missing Hue IP and/or Username Configuration!");
        }

        Ok(settings)
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

mod tests {

    use super::*;
    use std::fs::{remove_file, File};
    use std::io::{Read, Write};
    use std::sync::Mutex;

    lazy_static! {
        static ref ENV_RESOURCE: Mutex<()> = Mutex::new(());
    }

    #[test]
    #[should_panic]
    fn missing_config() {
        let tmp_file = "./.test_config_missing".to_owned();
        Settings::new(Some(&tmp_file)).unwrap();
    }

    #[test]
    fn env_config() {
        let _shared = ENV_RESOURCE.lock().unwrap();
        env::set_var("HUE_IP", "10.10.10.10");
        env::set_var("HUE_USERNAME", "HueGrant");

        let s = Settings::new(None).unwrap();
        assert_eq!(s.ip(), "10.10.10.10");
        assert_eq!(s.username(), "HueGrant");

        env::remove_var("HUE_IP");
        env::remove_var("HUE_USERNAME");
    }

    #[test]
    fn file_config() {
        let _shared = ENV_RESOURCE.lock().unwrap();

        let cfg = r#"
                  debug = false
                  [hue]
                  ip = '192.168.2.42'
                  username = 'huehue'"#;

        let tmp_file = "./.test_config".to_owned();
        let p = Path::new(&tmp_file);
        let mut f = File::create(p).unwrap();
        f.write_all(cfg.as_bytes()).unwrap();
        let settings = Settings::new(Some(&tmp_file.to_owned())).unwrap();

        assert_eq!(settings.debug, false);
        assert_eq!(settings.hue.ip, "192.168.2.42");
        assert_eq!(settings.hue.username, "huehue");

        remove_file(p).unwrap();
    }

    #[test]
    fn file_and_env_config() {
        let _shared = ENV_RESOURCE.lock().unwrap();

        let cfg = r#"
                  debug = true
                  [hue]
                  ip = '192.168.2.42'
                  username = 'huehue'"#;

        let tmp_file = "./.test_config".to_owned();
        let p = Path::new(&tmp_file);
        let mut f = File::create(p).unwrap();
        f.write_all(cfg.as_bytes()).unwrap();

        env::set_var("HUE_IP", "10.10.10.10");
        env::set_var("HUE_USERNAME", "HueGrant");

        let settings = Settings::new(Some(&tmp_file.to_owned())).unwrap();
        assert_eq!(settings.ip(), "10.10.10.10");
        assert_eq!(settings.username(), "HueGrant");
        assert_eq!(settings.debug, true);

        remove_file(p).unwrap();
        env::remove_var("HUE_IP");
        env::remove_var("HUE_USERNAME");
    }
}
