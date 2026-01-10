/* Rust-Tutorial */

fn main() {
    /* for loop */
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    /* while loop
        let mut number = 3;

        while number != 0 {
            println!("{}", number);
            number -= 1;
        }
        println!("LIFTOFF!!!");
    */

    /* loop expression
        loop {
            println!("again!");
        }
        let mut counter = 0;

        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        println!("The result is {}", result);
    */

    /* if expression
    let condition = false;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {}", number);
    */

    /* if expression with else if
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4 or 2");
    }
    */
}
// 6 - control structures
