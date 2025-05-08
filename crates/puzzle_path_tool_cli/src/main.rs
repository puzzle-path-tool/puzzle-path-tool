use iced::{
    widget::{self, Column},
    window::{self, Icon, icon},
};

#[derive(Default, Debug)]
struct State {
    value: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}

impl State {
    pub fn view(&self) -> Column<Message> {
        widget::column![
            widget::button("+").on_press(Message::Increment),
            widget::text(self.value).size(50),
            widget::button("-").on_press(Message::Decrement),
        ]
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => self.value += 1,
            Message::Decrement => self.value -= 1,
        }
    }
}

fn get_icon() -> anyhow::Result<Icon> {
    let icon = include_bytes!("../assets/icon.png");
    let icon = image::load_from_memory_with_format(icon, image::ImageFormat::Png)?;

    let icon = icon::from_rgba(
        icon.clone().into_rgba8().into_raw(),
        icon.width(),
        icon.height(),
    )?;
    Ok(icon)
}

fn main() -> anyhow::Result<()> {
    println!("Starting Gui...");

    let window_settings = window::Settings {
        icon: get_icon().ok(),
        ..Default::default()
    };

    iced::application("Test Window", State::update, State::view)
        .window(window_settings)
        .run()?;

    Ok(())
}
