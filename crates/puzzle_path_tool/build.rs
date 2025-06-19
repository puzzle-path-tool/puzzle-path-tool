#![allow(clippy::expect_used, clippy::unwrap_used)] // [[build-clippy-cfg]]
use std::{
    fs::{self, File},
    io::Write,
};

use puzzle_core::ts_api;

fn main() {
    let types = ts_api::load_types();
    fs::create_dir_all("ts/packs/api/").unwrap();
    let mut file = File::create("ts/packs/api/puzzpt_api.ts").unwrap();
    file.write_all(types.as_bytes()).unwrap();
}
