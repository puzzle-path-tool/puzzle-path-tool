use iced::{
    Length, Padding, Theme,
    widget::{self, button},
};
use puzzle_core::explorer_collection::ExplorerObject;

use super::{ControllerMessage, DetailsMessage, ExplorerMessage, State, SudokuCanvasMessage};

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
        let mut col = widget::column!().padding(Padding {
            top: 2.0,
            right: 0.0,
            bottom: 8.0,
            left: 10.0,
        });
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
            col = col.push(children.iter().fold(widget::column![], |x, child| {
                x.push(explorer_view_object(child))
            }));
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

pub(super) fn control_view(_state: &State) -> iced::Element<'_, ControllerMessage> {
    widget::container(widget::text("control_view"))
        .padding(5)
        .height(250)
        .center_x(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}

pub(super) fn sudoku_view(_state: &State) -> iced::Element<'_, SudokuCanvasMessage> {
    widget::container(widget::text("sudoku_view"))
        .padding(5)
        .center(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}
