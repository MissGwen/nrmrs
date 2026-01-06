use std::io;
use thiserror::Error;

use crate::error_handling::npm_config::ConfigError;

#[derive(Error, Debug)]
pub enum FindAllError {
    #[error("Failed to execute SQL query: {0}")]
    SelectError(#[from] rusqlite::Error),
    #[error("Failed to retrieve registry URL: {0}")]
    RegistryError(#[from] ConfigError),
}

#[derive(Error, Debug)]
pub enum CreateError {
    #[error("Npm registry already exists: {0}")]
    SelectError(#[from] io::Error),
    #[error("Failed to execute SQL query: {0}")]
    AddError(#[from] rusqlite::Error),
}

#[derive(Error, Debug)]
pub enum DeleteError {
    #[error("Npm registry does not exist: {0}")]
    SelectError(#[from] io::Error),
    #[error("Failed to execute SQL query: {0}")]
    RemoveError(#[from] rusqlite::Error),
}
