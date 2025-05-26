use crate::commands::{ExportFormat, GenerationOptions};
use std::ffi::OsString;
use tokio::sync::mpsc::{self, Receiver, Sender};
use tokio_stream::StreamExt;

#[cfg(feature = "ui")]
mod run_ui;

#[derive(Debug)]
struct ParsingTask {}

#[derive(Debug)]
struct BuildingTask {
    join_handler: tokio::task::JoinHandle<()>,
}

#[derive(Debug)]
struct WatchTask {
    join_handler: tokio::task::JoinHandle<()>,
}

#[derive(Debug)]
struct UIWindow {
    sender: Sender<UICommand>,
    message_handler: tokio::task::JoinHandle<()>,
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
    parsing_tasks: Vec<ParsingTask>,
    watch: Option<WatchTask>,
    window: Option<UIWindow>,
    builder: Option<BuildingTask>,
}

impl ApplicationRunner {
    fn run_new(receiver: Receiver<LogicThreadCommand>) {
        let mut runner = ApplicationRunner {
            parsing_tasks: Vec::new(),
            watch: None,
            window: None,
            builder: None,
        };

        let rt = tokio::runtime::Runtime::new();
        match rt {
            Ok(rt) => rt.block_on(async move {
                let main_message_handler = tokio::spawn(async move {
                    println!("Starting Main message handler");
                    let mut stream = tokio_stream::wrappers::ReceiverStream::new(receiver);
                    while let Some(command) = stream.next().await {
                        println!("MainMessageHandler recieves {command:?}");
                        match command {
                            LogicThreadCommand::SetUpExport { format } => {
                                println!("TODO: put Sudoku as JSON/URL in Terminal for {format:?}");
                            }
                            LogicThreadCommand::SetUpJson { path } => {
                                println!("TODO: create full Json file at {path:?}");
                            }
                            LogicThreadCommand::SetUpUI { sender, receiver } => {
                                runner.set_up_ui(sender, receiver);
                            }
                            LogicThreadCommand::Join => {
                                runner.join_all_tasks().await;
                                break;
                            }
                            LogicThreadCommand::BuildFromLuaFile { path } => {
                                println!("TODO: Get PuzzleLua file at {path:?}");
                            }
                            LogicThreadCommand::BuildFromWorkSpace { path, puzzlenames } => {
                                if puzzlenames.is_empty() {
                                    println!("TODO: Generate all Sudokus at {path:?}");
                                } else {
                                    println!("TODO: Generate {puzzlenames:?} at {path:?}");
                                }
                            }
                            LogicThreadCommand::SetGenerationOptions { options } => {
                                println!("TODO: Generate Sudoku with {options:?}");
                            }
                        }
                    }
                    println!("Ending Main message handler");
                });
                let _todo = main_message_handler.await;
            }),
            Err(_) => todo!(),
        }
    }

