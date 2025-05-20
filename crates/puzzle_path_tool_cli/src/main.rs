use clap::Parser;

mod commands;
#[allow(dead_code)]
mod run_application;

fn main() {
    //run_application::run();
    let args = commands::Cli::parse();

    match args.task {
        commands::Task::Stub {} => todo!(),
        commands::Task::Gen {
            input,
            generation_options,
        } => {
            let application = run_application::ApplicationRunner::new();
            match input {
                commands::Input::PuzzleLua {
                    path,
                    output_options,
                } => {
                    println!("TODO: Get PuzzleLua file at {path:?}");
                    println!("TODO: Generate Sudoku with {generation_options:?}");
                    handle_output(output_options, application);
                }
                commands::Input::WorkspaceLua {
                    path,
                    puzzlenames,
                    output_options,
                } => {
                    println!("TODO: Get WorkspaceLua file at {path:?}");
                    if puzzlenames.is_empty() {
                        println!("TODO: Generate all Sudokus with {generation_options:?}");
                    } else {
                        println!("TODO: Generate {puzzlenames:?} with {generation_options:?}");
                    }
                    handle_output(output_options, application);
                }
                commands::Input::FullJson {
                    path,
                    output_options,
                } => {
                    println!("TODO: Get FullJson at {path:#?}");
                    println!("TODO: Generate Sudoku with {generation_options:?}");
                    handle_output(output_options, application);
                }
            }
        }
    }
}

//TODO add Sudoku field
fn handle_output(output: commands::OutputOptions, mut application: run_application::ApplicationRunner) {
    #[allow(clippy::manual_assert)]
    if output.ui {
        application.start_ui();
    }
    if let Some(format) = output.export_format {
        println!("TODO: put Sudoku as JSON/URL in Terminal for {format:?}");
    }
    if let Some(path) = output.json_path {
        println!("TODO: create full Json file at {path:?}");
    }
}
