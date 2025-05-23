use std::str::FromStr;

use iced::{
    widget::{self, Row},
    window::{self, Icon, icon},
};

use tokio::sync::mpsc;

use super::{UICommand, UIMessage};

#[derive(Debug)]
pub(super) struct Flags {
    pub(super) sender: mpsc::Sender<UIMessage>,
    pub(super) reciever: mpsc::Receiver<UICommand>,
}

#[derive(Debug)]
struct State {
    value: i32,
    recieved_from_stream: u32,
    sender: mpsc::Sender<UIMessage>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
    FromStream { command: UICommand },
}

impl State {
    fn new(flags: Flags) -> (Self, iced::Task<Message>) {
        let reciever = tokio_stream::wrappers::ReceiverStream::new(flags.reciever);
        let task = iced::Task::run(reciever, |c| Message::FromStream { command: c });
        (
            State {
                value: 10,
                recieved_from_stream: 0,
                sender: flags.sender,
            },
            task,
        )
    }

    pub fn view(&self) -> Row<Message> {
        widget::row![
            widget::column![
                widget::button("+").on_press(Message::Increment),
                widget::text(self.value).size(50),
                widget::button("-").on_press(Message::Decrement),
            ],
            widget::text("recieved"),
            widget::text(self.recieved_from_stream).size(50)
        ]
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
    let _ = iced::application("Test Window", State::update, State::view)
        .window(window_settings)
        .run_with(|| State::new(flags));
    //TODO: Errorhandling
    let _ = sender.try_send(UIMessage::WindowClosed);
    println!("Gui closed");
}
