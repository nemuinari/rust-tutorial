fn main() {
    let s = "hello"; 

    let slice = &s[0..2];
    println!("Slice: {}", slice);

    let slice = &s[0..=2];
    println!("Slice: {}", slice);

    let slice = &s[..2];
    println!("Slice: {}", slice);

    let slice = &s[2..];
    println!("Slice: {}", slice);

    let slice = &s; 
    println!("Slice: {}", slice);
}
// #04 ~データ型
