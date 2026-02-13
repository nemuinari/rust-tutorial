pub mod second;
pub mod third;

use second::hello;
use third::third::execute;

fn main() {
    println!("Hello, main!");
    hello();
    execute();
}
