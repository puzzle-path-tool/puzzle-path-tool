use iced::{
    Length, Padding, Theme,
    widget::{self, button},
};
use puzzle_core::explorer_collection::ExplorerObject;

use super::{ControlsMessage, DetailsMessage, ExplorerMessage, State, SudokuCanvasMessage};

pub(super) mod sudoku_canvas;

pub(super) fn explorer_view(
    explorer: &ExplorerObject,
) -> Option<iced::Element<'_, ExplorerMessage>> {
    explorer.expanded().map(|(children, expanded)| {
        widget::container(
            widget::column![text_button(
                format!("||| {}", explorer.name()),
                ExplorerMessage::Expand {
                    id: explorer.id(),
                    value: !expanded,
                }
            ),]
            .push_maybe(if expanded {
                Some({
                    widget::scrollable(
                        children
                            .iter()
                            .fold(widget::column![], |col, child| {
                                col.push(explorer_view_object(child))
                            })
                            .padding(5),
                    )
                })
            } else {
                None
            }),
        )
        .align_top(if expanded {
            Length::Fill
        } else {
            Length::Shrink
        })
        .max_width(250)
        .clip(true)
        .style(widget::container::bordered_box)
        .into()
    })
}

fn explorer_view_object(object: &ExplorerObject) -> iced::Element<'_, ExplorerMessage> {
    if let Some((children, expanded)) = object.expanded() {
        let mut col = widget::column!();
        let file_string = if expanded {
            format!("v {}", object.name())
        } else {
            format!("> {}", object.name())
        };
        col = col.push(text_button(
            file_string,
            ExplorerMessage::Expand {
                id: object.id(),
                value: !expanded,
            },
        ));
        if expanded {
            col = col.push(children.iter().fold(
                widget::column![].padding(Padding {
                    top: 2.0,
                    right: 0.0,
                    bottom: 8.0,
                    left: 10.0,
                }),
                |x, child| x.push(explorer_view_object(child)),
            ));
        }
        col.into()
    } else {
        text_button(
            object.name().as_str(),
            ExplorerMessage::Selected { id: object.id() },
        )
        .into()
    }
}

fn text_button<'a, T>(
    text: impl widget::text::IntoFragment<'a>,
    message: T,
) -> widget::button::Button<'a, T> {
    widget::button(widget::text(text))
        .on_press(message)
        .style(|theme: &Theme, _status| button::Style {
            background: None,
            text_color: theme.palette().text,
            border: iced::border::Border::default(),
            shadow: iced::Shadow::default(),
        })
}

pub(super) fn path_info_view(_state: &State) -> iced::Element<'_, DetailsMessage> {
    widget::container(widget::text("path_info_view"))
        .padding(5)
        .width(250)
        .center_y(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}

pub(super) fn control_view(state: &super::ControlState) -> iced::Element<'_, ControlsMessage> {
    widget::container(
        widget::column![
            widget::container(widget::slider(
                0..=state.timeline_end,
                state.timeline_value,
                ControlsMessage::TimelineValue
            ))
            .center_x(Length::Fill),
            widget::row![
                widget::container(widget::column![
                    widget::row![
                        widget::button(widget::text(
                            if state.play_state == super::PlayState::Play
                                || state.timeline_value == state.timeline_end
                            {
                                "RePlay"
                            } else {
                                "Play"
                            }
                        ))
                        .on_press(ControlsMessage::Playstate(super::PlayState::Play)),
                        widget::button(widget::text(
                            if state.play_state == super::PlayState::Pause {
                                "Cancel"
                            } else {
                                "Pause"
                            }
                        ))
                        .on_press(ControlsMessage::Playstate(super::PlayState::Pause)),
                        widget::button(widget::text(
                            if state.play_state == super::PlayState::Backwards
                                || state.timeline_value == 0
                            {
                                "RePlay Backwards"
                            } else {
                                "Backwards"
                            }
                        ))
                        .on_press(ControlsMessage::Playstate(super::PlayState::Backwards)),
                    ],
                    widget::row![
                        widget::text("Step frequenz:"),
                        widget::text_input::TextInput::new(
                            "-",
                            {
                                let value = format!("{}", state.speed_frequence.as_secs_f32());
                                if value.contains('.') {
                                    value
                                } else {
                                    format!("{value}.")
                                }
                            }
                            .as_str()
                        )
                        .width(100)
                        .on_input(|mut value| if value.contains('.') {
                            ControlsMessage::SpeedFrequence(value)
                        } else {
                            value.pop();
                            ControlsMessage::SpeedFrequence(value)
                        }),
                        widget::text("Step stride:"),
                        widget::text_input::TextInput::new(
                            "-",
                            format!("{}", state.speed_stride).as_str()
                        )
                        .width(100)
                        .on_input(ControlsMessage::SpeedStride),
                    ]
                    .spacing(3)
                    .width(350)
                ]),
                widget::container(widget::text("skip_controlls")),
            ]
            .spacing(5)
        ]
        .spacing(5),
    )
    .padding(5)
    .center_x(Length::Fill)
    .style(widget::container::bordered_box)
    .into()
}

pub(super) fn sudoku_view(state: &State) -> iced::Element<'_, SudokuCanvasMessage> {
    widget::container(sudoku_canvas::view(state))
        .center(Length::Fill)
        .into()
}
