use iced::{
    widget::{self},
    window::{self, Icon, icon},
};

use tokio::sync::mpsc;

use puzzle_core::explorer_collection::{ExplorerId, ExplorerObject};

use super::{UICommand, UIMessage};
mod views;

#[derive(Debug)]
pub(super) struct Flags {
    pub(super) sender: mpsc::Sender<UIMessage>,
    pub(super) reciever: mpsc::Receiver<UICommand>,
}

#[derive(Debug, Clone, Copy)]
enum ExplorerViewState<'a> {
    NonExistent,
    Expandable,
    Open {
        explorer_stucture: &'a Vec<ExplorerObject>,
    },
}

#[derive(Debug)]
struct State {
    value: i32,
    recieved_from_stream: u32,
    sender: mpsc::Sender<UIMessage>,

    sudoku_explorer: ExplorerObject,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    FromExplorer { message: ExplorerMessage },
    FromController { message: ControllerMessage },
    FromDetails { message: DetailsMessage },
    FromSudokuCanvas { message: SudokuCanvasMessage },
    Command { command: UICommand },
}

#[derive(Debug, Clone, Copy)]
enum ExplorerMessage {
    Expand { id: ExplorerId, value: bool },
    Selected { id: ExplorerId },
}

#[derive(Debug, Clone, Copy)]
enum ControllerMessage {
    Example,
}

#[derive(Debug, Clone, Copy)]
enum DetailsMessage {
    Example,
}

#[derive(Debug, Clone, Copy)]
enum SudokuCanvasMessage {
    Example,
}

impl State {
    fn title(&self) -> String {
        println!("TODO: Build title with {self:?}");
        "Test Window".to_string()
    }

    fn new(flags: Flags) -> (Self, iced::Task<Message>) {
        let reciever = tokio_stream::wrappers::ReceiverStream::new(flags.reciever);
        let task = iced::Task::run(reciever, |c| Message::Command { command: c });
        (
            State {
                value: 10,
                recieved_from_stream: 0,
                sender: flags.sender,

                sudoku_explorer: {
                    let mut paths = [
                        vec!["examplefolder1", "examplefolder11", "examplefile111"],
                        vec!["examplefolder1", "examplefolder11", "examplefile112"],
                        vec![
                            "examplefolder1",
                            "examplefolder11",
                            "loooooooooooooooooooooooooongexamplefile113",
                        ],
                        vec!["examplefolder1", "examplefile12"],
                        vec!["examplefile2"],
                        vec!["examplefile3"],
                    ];
                    let mut vec: Vec<u32> = (1..15).collect();
                    let mut vec: Vec<Vec<String>> = vec
                        .iter_mut()
                        .map(|x| {
                            vec![
                                "long examplefolder4".to_string(),
                                format!("examplefile4{}", x),
                            ]
                        })
                        .collect();
                    let mut paths: Vec<Vec<String>> = paths
                        .iter_mut()
                        .map(|x| x.iter_mut().map(|z| (**z).to_string()).collect())
                        .collect();
                    paths.append(&mut vec);
                    ExplorerObject::build_folder("explorer".to_string(), &paths)
                },
            },
            task,
        )
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        widget::column![
            widget::row!()
                .push_maybe(
                    views::explorer_view(&self.sudoku_explorer)
                        .map(|element| element.map(|m| Message::FromExplorer { message: m }))
                )
                .push(views::sudoku_view(self).map(|m| Message::FromSudokuCanvas { message: m }))
                .push(views::path_info_view(self).map(|m| Message::FromDetails { message: m }))
                .spacing(10),
            views::control_view(self).map(|m| Message::FromController { message: m }),
        ]
        .spacing(10)
        .padding(5)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Command { command } => {
                _ = command;
                self.recieved_from_stream += 1;
            }
            Message::FromExplorer { message } => {
                self.update_explorer(message);
            }
            Message::FromController { message } => {
                self.update_controller(message);
            }
            Message::FromDetails { message } => {
                self.update_details(message);
            }
            Message::FromSudokuCanvas { message } => {
                self.update_sudoku_canvas(message);
            }
        }
    }

    pub fn update_controller(&mut self, message: ControllerMessage) {
        match message {
            ControllerMessage::Example => todo!(),
        }
    }

    pub fn update_details(&mut self, message: DetailsMessage) {
        match message {
            DetailsMessage::Example => todo!(),
        }
    }

    pub fn update_sudoku_canvas(&mut self, message: SudokuCanvasMessage) {
        match message {
            SudokuCanvasMessage::Example => todo!(),
        }
    }

    pub fn update_explorer(&mut self, message: ExplorerMessage) {
        match message {
            ExplorerMessage::Expand { id, value } => {
                if let Some(object) = self.sudoku_explorer.get_by_id_mut(id) {
                    object.expand(value);
                } else {
                    println!("TODO: Errorhandling for missing objects in Explorer");
                }
            }
            ExplorerMessage::Selected { id } => println!(
                "TODO: Explorerobject {} selected",
                self.sudoku_explorer
                    .get_by_id(id)
                    .map_or("NOT FOUND", |x| { x.name() })
            ),
        }
    }
}

fn load_icon() -> anyhow::Result<Icon> {
    let bytes = include_bytes!("../../assets/icon.png");
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;

    let width = image.width();
    let height = image.height();

    let icon = icon::from_rgba(image.into_rgba8().into_raw(), width, height)?;
    Ok(icon)
}

pub(super) fn run(flags: Flags) {
    println!("Starting Gui...");
    let window_settings = window::Settings {
        icon: load_icon().ok(),
        ..Default::default()
    };
    let sender = flags.sender.clone();
    let _ = iced::application(State::title, State::update, State::view)
        .window(window_settings)
        .run_with(|| State::new(flags));
    //TODO: Errorhandling
    let _ = sender.try_send(UIMessage::WindowClosed);
    println!("Gui closed");
}
