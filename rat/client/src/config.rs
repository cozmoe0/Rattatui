use crate::Error;
use base64::{Engine as _, engine::general_purpose};

pub const SERVER_URL: &str = "http://localhost:8080";
pub const IDENTITY_PRIVATE_KEY: &str = "wToLgDfjCxFijRA+YKi6T9j7bTc/4grwoTRJZJs5DU8=";

#[derive(Debug)]
pub struct Config {
    pub identity_public_key: ed25519_dalek::PublicKey,
    pub identity_private_key: ed25519_dalek::SecretKey,
}

impl Config {
    pub fn load() -> Result<Config, Error> {
        let private_key_bytes = general_purpose::STANDARD.decode(IDENTITY_PRIVATE_KEY)?;

        let identity_private_key = ed25519_dalek::SecretKey::from_bytes(
            private_key_bytes.as_slice().try_into()
                .map_err(|_| Error::Internal("Invalid key length".to_string()))?
        )?;
        let identity_public_key: ed25519_dalek::PublicKey = (&identity_private_key).into();

        Ok(Config {
            identity_private_key,
            identity_public_key,
        })
    }
}
