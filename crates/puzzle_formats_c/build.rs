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

    let package_name =
        PathBuf::from(env::var("CARGO_PKG_NAME").expect("CARGO_PKG_NAME env var is not defined"));

    let config = cbindgen::Config::from_file("cbindgen.toml")
        .expect("Unable to find cbindgen.toml configuration file");

    let filename = format!(
        "{}.h",
        package_name
            .to_str()
            .expect("Invalid Package name")
            .replace("-", "_")
    );

    cbindgen::generate_with_config(&crate_dir, config)
        .expect("Unable to generate bindings")
        .write_to_file(target_dir.join(filename));

    println!("cargo:rerun-if-changed=**/*");
}
