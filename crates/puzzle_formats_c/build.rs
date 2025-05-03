use std::{env, path::PathBuf};

fn main() {
    let crate_dir = PathBuf::from(
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var is not defined"),
    );

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR env var is not defined"));

    let target_dir = out_dir
        .ancestors()
        .nth(3)
        .expect("Failed to retrieve target dir");

    let package_name = env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env var is not defined");

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    let target_os =
        env::var("CARGO_CFG_TARGET_OS").expect("CARGO_CFG_TARGET_OS env var is not defined");
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    let file_prefix = match target_os.as_str() {
        "windows" => "",
        _ => "lib",
    };

    println!("cargo:warning=Target OS: {target_os}, env: {target_env}");

    let filename = format!("{}{}.h", file_prefix, package_name.replace("-", "_"));

    cbindgen::generate_with_config(&crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file(target_dir.join(filename));

    println!("cargo:rerun-if-changed=**/*");
}
