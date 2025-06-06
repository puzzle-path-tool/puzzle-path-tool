use iced::{
    Subscription,
    time::Duration,
    widget,
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
    title: String,

    sender: mpsc::Sender<UIMessage>,

    sudoku_explorer: ExplorerObject,

    control: ControlState,
}

#[derive(Debug, Clone, Copy)]
struct ControlState {
    timeline_end: u32,
    timeline_value: u32,

    speed_stride: u32,
    speed_frequence: std::time::Duration,

    play_state: PlayState,
    skip_controls: SkipControls,
}

impl ControlState {
    fn new(timeline: u32) -> ControlState {
        ControlState {
            timeline_end: timeline,
            timeline_value: 0,
            speed_stride: 1,
            speed_frequence: Duration::from_secs(1),
            play_state: PlayState::Pause,
            skip_controls: SkipControls::default(),
        }
    }

    pub fn update_controls(&mut self, message: ControlsMessage) {
        match message {
            ControlsMessage::Playstate(play_state) => {
                if self.play_state == play_state
                    || ((play_state == PlayState::Play) && self.timeline_value == self.timeline_end)
                    || ((play_state == PlayState::Backwards) && self.timeline_value == 0)
                {
                    self.timeline_value = if play_state == PlayState::Backwards {
                        self.timeline_end
                    } else {
                        0
                    };
                }
                self.play_state = play_state;
            }
            ControlsMessage::TimelineValue(value) => {
                self.timeline_value = value.min(self.timeline_end);
                self.play_state = PlayState::Pause;
            }
            ControlsMessage::SkipControls(skip_controls) => {
                self.skip_controls = skip_controls;
            }
            ControlsMessage::TimelineNext => {
                self.skip_controls.todo_skip();
                match self.play_state {
                    PlayState::Play => {
                        self.timeline_value =
                            (self.timeline_value + self.speed_stride).min(self.timeline_end);
                        if self.timeline_value == self.timeline_end {
                            self.play_state = PlayState::Pause;
                        }
                    }
                    PlayState::Backwards => {
                        self.timeline_value -= self.speed_stride;
                        if self.timeline_value == 0 {
                            self.play_state = PlayState::Pause;
                        }
                    }
                    PlayState::Pause => {
                        eprintln!("TODO: Errorhandling TimelineNext on Pause");
                    }
                }
            }
            ControlsMessage::SpeedStride(stride) => {
                if stride.is_empty() {
                    self.speed_stride = 0;
                } else {
                    let _ = stride
                        .parse::<u32>()
                        .map(|stride| self.speed_stride = stride.min(self.timeline_end));
                }
            }
            ControlsMessage::SpeedFrequence(frequence) => {
                if frequence.is_empty() {
                    self.speed_frequence = Duration::ZERO;
                } else {
                    let _ = frequence.parse::<f32>().map(|frequence| {
                        self.speed_frequence = Duration::from_secs_f32(frequence);
                    });
                }
            }
        }
    }

    fn subscription(&self) -> Subscription<ControlsMessage> {
        match self.play_state {
            PlayState::Pause => Subscription::none(),
            _ => iced::time::every(self.speed_frequence.max(Duration::from_secs_f32(0.01)))
                .map(|_| ControlsMessage::TimelineNext),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum PlayState {
    Play,
    Backwards,
    Pause,
}

#[derive(Debug, Default, Clone, Copy)]
struct SkipControls {}

impl SkipControls {
    fn todo_skip(self) {
        _ = self;
    }
}

#[derive(Debug, Clone)]
enum Message {
    FromExplorer { message: ExplorerMessage },
    FromControls { message: ControlsMessage },
    FromDetails { message: DetailsMessage },
    FromSudokuCanvas { message: SudokuCanvasMessage },
    Command { command: UICommand },
}

#[derive(Debug, Clone, Copy)]
enum ExplorerMessage {
    Expand { id: ExplorerId, value: bool },
    Selected { id: ExplorerId },
}

#[derive(Debug, Clone)]
enum ControlsMessage {
    Playstate(PlayState),
    TimelineValue(u32),
    TimelineNext,
    SpeedStride(String),
    SpeedFrequence(String),
    SkipControls(SkipControls),
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
        // TODO: determine title from state
        self.title.clone()
    }

    fn new(flags: Flags) -> (Self, iced::Task<Message>) {
        let reciever = tokio_stream::wrappers::ReceiverStream::new(flags.reciever);
        let task = iced::Task::run(reciever, |c| Message::Command { command: c });
        (
            State {
                title: "Test Window".to_string(),
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

                control: ControlState::new(9 * 9),
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
            views::control_view(&self.control).map(|m| Message::FromControls { message: m }),
        ]
        .spacing(10)
        .padding(5)
        .into()
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Command { command } => {
                println!("TODO: recieved {command:?} from stream");
            }
            Message::FromExplorer { message } => {
                self.update_explorer(message);
            }
            Message::FromControls { message } => {
                self.control.update_controls(message);
            }
            Message::FromDetails { message } => {
                self.update_details(message);
            }
            Message::FromSudokuCanvas { message } => {
                self.update_sudoku_canvas(message);
            }
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        self.control
            .subscription()
            .map(|m| Message::FromControls { message: m })
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
        .subscription(State::subscription)
        .run_with(|| State::new(flags));
    //TODO: Errorhandling
    let _ = sender.try_send(UIMessage::WindowClosed);
    println!("Gui closed");
}
