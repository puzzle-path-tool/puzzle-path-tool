use std::error::Error;

use puzzle_base_build as base;

fn main() {
    generate_bindings().unwrap_or_else(|err| panic!("{}", err));
}

fn generate_bindings() -> Result<(), Box<dyn Error>> {
    let config = cbindgen::Config::from_file("cbindgen.toml")?;
    let out_file = base::beside_file_path(".h")?;

    cbindgen::generate_with_config(base::crate_dir()?, config)?.write_to_file(out_file);

    base::rerun_if_changed_any();
    Ok(())
}
