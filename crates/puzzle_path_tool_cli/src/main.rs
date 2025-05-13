use std::env::args;

use clap::Parser;

mod commands;
#[allow(dead_code)]
mod run_application;

fn main() {
    //run_application::run();

    println!("Args before parsing: {:?}", args());

    let args = commands::Cli::parse();

    println!("Args after parsing: {args:?}");
}
