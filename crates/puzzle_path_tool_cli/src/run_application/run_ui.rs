use std::{ffi::OsString, str::FromStr, sync::atomic};

use iced::{
    widget::{self},
    window::{self, Icon, icon},
};

use tokio::sync::mpsc;

use super::{UICommand, UIMessage};

mod views;

#[derive(Debug)]
pub(super) struct Flags {
    pub(super) sender: mpsc::Sender<UIMessage>,
    pub(super) reciever: mpsc::Receiver<UICommand>,
}

#[derive(Debug, Clone)]
struct ExplorerObject {
    displayed_name: String,
    id: usize,
    expanable: Option<(Vec<ExplorerObject>, bool)>,
}

static COUNTER: atomic::AtomicUsize = atomic::AtomicUsize::new(1);
impl ExplorerObject {
    fn new(name: String) -> ExplorerObject {
        ExplorerObject {
            displayed_name: name,
            id: COUNTER.fetch_add(1, atomic::Ordering::Relaxed),
            expanable: None,
        }
    }
    fn build_folder(name: String, paths: &Vec<Vec<String>>) -> ExplorerObject {
        let mut new = ExplorerObject::new(name);
        for path in paths {
            new.push_path(path);
        }
        new
    }
    fn push(&mut self, object: ExplorerObject) {
        if let Some((children, _expanded)) = &mut self.expanable {
            children.push(object);
        } else {
            self.expanable = Some((vec![object], false));
        }
    }
    fn push_path(&mut self, path: &[String]) {
        if let Some((first, rest)) = path.split_first() {
            if let Some((object, _)) = &mut self.expanable {
                if let Some(preexisting) = object.iter_mut().find(|a| &a.displayed_name == first) {
                    preexisting.push_path(rest);
                } else {
                    let mut new = ExplorerObject::new(first.clone());
                    new.push_path(rest);
                    object.push(new);
                }
            } else {
                let mut new = ExplorerObject::new(first.clone());
                new.push_path(rest);
                self.push(new);
            }
        }
    }
    fn get_id(&self) -> usize {
        self.id
    }
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
    Increment,
    Decrement,
    FromStream { command: UICommand },
}

impl State {
    fn title(&self) -> String {
        println!("TODO: Build title with {self:?}");
        "Test Window".to_string()
    }

    fn new(flags: Flags) -> (Self, iced::Task<Message>) {
        let reciever = tokio_stream::wrappers::ReceiverStream::new(flags.reciever);
        let task = iced::Task::run(reciever, |c| Message::FromStream { command: c });
        (
            State {
                value: 10,
                recieved_from_stream: 0,
                sender: flags.sender,

                sudoku_explorer: {
                    let paths = [
                        vec!["examplefolder1", "examplefolder11", "examplefile111"],
                        vec!["examplefolder1", "examplefolder11", "examplefile112"],
                        vec!["examplefolder1", "examplefile12"],
                        vec!["examplefile2"],
                        vec!["examplefile3"],
                    ]
                    .iter_mut()
                    .map(|x| x.iter_mut().map(|z| (**z).to_string()).collect())
                    .collect();
                    ExplorerObject::build_folder("explorer".to_string(), &paths)
                },
            },
            task,
        )
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        widget::column![
            widget::row![
                views::explorer_view(self),
                views::sudoku_view(self),
                views::path_info_view(self)
            ]
            .spacing(10),
            views::control_view(self),
        ]
        .spacing(10)
        .padding(5)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
                let e = self.sender.try_send(UIMessage::MessageFromUI {
                    value: String::from_str("Increment").expect("str can't fail"),
                });
                println!("{e:?}");
            }
            Message::Decrement => {
                self.value -= 1;
                let e = self.sender.try_send(UIMessage::MessageFromUI {
                    value: String::from_str("Decrement").expect("str can't fail"),
                });
                println!("{e:?}");
            }
            Message::FromStream { command } => {
                _ = command;
                self.recieved_from_stream += 1;
            }
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
