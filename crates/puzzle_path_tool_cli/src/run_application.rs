use tokio::sync::mpsc;
use tokio_stream::StreamExt;

#[cfg(feature = "ui")]
mod run_ui;

#[derive(Debug)]
struct ParsingTask {}

#[derive(Debug)]
struct BuildingTask {}

#[derive(Debug)]
struct WatchTask {}

#[derive(Debug)]
struct UIWindow {
    sender: mpsc::Sender<run_ui::UICommand>
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

    pub(super) fn start_ui(&mut self) {
        if self.window.is_none() {
            let (to_ui_sender, to_ui_receiver) = mpsc::channel::<run_ui::UICommand>(100);
            let (from_ui_sender, from_ui_receiver) = mpsc::channel::<run_ui::UIMessage>(100);

            self.window = Some(UIWindow {
                sender: to_ui_sender
            });

            tokio::spawn(async move {
                let mut stream = tokio_stream::wrappers::ReceiverStream::new(from_ui_receiver);

                while let Some(ui_message) = stream.next().await {
                    match ui_message {
                        run_ui::UIMessage::MessageFromUI { value } => {
                            println!("TODO: Handle UIMessage {value}");
                        },
                    }
                }
            });

            let flags = run_ui::Flags {
                sender: from_ui_sender,
                reciever: to_ui_receiver,
            };
            run_ui::run(flags);
        }
    }
}
