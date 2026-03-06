/* ============================================================= */
/*  Desktop Stopwatch */
/* ============================================================= */
use iced::theme;
use iced::widget::{button, column, row, text};
use iced::{
    executor, time, Alignment, Application, Command, Element, Font, Length, Settings, Theme,
};
use std::time::{Duration, Instant};

struct GUI {
    tick_state: TickState,
    duration: Duration,
    last_tick: Instant,
}

#[derive(Debug, Clone)]
pub enum Message {
    Start,
    Stop,
    Reset,
    Tick(Instant),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TickState {
    Stopped,
    Ticking,
}

impl Application for GUI {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();
    type Theme = Theme;

    fn new(_flags: ()) -> (GUI, Command<Self::Message>) {
        (
            GUI {
                tick_state: TickState::Stopped,
                duration: Duration::default(),
                last_tick: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("STOP WATCH")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        match self.tick_state {
            TickState::Ticking => time::every(Duration::from_millis(10)).map(Message::Tick),
            TickState::Stopped => iced::Subscription::none(),
        }
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Start => {
                self.tick_state = TickState::Ticking;
                self.last_tick = Instant::now();
            }
            Message::Stop => {
                self.tick_state = TickState::Stopped;
            }
            Message::Reset => {
                self.duration = Duration::default();
            }
            Message::Tick(now) => {
                if self.tick_state == TickState::Ticking {
                    self.duration += now - self.last_tick;
                    self.last_tick = now;
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        let total_seconds = self.duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds / 60) % 60;
        let seconds = total_seconds % 60;
        let centiseconds = self.duration.subsec_millis() / 10;

        let duration_text = format!(
            "{:02}:{:02}:{:02}:{:02}",
            hours, minutes, seconds, centiseconds
        );

        let pixel_font = Font::with_name("PixelMplus12-Regular");

        let (label, message, btn_style) = match self.tick_state {
            TickState::Stopped => ("Start", Message::Start, theme::Button::Secondary),
            TickState::Ticking => ("Stop", Message::Stop, theme::Button::Destructive),
        };

        let start_stop_button = button(
            text(label)
                .font(pixel_font)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(message)
        .style(btn_style)
        .width(100);

        let reset_button = button(
            text("Reset")
                .font(pixel_font)
                .horizontal_alignment(iced::alignment::Horizontal::Center)
                .width(Length::Fill),
        )
        .on_press(Message::Reset)
        .style(theme::Button::Secondary)
        .width(100);

        column![
            text(duration_text).font(pixel_font).size(60),
            row![start_stop_button, reset_button].spacing(20)
        ]
        .spacing(20)
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill)
        .align_items(Alignment::Center)
        .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }
}

fn main() -> iced::Result {
    let mut settings = Settings::default();
    settings.window.size = (400, 180);

    settings.default_font = Font::with_name("PixelMplus12-Regular");

    GUI::run(settings)
}

/*
   =============================================================
   COLOR CHART (Theme::Dark / Iced 0.10)
   =============================================================

   [ Background / Surface ]
   - Window Background : #202225 (濃いグレー/黒)
   - Surface/Card      : #2F3136 (やや明るいグレー)

   [ Button Styles (.style) ]
   - theme::Button::Positive    : [  Start  ] -> 背景: #43B581 (緑) / 文字: White
   - theme::Button::Destructive : [  Stop   ] -> 背景: #F04747 (赤) / 文字: White
   - theme::Button::Primary     : [  Reset  ] -> 背景: #5865F2 (青) / 文字: White
   - theme::Button::Secondary   : [  Other  ] -> 背景: #4F545C (灰) / 文字: White
   - theme::Button::Text        : [  Link   ] -> 背景: 透明      / 文字: #00AFF4 (水色)

   [ Text Colors ]
   - Default Text : #FFFFFF (白)
   - Soft Text    : #B9BBBE (薄いグレー)
   =============================================================
*/
