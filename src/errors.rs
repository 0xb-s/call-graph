use thiserror::Error;
use std::io;
use std::fmt;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),

    #[error("Application error: {0}")]
    Generic(String),
}

impl From<&str> for AppError {
    fn from(msg: &str) -> Self {
        AppError::Generic(msg.to_string())
    }
}

impl From<String> for AppError {
    fn from(msg: String) -> Self {
        AppError::Generic(msg)
    }
}
