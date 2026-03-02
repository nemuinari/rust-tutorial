fn get_int_from_file() -> i32 {
    let path = "number.txt";

    let num_str = std::fs::read_to_string(path).expect("Failed to read the file");
    let ret = num_str
        .trim()
        .parse::<i32>()
        .expect("Failed to parse the number");

    ret * 2
}

fn main() {
    println!("The result is: {}", get_int_from_file());
}
