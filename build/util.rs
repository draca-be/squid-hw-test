use std::env;

use anyhow::Result;

pub fn is_idf() -> Result<bool> {
    let target_os = env::var("CARGO_CFG_TARGET_OS")?;

    if target_os == "espidf" {
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Handy helper function to quickly print all environment variables when debugging issues
#[allow(dead_code)]
pub fn print_env() {
    eprintln!("Environment variables:");

    for (key, value) in env::vars() {
        eprintln!("{} = {}", key, value);
    }
}
