//! Error handling for rhai-redis

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    #[error("Rhai script error: {0}")]
    Script(String),

    #[error("Connection error: {0}")]
    Connection(String),
}

impl From<rhai::EvalAltResult> for Error {
    fn from(err: rhai::EvalAltResult) -> Self {
        Error::Script(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
