// 09 - generics

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

struct Point<T> {
    x: T,
    y: T,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T> Point<T> {
    fn xy(self) -> (T, T) {
        (self.x, self.y)
    }
}

fn main() {
    let int_sum = add(5, 10);
    let float_sum = add(5.5, 10.2);
    println!("Integer sum: {}", int_sum);
    println!("Float sum: {}", float_sum);

    let int_point = Point { x: 3, y: 4 };
    let float_point = Point { x: 1.5, y: 2.5 };
    println!("Integer Point: {:?}", int_point.xy());
    println!("Float Point: {:?}", float_point.xy());

    let success: Result<i32, &str> = Result::Ok(42);
    let failure: Result<i32, &str> = Result::Err("An error occurred");

    match success {
        Result::Ok(value) => println!("Success with value: {}", value),
        Result::Err(err) => println!("Failure with error: {}", err),
    }

    match failure {
        Result::Ok(value) => println!("Success with value: {}", value),
        Result::Err(err) => println!("Failure with error: {}", err),
    }
}
