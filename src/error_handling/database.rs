use thiserror::Error;

use crate::error_handling::npm_config::ConfigError;

#[derive(Error, Debug)]
pub enum FindAllError {
    #[error("Failed to execute SQL query: {0}")]
    SelectError(#[from] rusqlite::Error),
    #[error("Failed to retrieve registry URL: {0}")]
    RegistryError(#[from] ConfigError),
}
