use clap::Parser;

mod commands;
#[allow(dead_code)]
mod run_application;

fn main() {
    let args = commands::Cli::parse();

    match args.task {
        commands::Task::Stub {} => todo!(),
        commands::Task::Gen {
            input,
            generation_options,
        } => {
            let mut main_runner = run_application::MainRunner::new();
            match input {
                commands::Input::PuzzleLua {
                    path,
                    output_options,
                } => {
                    println!("TODO: Get PuzzleLua file at {path:?}");
                    println!("TODO: Generate Sudoku with {generation_options:?}");
                    handle_output(output_options, &mut main_runner);
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
                    handle_output(output_options, &mut main_runner);
                }
                commands::Input::FullJson {
                    path,
                    output_options,
                } => {
                    println!("TODO: Get FullJson at {path:#?}");
                    println!("TODO: Generate Sudoku with {generation_options:?}");
                    handle_output(output_options, &mut main_runner);
                }
            }

            let main_runner = main_runner.join_all_tasks();
            println!("Joined all tasks from main: {main_runner:?}");
        }
    }
}

//TODO add Sudoku field
fn handle_output(output: commands::OutputOptions, main_runner: &mut run_application::MainRunner) {
    if output.ui {
        main_runner.set_up_ui();
    }
    if let Some(format) = output.export_format {
        main_runner.set_up_export(format);
    }
    if let Some(path) = output.json_path {
        main_runner.set_up_json(path);
    }
}
