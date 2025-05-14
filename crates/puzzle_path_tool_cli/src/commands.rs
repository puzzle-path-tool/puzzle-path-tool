use std::ffi::OsString;

use clap::{Args, Parser, Subcommand, ValueEnum};

#[derive(Debug, Parser)]
#[command(name = "puzzpt")]
#[command(about = "TODO(0): Explaining overall CLI", long_about = None)]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) task: Task,
}

#[derive(Debug, Subcommand)]
#[command(about = "TODO(1): task type", long_about = None)]
pub(super) enum Task {
    /// TODO(1.1): Explaining Stub Command
    Stub {
        //TODO
    },
    /// TODO(1.2): Explaining Gen Command
    Gen {
        #[command(subcommand)]
        input: Input,

        #[command(flatten)]
        generation_options: GenerationOptions,
    },
}

#[derive(Args, Debug)]
#[command(about = "TODO(2): Generation Options", long_about = None)]
pub(super) struct GenerationOptions {
    ///TODO(2.1): Script-Dir option text
    #[arg(short = 's', long = "script-dir")]
    pub(super) scriptdir: Option<OsString>,

    ///TODO(2.2): Seed option text
    #[arg(short = 'r', long = "rand-seed")]
    pub(super) seed: Option<String>,

    ///TODO(2.3): Watch option text
    #[arg(short = 'w', long = "watch")]
    pub(super) watch: bool,

    ///TODO(2.4): Cache Strategies option text
    #[arg(short = 'c', long = "cache")]
    pub(super) cache: Option<CacheStrategie>,

    ///TODO(2.5): Script-Dir option text
    #[arg(short = 't', long = "type-defs")]
    pub(super) typedefinitions: Option<OsString>,
}

#[derive(Args, Debug)]
#[command(about = "TODO(3): Output Options", long_about = None)]
pub(super) struct OutputOptions {
    ///TODO(3.1): Explaining UI command
    #[arg(short = 'u', long = "web-ui")]
    pub(super) ui: bool,
    ///TODO(3.2): Explaining fulljson command
    #[arg(short = 'j', long = "json-output")]
    pub(super) json_path: Option<OsString>,
    ///TODO(3.3): Explaining Export command
    #[arg(short = 'f', long = "export-format")]
    pub(super) export_format: Option<ExportFormat>,
}

#[derive(Debug, Subcommand)]
#[command(about = "TODO(4): Input Options", long_about = None)]
pub(super) enum Input {
    ///TODO(4.1): Explaining Puzzlelua Command
    PuzzleLua {
        #[command(flatten)]
        output_options: OutputOptions,
        ///TODO(4.1.1): Explain Path to lua file
        path: OsString,
    },
    ///TODO(4.2): Explaining Workspacelua Command
    WorkspaceLua {
        #[command(flatten)]
        output_options: OutputOptions,
        ///TODO(4.2.1): Explain Path to lua file
        path: OsString,
        ///TODO(4.2.2): Explain optional puzzlenames
        #[arg(last = true)]
        puzzlenames: Vec<OsString>,
    },
    ///TODO(4.3): Explaining fulljson command
    FullJson {
        #[command(flatten)]
        output_options: OutputOptions,
        ///TODO(4.3.1):Explain Path to json
        path: OsString,
    },
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum ExportFormat {
    SudokuPad,
    FPuzzles,
}

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Eq)]
pub(super) enum CacheStrategie {
    ExampleCacheStragegie,
}
