use std::path::Path;

use const_format::concatcp;
use include_dir::{Dir, include_dir};

#[derive(Debug, Clone)]
pub struct TsFiles {
    pub editorconfig: &'static str,
    pub gitignore: &'static str,
    pub eslint_config: &'static str,
    pub package_json: &'static str,
    pub package_lock: &'static str,
    pub tsconfig: &'static str,
    pub scripts: Dir<'static>,
}

const GITIGNORE_EXTRA: &str = "\
scripts/api/
";

macro_rules! include_str_ts {
    ($name:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ts/", $name))
    };
}

pub struct TsFile<'a> {
    pub path: &'a Path,
    pub content: &'a str,
}

impl TsFiles {
    #[must_use]
    pub fn load() -> Self {
        TsFiles {
            editorconfig: include_str_ts!(".editorconfig"),
            gitignore: concatcp!(include_str_ts!(".gitignore"), GITIGNORE_EXTRA),
            eslint_config: include_str_ts!("eslint.config.js"),
            package_json: include_str_ts!("package.json"),
            package_lock: include_str_ts!("package-lock.json"),
            tsconfig: include_str_ts!("tsconfig.json"),
            scripts: include_dir!("$CARGO_MANIFEST_DIR/ts/scripts"),
        }
    }

    #[must_use]
    pub fn script_files(&self) -> Vec<TsFile<'static>> {
        let Ok(entries) = self.scripts.find("**/*.ts") else {
            return vec![];
        };

        entries
            .filter_map(|entry| {
                let path = entry.path();
                entry
                    .as_file()
                    .and_then(|f| f.contents_utf8())
                    .map(|content| TsFile { path, content })
            })
            .collect()
    }
}
