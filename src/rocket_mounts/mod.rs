extern crate base64;

mod api;
mod monitor;
mod static_resources;

use std::ops::Deref;
use std::time::Duration;

use crate::rocket::{config::Environment, Config};

use crate::rand::{self, RngCore};

#[derive(Debug)]
struct DetectInterval(Duration);

impl DetectInterval {
    #[inline]
    fn get_value(&self) -> Duration {
        self.0
    }
}

impl Deref for DetectInterval {
    type Target = Duration;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
struct AuthKey(Option<String>);

impl AuthKey {
    #[inline]
    fn get_value(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

pub fn launch(
    monitor: Duration,
    address: String,
    listen_port: u16,
    auth_key: Option<String>,
    only_api: bool,
) {
    let mut config = Config::build(if cfg!(debug_assertions) {
        Environment::Development
    } else {
        Environment::Production
    });

    let mut secret_key = [0u8; 32];

    rand::thread_rng().fill_bytes(&mut secret_key);

    config.secret_key = Some(base64::encode(&secret_key));

    config.address = address;

    config.port = listen_port;

    let rocket =
        rocket::custom(config.unwrap()).manage(DetectInterval(monitor)).manage(AuthKey(auth_key));

    let rocket = api::mounts(rocket);

    let rocket = if only_api {
        rocket
    } else {
        let rocket = static_resources::rocket_handler(rocket);

        monitor::rocket_handler(rocket)
    };

    rocket.launch();
}
