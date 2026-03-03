// ABC447 A-Seats2
// https://kenkoooo.com/atcoder/#/table/
use proconio::input;

fn main() {
    input! {
        n: i32,
        m: i32,
    }
    if n >= (m * 2) - 1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
