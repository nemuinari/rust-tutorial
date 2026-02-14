// Example from "Rust for Beginners" tutorial, section 3.3: Functions with Parameters and Return Values
pub fn execute() {
    let x = five(4);
    let a = 'h';

    print_labeled_measurement(x, a);
}

fn five(x: i32) -> i32 {
    x + 1
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
