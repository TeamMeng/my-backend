use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("argon2 error password hash: {0}")]
    Argon2Error(#[from] argon2::password_hash::Error),

    #[error("user error: {0}")]
    UserError(String),

    #[error("io error: {0}")]
    IoError(#[from] io::Error),

    #[error("jwt error: {0}")]
    JwtError(#[from] jwt_simple::Error),

    #[error("serde yaml error: {0}")]
    SerdeYamlError(#[from] serde_yaml::Error),

    #[error("sqlx error: {0}")]
    SqlxError(#[from] sqlx::Error),
}
