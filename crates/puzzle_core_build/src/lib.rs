use std::{
    env::{self, VarError},
    path::PathBuf,
};

use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq, Clone)]
pub enum BuildEnvError {
    #[error("Env var {0} is invalid: {1}")]
    InvalidEnvVar(VarError, String),
    #[error("Dir Structure is invalid: {}", .0.to_str().unwrap_or_default())]
    InvalidDirStructure(PathBuf),
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn crate_dir() -> Result<PathBuf, BuildEnvError> {
    dir_from_env_var("CARGO_MANIFEST_DIR")
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn out_dir() -> Result<PathBuf, BuildEnvError> {
    dir_from_env_var("OUT_DIR")
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn target_dir() -> Result<PathBuf, BuildEnvError> {
    let out_dir = out_dir()?;
    out_dir
        .ancestors()
        .nth(3)
        .map(PathBuf::from)
        .ok_or(BuildEnvError::InvalidDirStructure(out_dir))
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn target_file_name() -> Result<String, BuildEnvError> {
    let file_prefix = match target_os()?.as_str() {
        "windows" => "",
        _ => "lib",
    };
    Ok(format!(
        "{}{}",
        file_prefix,
        package_name()?.replace('-', "_")
    ))
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn beside_file_path(suffix: &str) -> Result<PathBuf, BuildEnvError> {
    Ok(target_dir()?.join(format!("{}{}", target_file_name()?, suffix)))
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn package_name() -> Result<String, BuildEnvError> {
    string_from_env_var("CARGO_PKG_NAME")
}

/// TODO: Doc
/// # Errors
/// TODO: Errors
pub fn target_os() -> Result<String, BuildEnvError> {
    string_from_env_var("CARGO_CFG_TARGET_OS")
}

fn string_from_env_var(var: &str) -> Result<String, BuildEnvError> {
    env::var(var).map_err(|err| BuildEnvError::InvalidEnvVar(err, String::from(var)))
}

fn dir_from_env_var(var: &str) -> Result<PathBuf, BuildEnvError> {
    string_from_env_var(var).map(PathBuf::from)
}

pub fn rerun_if_changed_any() {
    rerun_if_changed("**/*");
}

pub fn rerun_if_changed(pattern: &str) {
    println!("cargo:rerun-if-changed={pattern}");
}
