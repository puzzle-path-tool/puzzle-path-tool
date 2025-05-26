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
                    main_runner.set_generation_options(generation_options);
                    main_runner.build_with_lua_file(path);
                    handle_output(output_options, &mut main_runner);
                }
                commands::Input::WorkspaceLua {
                    path,
                    puzzlenames,
                    output_options,
                } => {
                    main_runner.set_generation_options(generation_options);
                    main_runner.build_with_workspace(path, puzzlenames);
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
