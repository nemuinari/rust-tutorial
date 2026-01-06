fn main() {
    println!("Hello, Rust!");

    let mut a: i32 = 10;
    let a_mut_ref: &mut i32 = &mut a;

    *a_mut_ref = 20;
    println!("{}", a_mut_ref);
}
// #01 ~基本
