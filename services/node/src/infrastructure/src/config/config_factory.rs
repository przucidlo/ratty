use std::env;

use super::{config::Config, config_key::ConfigKey};

pub struct ConfigFactory {}

impl ConfigFactory {
    pub fn build() -> Config {
        Self::build_from_env()
    }

    fn build_from_env() -> Config {
        Config {
            auth_rsa_public_key: Self::env(ConfigKey::AuthRsaPublicKey).replace("/\\n/g", "\n"),
            auth_rsa_private_key: Self::env(ConfigKey::AuthRsaPrivateKey).replace("/\\n/g", "\n"),
        }
    }

    fn env(key: ConfigKey) -> String {
        let key = key.to_str();

        env::var(key).unwrap_or_else(|_a| panic!("No such key as {} found in env", key))
    }
}
