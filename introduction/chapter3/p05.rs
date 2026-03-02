// move semantic and borrowing
struct Color {
    r: i32,
    g: i32,
    b: i32,
}

pub fn execute() {
    let a = Color { r: 255, g: 0, b: 0 };
    let b = a; // move
    println!("b: {}, {}, {}", b.r, b.g, b.b);

    calc_data(&b); // borrow
}

fn calc_data(data: &Color) {
    println!("calc_data: {}, {}, {}", data.r, data.g, data.b);
}
