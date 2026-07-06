use iced::{
    Alignment, Element, Length, widget::{
        container,
        text,
    },
};

#[derive(Debug, Clone)]
pub enum StartPaneMessage {

}

#[derive(Debug, Clone)]
pub struct StartPane {

}

impl StartPane {
    pub fn new() -> Self {
        Self {}
    }

    pub fn view(&self) -> Element<'_, StartPaneMessage> {
        container(
            text("Select a feed to get started")
        )
            .align_y(Alignment::Center)
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn update(&self, msg: StartPaneMessage) {

    }
}
