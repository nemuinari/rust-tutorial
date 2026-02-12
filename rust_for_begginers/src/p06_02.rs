/*
enum Option<T> {
    Some(T),
    None,
}
*/

pub fn execute() {
    let mut maybe_number: Option<i32> = Some(5);
    println!("value: {:?}", maybe_number);

    maybe_number = None;
    println!("value: {:?}", maybe_number);
}
