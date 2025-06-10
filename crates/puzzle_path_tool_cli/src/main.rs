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
            let main_runner = run_application::MainRunner::new(input, generation_options);

            println!("Joined all tasks from main");
            main_runner.join_all_tasks();
        }
    }
}
