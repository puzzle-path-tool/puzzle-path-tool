use iced::{
    widget::{self, Column},
    window::{self, Icon, icon},
};

#[derive(Debug)]
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

impl Default for State {
    fn default() -> Self {
        State { value: 10 }
    }
}

fn load_icon() -> anyhow::Result<Icon> {
    let bytes = include_bytes!("../assets/icon.png");
    let image = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)?;

    let width = image.width();
    let height = image.height();

    let icon = icon::from_rgba(image.into_rgba8().into_raw(), width, height)?;
    Ok(icon)
}

fn main() -> anyhow::Result<()> {
    println!("Starting Gui...");

    let window_settings = window::Settings {
        icon: load_icon().ok(),
        ..Default::default()
    };

    iced::application("Test Window", State::update, State::view)
        .window(window_settings)
        .run()?;

    Ok(())
}
