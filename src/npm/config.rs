use crate::error_handling::npm_config::ConfigError;
use std::{io, process::Command, result};

#[cfg(target_os = "windows")]
const NPM_CMD: &str = "npm.cmd";

#[cfg(not(target_os = "windows"))]
const NPM_CMD: &str = "npm";

pub fn get_registry() -> result::Result<String, ConfigError> {
    let output = Command::new(NPM_CMD).args(&["get", "registry"]).output()?;
    if !output.status.success() {
        let err_msg = String::from_utf8(output.stderr)?;
        let io_error = io::Error::new(io::ErrorKind::Other, err_msg);
        return Err(ConfigError::NpmError(io_error));
    }
    let registry = String::from_utf8(output.stdout)?.trim().to_string();
    Ok(registry)
}

pub fn set_registry(
    registry_url: &str,
    registry_name: &str,
) -> result::Result<String, ConfigError> {
    let output = Command::new(NPM_CMD)
        .args(&["config", "set", "registry", registry_url])
        .output()?;
    if !output.status.success() {
        let err_msg = String::from_utf8(output.stderr)?;
        let io_error = io::Error::new(io::ErrorKind::Other, err_msg);
        return Err(ConfigError::NpmError(io_error));
    }
    let success_msg = format!("The registry has been changed to '{}'", registry_name);
    Ok(success_msg)
}
