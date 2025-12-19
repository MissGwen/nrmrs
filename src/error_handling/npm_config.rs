use std::{io, string};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("Failed to execute npm command: {0}")]
    NpmError(#[from] io::Error),
    #[error("UTF-8 decoding failed: {0}")]
    Utf8Error(#[from] string::FromUtf8Error),
}
