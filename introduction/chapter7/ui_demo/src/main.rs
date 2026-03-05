use iced::{executor, widget::Text, Application, Command, Element, Settings, Theme};

struct GUI;

impl Application for GUI {
    type Executor = executor::Default;
    type Message = ();
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (GUI, Command::none())
    }

    fn title(&self) -> String {
        String::from("DEMO")
    }

    fn update(&mut self, _message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        Text::new("Hello, world!").into()
    }
}

fn main() -> iced::Result {
    GUI::run(Settings::default())
}
