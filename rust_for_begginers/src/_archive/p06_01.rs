#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn show_this_message(&self) {
        println!("{:?}", self);
    }
}

pub fn execute() {
    let mut message: Message = Message::Quit;
    message.show_this_message();

    message = Message::Move { x: 10, y: 20 };
    message.show_this_message();

    message = Message::Write("content".to_string());
    message.show_this_message();

    message = Message::ChangeColor(255, 0, 0);
    message.show_this_message();
}
