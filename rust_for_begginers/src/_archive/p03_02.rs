pub fn execute() {
    let x: usize = 6;
    let y: f64 = 1.5;
    let z: f64 = (x as f64) / y;
    println!("The value of z is: {}", z);

    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The first element of a is: {}", a[1]);

    func_example(x, y);
}

fn func_example(x: usize, y: f64) {
    let z: f64 = (x as f64) / y;
    println!("The value of z inside func_example is: {}", z);
}
