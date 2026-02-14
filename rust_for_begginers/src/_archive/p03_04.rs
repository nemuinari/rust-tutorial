// Example of using if expressions and for loops in Rust
pub fn execute() {
    let confition = true;
    let number = if confition { 5 } else { 6 };

    println!("The value of number is: {}", number);

    let num_arry = [1, 2, 3, 4, 5];
    for element in num_arry {
        if element == 4 {
            break;
        } else {
            println!("The value is: {}", element);
        }
    }
}
