use iced::{Length, widget};

use super::{Message, State};

pub(super) fn explorer_view(_state: &State) -> iced::Element<'_, Message> {
    widget::container(widget::text("explorer_view"))
        .padding(5)
        .width(250)
        .center_y(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}

pub(super) fn path_info_view(_state: &State) -> iced::Element<'_, Message> {
    widget::container(widget::text("path_info_view"))
        .padding(5)
        .width(250)
        .center_y(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}

pub(super) fn control_view(_state: &State) -> iced::Element<'_, Message> {
    widget::container(widget::text("control_view"))
        .padding(5)
        .height(250)
        .center_x(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}

pub(super) fn sudoku_view(_state: &State) -> iced::Element<'_, Message> {
    widget::container(widget::text("sudoku_view"))
        .padding(5)
        .center(Length::Fill)
        .style(widget::container::bordered_box)
        .into()
}
