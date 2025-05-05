use puzzle_core_build as cb;

fn main() {
    generate_bindings().unwrap_or_else(|err| panic!("{}", err));
}

fn generate_bindings() -> anyhow::Result<()> {
    let config = cbindgen::Config::from_file("cbindgen.toml").map_err(anyhow::Error::msg)?;
    let out_file = cb::beside_file_path(".h")?;

    cbindgen::generate_with_config(cb::crate_dir()?, config)?.write_to_file(out_file);

    cb::rerun_if_changed_any();
    Ok(())
}
