// tuple structs and derived traits
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(width: u32) -> Self {
        Self {
            width,
            height: width,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

pub fn execute() {
    let mut rect: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    rect.set_width(60);

    let square: Rectangle = Rectangle::square(40);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!("The area of the square is {} square pixels.", square.area());

    println!("Rectangle details: {:?}", &rect);
    println!("Square details: {:?}", &square);
}
