use std::fmt;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    Num(ParseIntError),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::Io(cause) => write!(f, "I/O Error: {}", cause),
            MyError::Num(cause) => write!(f, "Parse Error: {}", cause),
        }
    }
}

impl From<std::io::Error> for MyError {
    fn from(cause: std::io::Error) -> Self {
        MyError::Io(cause)
    }
}

impl From<ParseIntError> for MyError {
    fn from(cause: ParseIntError) -> Self {
        MyError::Num(cause)
    }
}

fn get_int_from_file() -> Result<i32, MyError> {
    let path = "number.txt";
    let num_str = fs::read_to_string(path)?;
    let result = num_str.trim().parse::<i32>()? * 2;

    Ok(result)
}

fn main() {
    match get_int_from_file() {
        Ok(x) => println!("The result is: {}", x),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}