    fn set_up_ui(
        &mut self,
        to_ui_sender: Sender<UICommand>,
        from_ui_receiver: Receiver<UIMessage>,
    ) {
        if cfg!(feature = "ui") {
            #[cfg(feature = "ui")]
            {
                println!("Building UI message handler");
                let message_handler = tokio::spawn(async move {
                    println!("Finished UI message handler");
                    let mut stream = tokio_stream::wrappers::ReceiverStream::new(from_ui_receiver);
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
                self.window = Some(UIWindow {
                    sender: to_ui_sender,
                    message_handler,
                });
            }
        }
    }

    async fn join_all_tasks(mut self) -> ApplicationRunner {
        //TODO: remove println
        println!("join watch task");
        if let Some(watch_task) = self.watch {
            let _ = watch_task.join_handler.await;
            self.watch = None;
        }
        //TODO: remove println
        println!("join builder task");
        if let Some(bilder_task) = self.builder {
            let _ = bilder_task.join_handler.await;
            self.builder = None;
        }

        self
    }
}

#[derive(Debug)]
pub(super) struct MainRunner {
    ui_flags: Option<run_ui::Flags>,
    logic_thread_sender: Sender<LogicThreadCommand>,
    logic_thread_handler: std::thread::JoinHandle<()>,
}

#[derive(Debug)]
enum LogicThreadCommand {
    BuildFromLuaFile {
        path: OsString,
    },
    BuildFromWorkSpace {
        path: OsString,
        puzzlenames: Vec<String>,
    },
    SetGenerationOptions {
        options: GenerationOptions,
    },
    SetUpUI {
        sender: Sender<UICommand>,
        receiver: Receiver<UIMessage>,
    },
    SetUpExport {
        format: ExportFormat,
    },
    SetUpJson {
        path: OsString,
    },
    Join,
}

impl MainRunner {
    pub(super) fn new() -> MainRunner {
        let (sender, receiver) = mpsc::channel::<LogicThreadCommand>(100);
        let handle = std::thread::spawn(move || {
            ApplicationRunner::run_new(receiver);
        });
        MainRunner {
            ui_flags: None,
            logic_thread_sender: sender,
            logic_thread_handler: handle,
        }
    }

    pub(super) fn build_with_workspace(&self, path: OsString, puzzlenames: Vec<String>) {
        let e = self
            .logic_thread_sender
            .blocking_send(LogicThreadCommand::BuildFromWorkSpace { path, puzzlenames });
        println!("Build from Workspace: {e:?}");
    }

    pub(super) fn build_with_lua_file(&self, path: OsString) {
        let e = self
            .logic_thread_sender
            .blocking_send(LogicThreadCommand::BuildFromLuaFile { path });
        println!("Build from Lua file: {e:?}");
    }

    pub(super) fn set_generation_options(&self, options: GenerationOptions) {
        let e = self
            .logic_thread_sender
            .blocking_send(LogicThreadCommand::SetGenerationOptions { options });
        println!("Set GenerationOptions: {e:?}");
    }

    pub(super) fn set_up_export(&self, format: ExportFormat) {
        let e = self
            .logic_thread_sender
            .blocking_send(LogicThreadCommand::SetUpExport { format });
        println!("SetUp export: {e:?}");
    }

    pub(super) fn set_up_json(&self, path: OsString) {
        let e = self
            .logic_thread_sender
            .blocking_send(LogicThreadCommand::SetUpJson { path });
        println!("SetUp export: {e:?}");
    }

    pub(super) fn set_up_ui(&mut self) {
        if self.ui_flags.is_some() {
            todo!()
        } else if cfg!(feature = "ui") {
            #[cfg(feature = "ui")]
            {
                let (to_ui_sender, to_ui_receiver) = mpsc::channel::<UICommand>(100);
                let (from_ui_sender, from_ui_receiver) = mpsc::channel::<UIMessage>(100);

                let e = self
                    .logic_thread_sender
                    .blocking_send(LogicThreadCommand::SetUpUI {
                        sender: to_ui_sender,
                        receiver: from_ui_receiver,
                    });
                println!("Set up UI channel with logic_thread: {e:?}");
                let flags = run_ui::Flags {
                    sender: from_ui_sender,
                    reciever: to_ui_receiver,
                };
                self.ui_flags = Some(flags);
            }
        } else {
            println!("TODO: UI feature not activated - handling");
        }
    }

    fn run_ui(mut self) -> MainRunner {
        println!("join window task");
        if let Some(flags) = self.ui_flags {
            run_ui::run(flags);
        }
        self.ui_flags = None;
        self
    }

    pub(super) fn join_all_tasks(
        mut self,
    ) -> std::result::Result<(), std::boxed::Box<(dyn std::any::Any + std::marker::Send + 'static)>>
    {
        self = self.run_ui();
        //TODO: remove println
        let e = self.logic_thread_sender.try_send(LogicThreadCommand::Join);
        println!("join logic thread: {e:?}");
        self.logic_thread_handler.join()
    }
}
