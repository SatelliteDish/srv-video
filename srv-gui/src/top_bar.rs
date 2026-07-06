use iced::{ Alignment::Center,
    Element,
    Length::Fill,
    widget::{
        button,
        row,
        text,
        container,
    },
};
use crate::icon;

#[derive(Debug, Clone)]
pub enum TopBarAction {
    Close,
}

#[derive(Debug)]
pub struct TopBar {
    title: String,
}

impl TopBar {
    pub fn new() -> Self {
        Self {
            title: "SrvGui".to_string(),
        }
    }

    pub fn view(&self) -> Element<'_, TopBarAction> {
        row![
            container(text(&self.title))
                .width(Fill)
                .align_x(Center),
            button(icon::x()).on_press(TopBarAction::Close),
        ].width(Fill).into()
    }
}
