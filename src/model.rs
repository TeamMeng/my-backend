use crate::{AppError, AppState};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

impl AppState {
    pub async fn create_user(&self, input: CreateUser) -> Result<User, AppError> {
        if let Some(user) = self.find_user_by_email(&input.email).await? {
            return Err(AppError::UserError(format!(
                "{} by user already exists",
                user.email
            )));
        }

        let password_hash = hash_password(&input.password)?;

        let user = sqlx::query_as(
            "
            INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING *
            ",
        )
        .bind(input.name)
        .bind(input.email)
        .bind(password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let user = sqlx::query_as(
            "
            SELECT * FROM users WHERE email = $1
            ",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn delete_user_by_email(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as(
            "
            DELETE FROM users WHERE email = $1 RETURNING *
            ",
        )
        .bind(email)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn login_user(&self, input: LoginUser) -> Result<bool, AppError> {
        let ret = self.find_user_by_email(&input.email).await?;

        match ret {
            Some(user) => {
                let is_valid = verify_password(&input.password, &user.password_hash)?;
                if is_valid {
                    Ok(true)
                } else {
                    Err(AppError::UserError(format!(
                        "{} by user password is error",
                        input.email
                    )))
                }
            }
            None => Err(AppError::UserError(format!(
                "{} by user not find",
                input.email
            ))),
        }
    }
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    let parsed_hash = PasswordHash::new(&password_hash)?.to_string();
    Ok(parsed_hash)
}

pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let argon2 = Argon2::default();

    let password_hash = PasswordHash::new(password_hash)?;

    let is_valid = argon2
        .verify_password(password.as_bytes(), &password_hash)
        .is_ok();

    Ok(is_valid)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn password_hash_and_verify_password_should_work() -> Result<()> {
        let password = "hunter42";

        let password_hash = hash_password(password)?;

        let is_valid = verify_password(password, &password_hash)?;

        assert!(is_valid);

        Ok(())
    }

    #[tokio::test]
    async fn create_user_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let name = "TeamMeng";
        let email = "Meng@acme.org";
        let password = "hunter42";

        let input = CreateUser::new(name, email, password);

        let user = state.create_user(input).await?;

        assert_eq!(user.name, name);
        assert_eq!(user.email, email);

        let is_valid = verify_password(password, &user.password_hash)?;

        assert!(is_valid);

        Ok(())
    }

    #[tokio::test]
    async fn find_user_by_email_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";

        let user = state
            .find_user_by_email(email)
            .await?
            .expect("User should exist");

        assert_eq!(user.email, email);
        assert_eq!(user.name, "TeamMeng");

        Ok(())
    }

    #[tokio::test]
    async fn delete_user_by_email_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";

        let user = state.delete_user_by_email(email).await?;

        assert_eq!(user.email, email);
        assert_eq!(user.name, "TeamMeng");
        Ok(())
    }

    #[tokio::test]
    async fn login_user_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let email = "Meng@123.com";
        let password = "123456";

        let input = LoginUser::new(email, password);

        let ret = state.login_user(input).await?;

        assert!(ret);

        Ok(())
    }

    impl CreateUser {
        fn new(
            name: impl Into<String>,
            email: impl Into<String>,
            password: impl Into<String>,
        ) -> Self {
            Self {
                name: name.into(),
                email: email.into(),
                password: password.into(),
            }
        }
    }

    impl LoginUser {
        fn new(email: impl Into<String>, password: impl Into<String>) -> Self {
            Self {
                email: email.into(),
                password: password.into(),
            }
        }
    }
}
