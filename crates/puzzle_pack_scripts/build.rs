#![allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-build-cfg")]
use std::{
    fs::{self, File},
    io::Write,
};

use puzzle_pack_bindings::ts_api;

fn main() {
    let types = ts_api::load_types();
    fs::create_dir_all("ts/api/").unwrap();
    let mut file = File::create("ts/api/puzzpt_bindings.ts").unwrap();
    file.write_all(types.as_bytes()).unwrap();
}
