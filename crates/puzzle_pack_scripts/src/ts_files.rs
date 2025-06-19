use std::path::{Path, PathBuf};

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
    pub example_puzzle: &'static str,
    pub packs: Dir<'static>,
}

const GITIGNORE_EXTRA: &str = "\
packs/api/
packs/core/
";

macro_rules! include_str_ts {
    ($name:literal) => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ts/", $name))
    };
}

pub struct TsFile<'a> {
    pub path: PathBuf,
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
            example_puzzle: include_str_ts!("puzzles/example.ts"),
            packs: include_dir!("$CARGO_MANIFEST_DIR/ts/packs"),
        }
    }

    #[allow(clippy::expect_used, clippy::missing_panics_doc)]
    pub fn script_files(&self) -> impl Iterator<Item = TsFile<'static>> {
        let entries = self.packs.find("**/*.ts").expect("invalid Pattern");

        entries.filter_map(|entry| {
            let path = entry.path();
            entry
                .as_file()
                .and_then(|f| f.contents_utf8())
                .map(|content| TsFile {
                    path: path.to_path_buf(),
                    content,
                })
        })
    }

    pub fn config_files(&self) -> impl Iterator<Item = TsFile<'static>> {
        [
            TsFile {
                path: Path::new(".editorconfig").to_path_buf(),
                content: self.editorconfig,
            },
            TsFile {
                path: Path::new(".gitignore").to_path_buf(),
                content: self.gitignore,
            },
            TsFile {
                path: Path::new("eslint.config.js").to_path_buf(),
                content: self.eslint_config,
            },
            TsFile {
                path: Path::new("package.json").to_path_buf(),
                content: self.package_json,
            },
            TsFile {
                path: Path::new("package-lock.json").to_path_buf(),
                content: self.package_lock,
            },
            TsFile {
                path: Path::new("tsconfig.json").to_path_buf(),
                content: self.tsconfig,
            },
            TsFile {
                path: Path::new("puzzles/example.ts").to_path_buf(),
                content: self.example_puzzle,
            },
        ]
        .into_iter()
    }

    pub fn all_files(&self) -> impl Iterator<Item = TsFile<'static>> {
        let packs_dir = Path::new("packs/");

        self.config_files()
            .chain(self.script_files().map(|file| TsFile {
                path: packs_dir.join(file.path),
                content: file.content,
            }))
    }
}
