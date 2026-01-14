// 10 - Options and Pattern Matching

fn main() {
    let some_number = Some(10);
    match some_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number found."),
    }

    let some_string = Some("Hello, Rust!");
    match some_string {
        Some(text) => println!("The string is: {}", text),
        None => println!("No string found."),
    }

    let absent_number: Option<i32> = Option::None;
    match absent_number {
        Some(value) => println!("The number is: {}", value),
        None => println!("No number found."),
    }
}
