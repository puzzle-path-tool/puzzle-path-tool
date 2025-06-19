#![allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]
#![allow(clippy::all, dead_code, unused_imports)]

use ignore::{DirEntry, WalkBuilder};
use itertools::Itertools;
use pathdiff::diff_paths;
use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};
use syn::{Attribute, Item, spanned::Spanned};

const CLIPPY_BUILD_CONFIG: Config = Config {
    name: "clippy-build-cfg",
    values: "clippy::expect_used, clippy::unwrap_used",
};

const CLIPPY_TEST_CONFIG: Config = Config {
    name: "clippy-test-cfg",
    values: "clippy::expect_used, clippy::unwrap_used",
};

// #[test]
fn check_build_scripts() {
    let config = CLIPPY_BUILD_CONFIG;

    let incorrect_configs = find_incorrect_clippy_configs(
        |entry| entry.path().is_file() && entry.file_name() == "build.rs",
        |file| vec![SelectorValue::File(file)],
        &config,
    );

    assert_correct_configs(
        &incorrect_configs,
        &config,
        "These build scripts do not contain the correct clippy config",
    );
}

// #[test]
fn check_integration_tests() {
    let config = CLIPPY_TEST_CONFIG;

    let incorrect_configs = find_incorrect_clippy_configs(
        |entry| {
            let path = entry.path();
            if !path.is_file() {
                return false;
            }

            let Some(extension) = path.extension().map(|s| s.to_string_lossy()) else {
                return false;
            };
            if extension != "rs" {
                return false;
            }

            let Some(dir_path) = path.parent() else {
                return false;
            };
            let Some(dir_name) = dir_path.file_name().map(|s| s.to_string_lossy()) else {
                return false;
            };
            if dir_name != "tests" {
                return false;
            }

            let toml_path = dir_path.join("../Cargo.toml");
            if !fs::exists(toml_path).expect("Error Checking for Cargo toml") {
                return false;
            }

            return true;
        },
        |file| vec![SelectorValue::File(file)],
        &config,
    );

    assert_correct_configs(
        &incorrect_configs,
        &config,
        "These tests do not contain the correct clippy config",
    );
}

// #[test]
fn check_tests() {
    let config = CLIPPY_TEST_CONFIG;

    let incorrect_configs = find_incorrect_clippy_configs(
        |entry| {
            let path = entry.path();
            if !path.is_file() {
                return false;
            }

            let Some(extension) = path.extension().map(|s| s.to_string_lossy()) else {
                return false;
            };

            extension == "rs"
        },
        |file| {
            let mut selector_values: Vec<SelectorValue> = vec![];

            //TODO: Search for all items with cfg(test) attr
            selector_values.push(SelectorValue::File(file));

            selector_values
        },
        &config,
    );

    assert_correct_configs(
        &incorrect_configs,
        &config,
        "These tests do not contain the correct clippy config",
    );
}

#[derive(Debug, Default, Clone)]
struct IncorrectConfigs {
    outdated: Vec<String>,
    missing: Vec<String>,
}

#[derive(Debug, Clone)]
struct Config {
    name: &'static str,
    values: &'static str,
}

#[derive(Clone)]
enum SelectorValue {
    Item(Item),
    File(syn::File),
}

impl SelectorValue {
    fn into_item(self) -> SelectorItem {
        match self {
            Self::Item(item) => {
                let pos = item.span().start();
                SelectorItem {
                    line: pos.line,
                    column: pos.column,
                    attributes: match item {
                        Item::Const(item) => item.attrs,
                        Item::Enum(item) => item.attrs,
                        Item::ExternCrate(item) => item.attrs,
                        Item::Fn(item) => item.attrs,
                        Item::ForeignMod(item) => item.attrs,
                        Item::Impl(item) => item.attrs,
                        Item::Macro(item) => item.attrs,
                        Item::Mod(item) => item.attrs,
                        Item::Static(item) => item.attrs,
                        Item::Struct(item) => item.attrs,
                        Item::Trait(item) => item.attrs,
                        Item::TraitAlias(item) => item.attrs,
                        Item::Type(item) => item.attrs,
                        Item::Union(item) => item.attrs,
                        Item::Use(item) => item.attrs,
                        _ => vec![],
                    },
                }
            }
            Self::File(file) => SelectorItem {
                line: 1,
                column: 0,
                attributes: file.attrs,
            },
        }
    }
}

struct SelectorItem {
    line: usize,
    column: usize,
    attributes: Vec<Attribute>,
}

fn find_incorrect_clippy_configs(
    file_filter: impl Fn(&DirEntry) -> bool,
    item_selector: impl Fn(syn::File) -> Vec<SelectorValue>,
    config: &Config,
) -> IncorrectConfigs {
    let workspace_root = concat!(env!("CARGO_MANIFEST_DIR"), "/../");

    let walk = WalkBuilder::new(workspace_root).hidden(false).build();

    let mut incorrect_configs = IncorrectConfigs::default();

    for entry in walk {
        let entry = entry.expect("Error traversing files");

        if !file_filter(&entry) {
            continue;
        }

        let path = entry.path();

        let display_path = diff_paths(path, workspace_root).expect("Invalid Path");
        let display_path = display_path.to_string_lossy();

        let content = fs::read_to_string(path).expect("Error reading file");

        let Ok(file) = syn::parse_file(&content) else {
            continue;
        };

        let selector_values = item_selector(file);

        for selector_value in selector_values {
            let item = selector_value.into_item();

            // item.attributes.iter().find(|attr| {
            //     attr.style
            // })
            //TODO: Check if the attribute is there
            println!("{config:?} {:?}", item.attributes.len());
            let display_file_location = format!("{}:{}:{}", display_path, item.line, item.column);
            incorrect_configs.missing.push(display_file_location);
        }
    }
    incorrect_configs
}

fn assert_correct_configs(incorrect_configs: &IncorrectConfigs, config: &Config, message: &str) {
    assert!(
        incorrect_configs.missing.is_empty() && incorrect_configs.outdated.is_empty(),
        r#"
        {}

        Config: #![allow({}, reason = "{}")]

        Missing Config in:
        {}

        Outdated Config in:
        {}
        "#,
        message,
        config.values,
        config.name,
        incorrect_configs.missing.join("\n"),
        incorrect_configs.outdated.join("\n"),
    );
}
