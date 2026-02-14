#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

pub fn execute() {
    println!("--- Enums ---");

    println!("Color Red: {}", color_to_str(&Color::Red));
    println!("Color Green: {}", color_to_str(&Color::Green));
    println!("Color Blue: {}", color_to_str(&Color::Blue));

    find_maybe_number(Some(5));
    find_maybe_number(None);

    let maybe_number: Option<u32> = Some(10);

    if let Some(number) = maybe_number {
        println!("number: {}", number);
    } else {
        println!("no number found");
    }
}

fn find_maybe_number(maybe_number: Option<u32>) {
    match maybe_number {
        Some(number) => println!("Found a number: {}", number),
        None => println!("No number found"),
    }
}

fn color_to_str(color: &Color) -> &str {
    // Red #FF0000
    // Green #00FF00
    // Blue #0000FF
    match color {
        Color::Red => "#FF0000",
        Color::Green => "#00FF00",
        Color::Blue => "#0000FF",
    }
}
