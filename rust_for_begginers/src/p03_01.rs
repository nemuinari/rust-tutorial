pub fn execute() {
    let mut x: i32 = 5;
    const CONSTANT: usize = 100;

    println!("The value of x is: {}", x);
    println!("The value of the constant is: {}", CONSTANT);

    x = 6;
    println!("The value of x is: {}", x);

    let y: i32 = 10;
    let y: i32 = y + 5;
    {
        let y: i32 = y * 2;
        println!("The value of y in the inner scope is: {}", y); // y is 30
    }
    println!("The value of y is: {}", y); // y is 15

    let some_string: &str = "aaa";
    println!("The value of some_string is: {}", some_string);

    let some_string: usize = some_string.len();
    println!("The length of some_string is: {}", some_string);
}
