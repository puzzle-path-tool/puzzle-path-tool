use itertools::Itertools;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct CommonLoadedFile<'a> {
    relative_path: Cow<'a, str>,
    contents_str: Cow<'a, str>,
}

impl LoadedFile for CommonLoadedFile<'_> {
    fn contents(&self) -> Cow<str> {
        Cow::Borrowed(self.contents_str.as_ref())
    }

    fn relative_path(&self) -> Cow<str> {
        Cow::Borrowed(self.relative_path.as_ref())
    }

    fn is(&self, other: &Self) -> bool {
        self.relative_path == other.relative_path
    }
}

pub trait LoadedFile {
    fn contents(&self) -> Cow<str>;
    fn relative_path(&self) -> Cow<str>;
    fn is(&self, other: &Self) -> bool;
    fn to_common_file(&self) -> CommonLoadedFile {
        CommonLoadedFile {
            relative_path: self.relative_path(),
            contents_str: self.contents(),
        }
    }
}

pub mod configs {
    use indoc::indoc;
    use std::borrow::Cow;

    use crate::files::LoadedFile;

    #[iftree::include_file_tree(
        "
base_folder = 'ts'

paths = '''
/.editorconfig
/.gitignore
/eslint.config.js
/package-lock.json
/package.json
/tsconfig.json

!ts/node_modules
!ts/node_modules/**/*
'''
        "
    )]
    pub struct LoadedConfigFile {
        relative_path: &'static str,
        contents_str: &'static str,
    }

    const GITIGNORE_EXTRA: &str = indoc! {
        "
            packs/api/
            packs/core/
        "
    };

    impl LoadedFile for LoadedConfigFile {
        fn contents(&self) -> Cow<str> {
            if self.is(base::_GITIGNORE) {
                Cow::Owned(format!("{}{}", self.contents_str, GITIGNORE_EXTRA))
            } else {
                Cow::Borrowed(self.contents_str)
            }
        }

        fn relative_path(&self) -> Cow<str> {
            Cow::Borrowed(self.relative_path)
        }

        fn is(&self, other: &Self) -> bool {
            self.relative_path == other.relative_path
        }
    }
}

pub mod packs {
    use std::borrow::Cow;

    use crate::files::LoadedFile;

    #[iftree::include_file_tree(
        "
        base_folder = 'ts'
        paths = '/packs/**/*.ts'
        "
    )]
    pub struct LoadedPackFile {
        relative_path: &'static str,
        contents_str: &'static str,
    }

    impl LoadedFile for LoadedPackFile {
        fn contents(&self) -> Cow<str> {
            Cow::Borrowed(self.contents_str)
        }

        fn relative_path(&self) -> Cow<str> {
            Cow::Borrowed(self.relative_path)
        }

        fn is(&self, other: &Self) -> bool {
            self.relative_path == other.relative_path
        }
    }
}

pub mod puzzles {
    use std::borrow::Cow;

    use crate::files::LoadedFile;

    #[iftree::include_file_tree(
        "
        base_folder = 'ts'
        paths = '/puzzles/**/*.ts'
        "
    )]
    pub struct LoadedPuzzleFile {
        relative_path: &'static str,
        contents_str: &'static str,
    }

    impl LoadedFile for LoadedPuzzleFile {
        fn contents(&self) -> Cow<str> {
            Cow::Borrowed(self.contents_str)
        }

        fn relative_path(&self) -> Cow<str> {
            Cow::Borrowed(self.relative_path)
        }

        fn is(&self, other: &Self) -> bool {
            self.relative_path == other.relative_path
        }
    }
}

pub fn all_assets() -> impl Iterator<Item = CommonLoadedFile<'static>> {
    configs::ASSETS
        .iter()
        .map(|f| f.to_common_file())
        .chain(puzzles::ASSETS.iter().map(|f| f.to_common_file()))
        .chain(packs::ASSETS.iter().map(|f| f.to_common_file()))
}

pub fn print_file_paths() {
    let file = configs::base::_GITIGNORE.contents();
    println!("{file}\n\n|\n");

    let assets = &packs::ASSETS;
    println!(
        "Pack Files: \n{}\n",
        assets.iter().map(|f| f.relative_path()).join("\n")
    );

    let assets = &configs::ASSETS;
    println!(
        "Config Files: \n{}\n",
        assets.iter().map(|f| f.relative_path()).join("\n")
    );

    let assets = &puzzles::ASSETS;
    println!(
        "Puzzle Files: \n{}\n",
        assets.iter().map(|f| f.relative_path()).join("\n")
    );

    let assets = all_assets();
    println!(
        "All Files: \n{}\n",
        assets.map(|f| f.relative_path().into_owned()).join("\n")
    );
}
