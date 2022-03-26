use std::collections::BTreeMap;

use hmac::{Hmac, Mac};
use jwt::error::Error;
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;

#[derive(Clone)]
pub struct Authenticator {
    key: Hmac<Sha256>,
}

impl Authenticator {
    pub fn new(secret: String) -> Self {
        Authenticator {
            key: Hmac::new_from_slice(secret.as_bytes()).unwrap(),
        }
    }

    pub fn create_token(&self, user_id: &u64) -> String {
        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id.to_string());

        claims
            .sign_with_key(&self.key)
            .expect("token claims signing error")
    }

    pub fn extract_user_id_from_token(&self, token: String) -> Option<u64> {
        let claims: Result<BTreeMap<String, String>, Error> = token.verify_with_key(&self.key);
        if claims.is_err() {
            return None;
        } else {
            return claims
                .unwrap()
                .get("user_id")
                .map(|string_id| string_id.parse::<u64>().expect("invalid user id type"));
        }
    }
}