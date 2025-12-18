use std::{error, io, process::Command, result};

pub fn get_registry() -> result::Result<String, Box<dyn error::Error>> {
    let output = Command::new("npm.cmd")
        .args(&["get", "registry"])
        .output()?;
    if output.status.success() {
        let registry_url = String::from_utf8(output.stdout)?;
        let registry_url = registry_url.trim_end();
        Ok(registry_url.to_string())
    } else {
        let error_message = String::from_utf8(output.stderr)?;
        let err = io::Error::new(io::ErrorKind::Other, error_message);
        Err(Box::new(err))
    }
}

pub fn set_registry(registry_url: &str) {
    // TODO: Implement
}
