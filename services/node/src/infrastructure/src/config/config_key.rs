pub enum ConfigKey {
    AuthRsaPublicKey,
    AuthRsaPrivateKey,
}

impl ConfigKey {
    pub fn to_str(&self) -> &'static str {
        match self {
            ConfigKey::AuthRsaPublicKey => "AUTH_RSA_PUBLIC_KEY",
            ConfigKey::AuthRsaPrivateKey => "AUTH_RSA_PRIVATE_KEY",
        }
    }
}
