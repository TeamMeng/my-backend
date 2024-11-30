use crate::{AppError, User};
use jwt_simple::prelude::*;

const JWT_DURATION: u64 = 30 * 24 * 60 * 60;
const JWT_ISSUER: &str = "server";
const JWT_AUDIENCE: &str = "web";

pub struct EncodingKey(Ed25519KeyPair);

pub struct DecodingKey(Ed25519PublicKey);

impl EncodingKey {
    pub fn new(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519KeyPair::from_pem(pem)?))
    }

    pub fn sign(&self, user: User) -> Result<String, AppError> {
        let claims = Claims::with_custom_claims(user, Duration::from_secs(JWT_DURATION));

        let claims = claims.with_issuer(JWT_ISSUER).with_audience(JWT_AUDIENCE);

        Ok(self.0.sign(claims)?)
    }
}

impl DecodingKey {
    pub fn new(pem: &str) -> Result<Self, AppError> {
        Ok(Self(Ed25519PublicKey::from_pem(pem)?))
    }

    pub fn verify(&self, token: &str) -> Result<User, AppError> {
        let options = VerificationOptions {
            allowed_issuers: Some(HashSet::from_strings(&[JWT_ISSUER])),
            allowed_audiences: Some(HashSet::from_strings(&[JWT_AUDIENCE])),
            time_tolerance: Some(Duration::from_secs(JWT_DURATION)),
            ..Default::default()
        };
        let claims = self.0.verify_token(token, Some(options))?;
        Ok(claims.custom)
    }
}

#[cfg(test)]
mod tests {
    use crate::AppState;
    use anyhow::Result;

    #[tokio::test]
    async fn sign_and_verify_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("user should exists");

        let token = state.ek.sign(user.clone())?;

        let ret = state.dk.verify(&token)?;

        assert_eq!(user.id, ret.id);
        assert_eq!(user.name, ret.name);
        assert_eq!(user.password_hash, ret.password_hash);
        assert_eq!(user.created_at, ret.created_at);
        assert_eq!(user.email, ret.email);

        Ok(())
    }
}
