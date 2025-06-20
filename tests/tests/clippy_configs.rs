#![allow(clippy::expect_used, clippy::unwrap_used, reason = "clippy-test-cfg")]

use ignore::{DirEntry, WalkBuilder};
use indoc::indoc;
use itertools::Itertools;
use pathdiff::diff_paths;
use std::fs::{self};
use syn::{Attribute, Expr, Item, ItemMod, LitStr, spanned::Spanned, visit::Visit};

const CLIPPY_BUILD_CONFIG: Config = Config {
    name: "clippy-build-cfg",
    values: &["clippy::expect_used", "clippy::unwrap_used"],
};

const CLIPPY_TEST_CONFIG: Config = Config {
    name: "clippy-test-cfg",
    values: &["clippy::expect_used", "clippy::unwrap_used"],
};

#[test]
fn check_build_script_clippy_conf() {
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
        "#!",
    );
}

#[test]
fn check_integration_test_clippy_conf() {
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

            true
        },
        |file| vec![SelectorValue::File(file)],
        &config,
    );

    assert_correct_configs(
        &incorrect_configs,
        &config,
        "These tests do not contain the correct clippy config",
        "#!",
    );
}

#[test]
fn check_test_clippy_conf() {
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

            if has_test_attr(&file.attrs) {
                selector_values.push(SelectorValue::File(file));
            }

            let mut collector = TestItemCollector::default();
            collector.visit_file(file);

            selector_values.extend(collector.items.iter().map(|item| SelectorValue::Item(item)));

            selector_values
        },
        &config,
    );

    assert_correct_configs(
        &incorrect_configs,
        &config,
        "These tests do not contain the correct clippy config",
        "#",
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
    values: &'static [&'static str],
}

#[derive(Clone)]
enum SelectorValue<'a> {
    Item(&'a Item),
    File(&'a syn::File),
}

fn get_attrs(item: &Item) -> &[Attribute] {
    match item {
        Item::Const(item) => &item.attrs,
        Item::Enum(item) => &item.attrs,
        Item::ExternCrate(item) => &item.attrs,
        Item::Fn(item) => &item.attrs,
        Item::ForeignMod(item) => &item.attrs,
        Item::Impl(item) => &item.attrs,
        Item::Macro(item) => &item.attrs,
        Item::Mod(item) => &item.attrs,
        Item::Static(item) => &item.attrs,
        Item::Struct(item) => &item.attrs,
        Item::Trait(item) => &item.attrs,
        Item::TraitAlias(item) => &item.attrs,
        Item::Type(item) => &item.attrs,
        Item::Union(item) => &item.attrs,
        Item::Use(item) => &item.attrs,
        _ => &[],
    }
}

impl SelectorValue<'_> {
    fn as_item(&self) -> SelectorItem {
        match self {
            Self::Item(item) => {
                let pos = item.span().start();
                SelectorItem {
                    line: pos.line,
                    column: pos.column,
                    attributes: get_attrs(item),
                }
            }
            Self::File(file) => SelectorItem {
                line: 1,
                column: 0,
                attributes: &file.attrs,
            },
        }
    }
}

struct SelectorItem<'a> {
    line: usize,
    column: usize,
    attributes: &'a [Attribute],
}

fn find_incorrect_clippy_configs(
    file_filter: impl Fn(&DirEntry) -> bool,
    item_selector: impl for<'a> Fn(&'a syn::File) -> Vec<SelectorValue<'a>>,
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

        let selector_values = item_selector(&file);

        for selector_value in selector_values {
            let item = selector_value.as_item();

            let attr_status = has_config_attr(item.attributes, config);

            let display_file_location = format!("{}:{}:{}", display_path, item.line, item.column);

            match attr_status {
                ConfigAttrStatus::Missing => incorrect_configs.missing.push(display_file_location),
                ConfigAttrStatus::Outdated => {
                    incorrect_configs.outdated.push(display_file_location);
                }
                ConfigAttrStatus::Present => {}
            }
        }
    }
    incorrect_configs
}

enum ConfigAttrStatus {
    Missing,
    Outdated,
    Present,
}

fn has_config_attr(attributes: &[Attribute], config: &Config) -> ConfigAttrStatus {
    for attr in attributes {
        if !attr.path().is_ident("allow") {
            continue;
        }
        let mut items = config.values.iter();
        let mut reason_found = false;
        let mut mismatch_found = false;

        let _ = attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("reason") {
                if let Ok(Ok(value)) = meta.value().map(syn::parse::ParseBuffer::parse::<LitStr>) {
                    if value.value() == config.name {
                        reason_found = true;
                        if items.next().is_some() {
                            mismatch_found = true;
                        }
                    }
                }
            } else {
                let parsed_item = meta
                    .path
                    .segments
                    .iter()
                    .map(|seg| seg.ident.to_string().trim().to_string())
                    .join("::");

                if let Some(required_item) = items.next() {
                    if *required_item != parsed_item {
                        mismatch_found = true;
                    }
                } else {
                    mismatch_found = true;
                }
            }
            Ok(())
        });

        if reason_found {
            if mismatch_found {
                return ConfigAttrStatus::Outdated;
            }
            return ConfigAttrStatus::Present;
        }
    }
    ConfigAttrStatus::Missing
}

#[derive(Default)]
struct TestItemCollector<'a> {
    pub items: Vec<&'a Item>,
}

impl<'a> Visit<'a> for TestItemCollector<'a> {
    fn visit_item(&mut self, item: &'a Item) {
        let attrs = get_attrs(item);

        if has_test_attr(attrs) {
            self.items.push(item);
        }

        if let Item::Mod(ItemMod {
            content: Some((_, items)),
            ..
        }) = item
        {
            for item in items {
                self.visit_item(item);
            }
        }
    }
}

fn has_test_attr(attributes: &[Attribute]) -> bool {
    for attr in attributes {
        if !attr.path().is_ident("cfg") {
            continue;
        }
        let Ok(expr) = attr.parse_args::<Expr>() else {
            continue;
        };
        if expr_contains_test(&expr) {
            return true;
        }
    }
    false
}

fn expr_contains_test(expr: &Expr) -> bool {
    match expr {
        Expr::Path(path) => path.path.is_ident("test"),
        Expr::Call(call) => {
            for arg in &call.args {
                if expr_contains_test(arg) {
                    return true;
                }
            }
            false
        }
        _ => false,
    }
}

fn assert_correct_configs(
    incorrect_configs: &IncorrectConfigs,
    config: &Config,
    message: &str,
    macro_prefix: &str,
) {
    let missing_message = format_config_list(&incorrect_configs.missing, "Missing Configs at:");
    let outdated_message = format_config_list(&incorrect_configs.outdated, "Outdated Configs at:");

    assert!(
        incorrect_configs.missing.is_empty() && incorrect_configs.outdated.is_empty(),
        indoc! {
            r#"

                {}

                Config: 
                {}[allow({}, reason = "{}")]

                {}{}
            "#
        },
        message,
        macro_prefix,
        config.values.join(", "),
        config.name,
        missing_message,
        outdated_message,
    );
}

fn format_config_list(configs: &[String], message: &str) -> String {
    if configs.is_empty() {
        String::new()
    } else {
        format!(
            "{}\n{}\n\n",
            message,
            configs.iter().map(|c| format!(">  {c}")).join("\n"),
        )
    }
}

//TODO: clean up ordering an maybe names
