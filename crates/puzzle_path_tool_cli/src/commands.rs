use std::ffi::OsString;

use clap::{Parser, Subcommand, ValueEnum};

///TODO(1): Explaining overall CLI
#[derive(Debug, Parser)]
#[command(name = "puzzpt")]
#[command(about = "TODO(1.1): Explaining overall CLI", long_about = None)]
pub(super) struct Cli {
    ///TODO(2): The specified output
    output: Output,

    #[command(subcommand)]
    input: Input,

    ///TODO(4): Script-Dir option text
    #[arg(short = 's', long = "script-dir")]
    scriptdir: Option<OsString>,

    ///TODO(5): Seed option text
    #[arg(short = 'r', long = "rand-seed")]
    seed: Option<String>,

    ///TODO(6): Watch option text
    #[arg(short = 'w', long = "watch")]
    watch: bool,

    ///TODO(7): Cache Strategies option text
    #[arg(short = 'c', long = "cache")]
    cache: Option<CacheStrategie>,

    ///TODO(8): Script-Dir option text
    #[arg(short = 't', long = "type-defs")]
    typedefinitions: Option<OsString>,
}

#[derive(Debug, Subcommand)]
enum Input {
    ///TODO(9): Explaining Puzzlelua Command
    PuzzleLua {
        ///TODO(10): Explain Path to lua file
        path: OsString,
    },
    ///TODO(11): Explaining Workspacelua Command
    WorkspaceLua {
        ///TODO(12): Explain Path to lua file
        path: OsString,
        ///TODO(13): Explain optional puzzlenames
        #[arg(last = true)]
        puzzlenames: Vec<OsString>,
    },
    ///TODO(14): Explaining fulljson command
    FullJson {
        ///TODO(15):Explain Path to json
        path: OsString,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum Output {
    UI,
    //Todo:Rename
    FullJson,
    Export,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
enum CacheStrategie {
    ExampleCacheStragegie,
}
