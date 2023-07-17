use jsonwebtoken::errors::Error as JWTError;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::de::DeserializeOwned;
use serde::Serialize;

pub struct RsaJwtFactory {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl RsaJwtFactory {
    pub fn new(public_key: String, private_key: String) -> Self {
        Self {
            encoding_key: EncodingKey::from_rsa_pem(private_key.as_bytes())
                .expect("Invalid private key"),
            decoding_key: DecodingKey::from_rsa_pem(public_key.as_bytes())
                .expect("Invalid public key"),
        }
    }

    pub fn encode<P>(&self, payload: P) -> Result<String, JWTError>
    where
        P: Serialize,
    {
        jsonwebtoken::encode(&Header::new(Algorithm::RS256), &payload, &self.encoding_key)
    }

    pub fn decode<P>(&self, token: String) -> Result<TokenData<P>, JWTError>
    where
        P: DeserializeOwned,
    {
        jsonwebtoken::decode(
            &token,
            &self.decoding_key,
            &Validation::new(Algorithm::RS256),
        )
    }
}
