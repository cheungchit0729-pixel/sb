use iced::widget::{container, text, scrollable};
use iced::alignment::{Horizontal, Vertical};
use iced::{Element, Length, Size, Task};
use iced_aw::{TabLabel, Tabs};

pub fn main() -> iced::Result {
    iced::application(App::default, App::update, App::view)
        .title("Personal Expense Tracker")
        .window(iced::window::Settings {
            size: Size::new(600.0, 400.0),
            ..Default::default()
        })
        .run()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TabId {
    #[default]
    Home,
    Settings,
}

#[derive(Debug, Clone)]
pub enum Message {
    TabSelected(TabId),
}

pub struct App {
    active_tab: TabId,
}

impl Default for App {
    fn default() -> Self {
        Self {
            active_tab: TabId::Home,
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::TabSelected(tab_id) => self.active_tab = tab_id,
        }
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        let tabs = Tabs::new(Message::TabSelected)
            .push(
                TabId::Home,
                TabLabel::Text("Home".to_string()),
                scrollable(column![
                    "data1",
                    "data2",
                    "data3",
                ])
            )
            .push(
                TabId::Settings,
                TabLabel::Text("Settings".to_string()),
                container(text("Settings Content").size(24))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center)
                    .align_y(Vertical::Center),
            )
            .push(
                TabId::Files,
                TabLabel::Text("File Management".to_string()).
                container(text("Settings Content").size(24))
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Horizontal::Center),
                    .align_y(Vertical::Center),
            )
            .set_active_tab(&self.active_tab);

        container(tabs)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

