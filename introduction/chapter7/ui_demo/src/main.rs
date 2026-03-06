use iced::widget::{button, column, row, text};
use iced::{executor, Alignment, Application, Command, Element, Font, Length, Settings, Theme};

struct GUI;

#[derive(Debug, Clone, Copy)]
enum Message {
    StartStop,
    Reset,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (GUI {}, Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let tick_text = text("00:00:00").size(60);

        let start_stop_button =
            button(text("Start").horizontal_alignment(iced::alignment::Horizontal::Center))
                .on_press(Message::StartStop)
                .width(80);

        let reset_button =
            button(text("Reset").horizontal_alignment(iced::alignment::Horizontal::Center))
                .on_press(Message::Reset)
                .width(80);

        column![tick_text, row![start_stop_button, reset_button].spacing(10)]
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(Alignment::Center)
            .into()
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();

    settings.window.size = (400, 150);
    settings.default_font = Font::default();

    GUI::run(settings)
}
