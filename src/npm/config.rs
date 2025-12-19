use crate::error_handling::npm_config::ConfigError;
use std::{io, process::Command, result};

pub fn get_registry() -> result::Result<String, ConfigError> {
    let output = Command::new("npm.cmd")
        .args(&["get", "registry"])
        .output()?;
    if !output.status.success() {
        let err_msg = String::from_utf8(output.stderr)?;
        let io_error = io::Error::new(io::ErrorKind::Other, err_msg);
        return Err(ConfigError::NpmError(io_error));
    }
    let registry = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(registry)
}

pub fn set_registry(registry_url: &str) {
    // TODO: Implement
}
