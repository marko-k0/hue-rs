use std::error::Error;
use std::time::Duration;
pub mod lights;

extern crate config;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate custom_derive;
#[macro_use]
extern crate derive_builder;

mod settings;
use settings::Settings;

type Res<T> = Result<T, Box<std::error::Error>>;

pub trait HTTPClient {
    fn get(&self, call: &str) -> Result<String, Box<Error>>;
    fn post(&self, call: &str, body: String) -> Result<String, Box<Error>>;
    fn put(&self, call: &str, body: String) -> Result<String, Box<Error>>;
    fn delete(&self, call: &str) -> Result<String, Box<Error>>;
}

#[derive(Debug)]
pub struct Client {
    settings: Settings,
    client: reqwest::Client,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            settings: Settings::new(None).unwrap(),
            client: reqwest::Client::new(),
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Client {
            settings: Settings::new(None).unwrap(),
            client: reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .danger_accept_invalid_certs(true)
                .build().unwrap()
        }
    }

    fn rest_call_url(&self, suffix: &str) -> String {
        format!("https://{}/api/{}/{}", self.settings.ip(), self.settings.username(), suffix)
    }
}

impl HTTPClient for Client {
    fn get(&self, call: &str) -> Result<String, Box<Error>> {
        Ok(self.client.get(self.rest_call_url(call).as_str()).send()?.text()?)
    }

    fn post(&self, call: &str, body: String) -> Result<String, Box<Error>> {
        Ok(self.client.post(self.rest_call_url(call).as_str()).body(body).send()?.text()?)
    }

    fn put(&self, call: &str, body: String) -> Result<String, Box<Error>> {
        Ok(self.client.put(self.rest_call_url(call).as_str()).body(body).send()?.text()?)
    }

    fn delete(&self, call: &str) -> Result<String, Box<Error>> {
        Ok(self.client.delete(self.rest_call_url(call).as_str()).send()?.text()?)
    }
}

mod test_common {

    use super::*;

    #[derive(Default, Debug)]
    pub struct HTTPClientMock {
        pub body: Option<String>,
        pub return_string: Option<String>,
        pub error: Option<String>,
    }

    impl HTTPClient for HTTPClientMock {
        fn get(&self, _: &str) -> Result<String, Box<Error>> {
            if let Some(s) = self.return_string.as_ref() {
                Ok(s.to_owned())
            } else {
                let e_str = self.error.as_ref().unwrap().clone();
                Err(e_str.into())
            }
        }

        fn post(&self, _: &str, _: String) -> Result<String, Box<Error>> {
            Ok("".to_owned())
        }

        fn put(&self, _: &str, _: String) -> Result<String, Box<Error>> {
            Ok("".to_owned())
        }

        fn delete(&self, _: &str) -> Result<String, Box<Error>> {
            Ok("".to_owned())
        }
    }
}
