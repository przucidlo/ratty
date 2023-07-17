use openssl::rsa::Rsa;
use serde::Serialize;

#[derive(Serialize)]
pub struct Jwk {
    kty: String,
    key_ops: String,
    kid: String,
    n: String,
    e: String,

    #[serde(rename = "use")]
    purpose: String,
}

impl Jwk {
    pub fn from_rsa(public_key: &str, kid: &str) -> Jwk {
        let rsa = Rsa::public_key_from_pem(public_key.as_bytes()).expect("Invalid public key");
        let n = base64_url::encode(&rsa.n().to_string());
        let e = base64_url::encode(&rsa.e().to_string());

        Jwk {
            kty: "RSA".to_owned(),
            purpose: "enc".to_owned(),
            key_ops: "verify".to_owned(),
            kid: kid.to_owned(),
            n,
            e,
        }
    }
}
