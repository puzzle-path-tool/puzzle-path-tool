use puzzle_core_build as cb;
use puzzle_core_build::BuildResult;

fn main() {
    generate_bindings().unwrap_build_result();
}

fn generate_bindings() -> anyhow::Result<()> {
    let config = cbindgen::Config::from_file("cbindgen.toml").map_err(anyhow::Error::msg)?;
    let out_file = cb::beside_file_path(".h")?;

    cbindgen::generate_with_config(cb::crate_dir()?, config)?.write_to_file(out_file);

    cb::rerun_if_changed_any();
    Ok(())
}
