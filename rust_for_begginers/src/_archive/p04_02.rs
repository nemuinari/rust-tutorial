// slice types
pub fn execute() {
    let mut s = String::from("Hello world!");
    let word = first_word(&s[..]);

    println!("The first word is: {}", word);
    println!("The first word is: {}", &s[..word]);
    s.clear();
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
