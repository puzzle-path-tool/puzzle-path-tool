#![allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]

use ignore::WalkBuilder;
use itertools::Itertools;
use pathdiff::diff_paths;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

const CLIPPY_BUILD_CONF: &str =
    r#"[allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-build-cfg")]"#;

const CLIPPY_TEST_CONF: &str =
    r#"[allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]"#;

#[test]
fn check_build_scripts() {
    let workspace_root = concat!(env!("CARGO_MANIFEST_DIR"), "/../");
    let conf_string = "#!".to_string() + CLIPPY_BUILD_CONF;

    let walk = WalkBuilder::new(workspace_root).hidden(false).build();

    let mut failed_scripts: Vec<PathBuf> = vec![];

    for entry in walk {
        let entry = entry.expect("Error traversing files");

        if !entry.path().is_file() || entry.file_name() != "build.rs" {
            continue;
        }

        let file = File::open(entry.path()).expect("Error opening file"); // access denied here: WHY?
        let reader = BufReader::new(file);
        let line = reader
            .lines()
            .next()
            .unwrap_or(Ok(String::new()))
            .expect("Error reading file: Invalid UTF8?");

        if line.trim() != conf_string {
            failed_scripts.push(entry.path().to_path_buf());
        }
    }

    assert!(
        failed_scripts.is_empty(),
        "These build scripts do not contain the required clippy config as first line:\n\n{conf_string}\n\n{}\n",
        failed_scripts
            .iter()
            .map(|s| diff_paths(s, workspace_root)
                .expect("Could not diff paths")
                .to_string_lossy()
                .to_string()
                + ":1")
            .join("\n")
    );
}

#[test]
fn check_integration_tests() {
    let workspace_root = concat!(env!("CARGO_MANIFEST_DIR"), "/../");
    let conf_string = "#!".to_string() + CLIPPY_TEST_CONF;

    let walk = WalkBuilder::new(workspace_root).hidden(false).build();

    let mut failed_scripts: Vec<PathBuf> = vec![];

    for entry in walk {
        let entry = entry.expect("Error traversing files");

        if !entry.path().is_file()
            || entry
                .path()
                .parent()
                .and_then(|p| p.file_name())
                .and_then(|s| s.to_str())
                != Some("tests")
            || entry.path().extension().map(|e| e.to_str()) != Some(Some("rs"))
        {
            continue;
        }

        let toml_path = entry
            .path()
            .parent()
            .expect("No Parent")
            .join("../Cargo.toml");

        if !fs::exists(toml_path).expect("Error Checking for Cargo toml") {
            continue;
        }

        let file = File::open(entry.path()).expect("Error opening file"); // access denied here: WHY?
        let reader = BufReader::new(file);
        let line = reader
            .lines()
            .next()
            .unwrap_or(Ok(String::new()))
            .expect("Error reading file: Invalid UTF8?");

        if line.trim() != conf_string {
            failed_scripts.push(entry.path().to_path_buf());
        }
    }

    assert!(
        failed_scripts.is_empty(),
        "These integration tests do not contain the required clippy config as first line:\n\n{conf_string}\n\n{}\n",
        failed_scripts
            .iter()
            .map(|s| diff_paths(s, workspace_root)
                .expect("Could not diff paths")
                .to_string_lossy()
                .to_string()
                + ":1")
            .join("\n")
    );
}

//TODO: Add Test for Tests
//TODO: Clean up logic
//TODO: Make Line Requirement more lenient (eg. anywhere before the first non macro line)

// Params:
//  file_filter: path -> bool // Which files are tested
//  item_selector: syn-root -> Vec<syn-item> // Which items are required to have the config
//  config-name: String
//  config: String // The last two might be a struct
// Returns:
//  outdated: Vec<String>
//  missing: Vec<String>
