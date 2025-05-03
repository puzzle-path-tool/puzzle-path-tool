use std::{
    env::{self, VarError},
    error::Error,
    fmt::Display,
    path::PathBuf,
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BuildEnvError {
    InvalidEnvVar(VarError, String),
    InvalidDirStructure(PathBuf),
}

impl Display for BuildEnvError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildEnvError::InvalidEnvVar(err, var) => {
                write!(f, "Env var {} is invalid: {}", var, err)
            }
            BuildEnvError::InvalidDirStructure(path) => write!(
                f,
                "Dir Structure is invalid: {}",
                path.to_str().unwrap_or_default()
            ),
        }
    }
}
impl Error for BuildEnvError {}

pub fn crate_dir() -> Result<PathBuf, BuildEnvError> {
    dir_from_env_var("CARGO_MANIFEST_DIR")
}

pub fn out_dir() -> Result<PathBuf, BuildEnvError> {
    dir_from_env_var("OUT_DIR")
}

pub fn target_dir() -> Result<PathBuf, BuildEnvError> {
    let out_dir = out_dir()?;
    out_dir
        .ancestors()
        .nth(3)
        .map(PathBuf::from)
        .ok_or(BuildEnvError::InvalidDirStructure(out_dir))
}

pub fn target_file_name() -> Result<String, BuildEnvError> {
    let file_prefix = match target_os()?.as_str() {
        "windows" => "",
        _ => "lib",
    };
    Ok(format!(
        "{}{}",
        file_prefix,
        package_name()?.replace("-", "_")
    ))
}

pub fn beside_file_path(suffix: &str) -> Result<PathBuf, BuildEnvError> {
    Ok(target_dir()?.join(format!("{}{}", target_file_name()?, suffix)))
}

pub fn package_name() -> Result<String, BuildEnvError> {
    string_from_env_var("CARGO_PKG_NAME")
}

pub fn target_os() -> Result<String, BuildEnvError> {
    string_from_env_var("CARGO_CFG_TARGET_OS")
}

fn string_from_env_var(var: &str) -> Result<String, BuildEnvError> {
    env::var(var).map_err(|err| BuildEnvError::InvalidEnvVar(err, String::from(var)))
}

fn dir_from_env_var(var: &str) -> Result<PathBuf, BuildEnvError> {
    string_from_env_var(var).map(|v| PathBuf::from(v))
}

pub fn rerun_if_changed_any() {
    rerun_if_changed("**/*");
}

pub fn rerun_if_changed(pattern: &str) {
    println!("cargo:rerun-if-changed={}", pattern);
}
