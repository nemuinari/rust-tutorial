// stack and heap memory example
pub fn execute() {
    {
        let s: &str = "hello";

        println!("{}", s);
    }

    {
        let mut s: String = String::from("hello");
        s.push_str(", world");

        println!("{}", s);
    }

    {
        let x: i32 = 5;
        let y: i32 = x;

        println!("x = {}, y = {}", x, y);
    }

    {
        let s: String = String::from("hello");
        let mut sc: String = s.clone();

        sc.push_str(", world");

        println!("s = {}, sc = {}", s, sc);
    }

    // references and borrowing
    let s = String::from("hello");
    let len = calculate_length(&s);

    println!("The length of s: '{}' is {}.", s, len);

    let mut sc: String = s.clone();
    concat_string(&mut sc);
    let len = calculate_length(&sc);

    println!("The length of sc: '{}' is {}.", sc, len);
}

fn calculate_length(s: &str) -> usize {
    s.len()
}

fn concat_string(s: &mut String) -> &str {
    s.push_str(", world");
    s
}
