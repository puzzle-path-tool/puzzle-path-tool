use crate::commands::{GenerationOptions, Input, OutputOptions};
use run_ui::Flags;
use std::thread::JoinHandle;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_stream::StreamExt;

#[cfg(feature = "ui")]
mod run_ui;

#[derive(Debug)]
struct BuildingTask {
    join_handler: tokio::task::JoinHandle<()>,
}

#[derive(Debug)]
struct WatchTask {
    join_handler: tokio::task::JoinHandle<()>,
}

#[derive(Debug)]
enum UIWindow {
    Closed,
    SetUp {
        sender: Sender<UICommand>,
        receiver: Receiver<UIMessage>,
    },
    Running {
        sender: Sender<UICommand>,
        message_handler: tokio::task::JoinHandle<()>,
    },
}

impl UIWindow {
    fn set_up_ui(self) -> UIWindow {
        if let UIWindow::SetUp { sender, receiver } = self {
            if cfg!(feature = "ui") {
                #[cfg(feature = "ui")]
                {
                    println!("Building UI message handler");
                    let message_handler = tokio::spawn(async move {
                        println!("Finished UI message handler");
                        let mut stream = tokio_stream::wrappers::ReceiverStream::new(receiver);
                        while let Some(ui_message) = stream.next().await {
                            match ui_message {
                                UIMessage::MessageFromUI { value } => {
                                    println!("TODO: Handle UIMessage {value}");
                                }
                                UIMessage::WindowClosed => {
                                    stream.close();
                                    break;
                                }
                            }
                        }

                        println!("Ending UI message handler");
                    });
                    UIWindow::Running {
                        sender,
                        message_handler,
                    }
                }
            } else {
                println!("TODO: 'UI feature not activated' Warning");
                UIWindow::Closed
            }
        } else {
            self
        }
    }
}

#[derive(Debug, Clone)]
enum UIMessage {
    //TODO
    MessageFromUI { value: String },
    WindowClosed,
}

#[derive(Debug, Clone, Copy)]
enum UICommand {
    //TODO
    CommandToUI,
}

#[derive(Debug)]
pub(super) struct ApplicationRunner {
    watch: Option<WatchTask>,
    window: UIWindow,
    builder: Option<BuildingTask>,
}

impl ApplicationRunner {
    fn run_new(input: Input, options: GenerationOptions) -> (Option<Flags>, JoinHandle<()>) {
        let mut runner = ApplicationRunner {
            watch: None,
            window: UIWindow::Closed,
            builder: None,
        };
        let mut ui_flags = None;

        match input {
            Input::PuzzleLua {
                path,
                output_options,
            } => {
                //main_runner.set_generation_options(generation_options);
                //main_runner.build_with_lua_file(path);
                ui_flags = runner.setup_output(&output_options);
            }
            Input::WorkspaceLua {
                path,
                puzzlenames,
                output_options,
            } => {
                //main_runner.set_generation_options(generation_options);
                //main_runner.build_with_workspace(path, puzzlenames);
                ui_flags = runner.setup_output(&output_options);
            }
        }

        let handle = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new();
            match rt {
                Ok(rt) => rt.block_on(async move {
                    runner.window = runner.window.set_up_ui();

                    runner.join_all_tasks().await;
                }),
                Err(_) => todo!(),
            }
        });

        (ui_flags, handle)
    }

    fn setup_output(&mut self, options: &OutputOptions) -> Option<Flags> {
        let mut ui_flags = None;
        if options.ui {
            if cfg!(feature = "ui") {
                #[cfg(feature = "ui")]
                {
                    let (to_ui_sender, to_ui_receiver) = mpsc::channel::<UICommand>(100);
                    let (from_ui_sender, from_ui_receiver) = mpsc::channel::<UIMessage>(100);

                    let flags = run_ui::Flags {
                        sender: from_ui_sender,
                        reciever: to_ui_receiver,
                    };
                    self.window = UIWindow::SetUp {
                        sender: to_ui_sender,
                        receiver: from_ui_receiver,
                    };
                    ui_flags = Some(flags);
                }
            } else {
                println!("TODO: ui-features not implemented");
            }
        }
        if let Some(format) = &options.export_format {
            println!("TODO: handle output Format {format:?}");
        }
        if let Some(path) = &options.json_path {
            println!("TODO: handle output Json path {path:?}");
        }

        ui_flags
    }

    async fn join_all_tasks(self) {
        //Add all tasks
        println!("joining all remaining logic tasks");
        if let UIWindow::Running {
            message_handler,
            sender: _,
        } = self.window
        {
            let _ = message_handler.await;
        }
    }
}

#[derive(Debug)]
pub(super) struct MainRunner {
    ui_flags: Option<run_ui::Flags>,
    logic_thread_handler: std::thread::JoinHandle<()>,
}

impl MainRunner {
    pub(super) fn new(input: Input, options: GenerationOptions) -> MainRunner {
        let (flags, handle) = ApplicationRunner::run_new(input, options);
        MainRunner {
            ui_flags: flags,
            logic_thread_handler: handle,
        }
    }

    fn run_ui(mut self) -> MainRunner {
        println!("Run UI with {:?}", self.ui_flags);
        if let Some(flags) = self.ui_flags {
            run_ui::run(flags);
        }
        self.ui_flags = None;
        self
    }

    pub(super) fn join_all_tasks(mut self) {
        self = self.run_ui();
        //TODO: remove println
        let e = self.logic_thread_handler.join();
        println!("join logic thread: {e:?}");
    }
}
