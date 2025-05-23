use tokio::sync::mpsc;
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
    sender: mpsc::Sender<UICommand>,
    flags: run_ui::Flags,
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
    pub(super) fn new() -> ApplicationRunner {
        ApplicationRunner {
            parsing_tasks: Vec::new(),
            watch: None,
            window: None,
            builder: None,
        }
    }

    pub(super) fn set_up_ui(&mut self) {
        if self.window.is_some() {
            todo!()
        } else if cfg!(feature = "ui") {
            #[cfg(feature = "ui")]
            {
                let (to_ui_sender, to_ui_receiver) = mpsc::channel::<UICommand>(100);
                let (from_ui_sender, from_ui_receiver) = mpsc::channel::<UIMessage>(100);

                println!("TODO: Remove this Debugprint");
                let message_handler = tokio::spawn(async move {
                    println!("Starting UI message handler");
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
                let flags = run_ui::Flags {
                    sender: from_ui_sender,
                    reciever: to_ui_receiver,
                };
                self.window = Some(UIWindow {
                    sender: to_ui_sender,
                    message_handler,
                    flags,
                });
            }
        } else {
            println!("TODO: UI feature not activated - handling");
        }
    }

    fn run_ui(mut self) -> ApplicationRunner {
        println!("join window task");
        if let Some(window_task) = self.window {
            println!(
                "Message handler still running: {}",
                !window_task.message_handler.is_finished()
            );
            run_ui::run(window_task.flags);
            window_task.message_handler.abort();
        }
        self.window = None;
        self
    }

    pub(super) async fn join_all_tasks(mut self) -> ApplicationRunner {
        self = self.run_ui();
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
