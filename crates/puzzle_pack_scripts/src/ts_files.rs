use std::path::{Path, PathBuf};

use const_format::concatcp;
use include_dir::{Dir, include_dir};

#[derive(Debug, Clone)]
pub struct TsFiles {
    pub editorconfig: LoadedFile<'static>,
    pub gitignore: LoadedFile<'static>,
    pub eslint_config: LoadedFile<'static>,
    pub package_json: LoadedFile<'static>,
    pub package_lock: LoadedFile<'static>,
    pub tsconfig: LoadedFile<'static>,
    pub example_puzzle: LoadedFile<'static>,
    pub packs: Dir<'static>,
}

const GITIGNORE_EXTRA: &str = "\
packs/api/
packs/core/
";

macro_rules! include_str_ts {
    ($name:literal $(, $appendix:expr)* $(,)?) => {
        LoadedFile {
            path: Path::new($name).to_path_buf(),
            content: concatcp!(
                include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/ts/", $name)),
                $($appendix),*
            ),
        }
    };
}

#[derive(Debug, Clone)]
pub struct LoadedFile<'a> {
    pub path: PathBuf,
    pub content: &'a str,
}

impl TsFiles {
    #[must_use]
    pub fn load() -> Self {
        TsFiles {
            editorconfig: include_str_ts!(".editorconfig"),
            gitignore: include_str_ts!(".gitignore", GITIGNORE_EXTRA),
            eslint_config: include_str_ts!("eslint.config.js"),
            package_json: include_str_ts!("package.json"),
            package_lock: include_str_ts!("package-lock.json"),
            tsconfig: include_str_ts!("tsconfig.json"),
            example_puzzle: include_str_ts!("puzzles/example.ts"),
            packs: include_dir!("$CARGO_MANIFEST_DIR/ts/packs"),
        }
    }

    #[allow(clippy::expect_used, clippy::missing_panics_doc)]
    pub fn script_files(&self) -> impl Iterator<Item = LoadedFile<'static>> {
        let entries = self.packs.find("**/*.ts").expect("invalid Pattern");

        entries.filter_map(|entry| {
            let path = entry.path();
            entry
                .as_file()
                .and_then(|f| f.contents_utf8())
                .map(|content| LoadedFile {
                    path: path.to_path_buf(),
                    content,
                })
        })
    }

    pub fn config_files(&self) -> impl Iterator<Item = LoadedFile<'static>> {
        [
            &self.editorconfig,
            &self.gitignore,
            &self.eslint_config,
            &self.package_json,
            &self.package_lock,
            &self.tsconfig,
            &self.example_puzzle,
        ]
        .map(std::clone::Clone::clone)
        .into_iter()
    }

    pub fn all_files(&self) -> impl Iterator<Item = LoadedFile<'static>> {
        let packs_dir = Path::new("packs/");

        self.config_files()
            .chain(self.script_files().map(|file| LoadedFile {
                path: packs_dir.join(file.path),
                content: file.content,
            }))
    }
}
