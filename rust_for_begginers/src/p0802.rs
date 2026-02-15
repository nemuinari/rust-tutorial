pub fn execute() {
    let s1 = "foo";
    let s2 = "bar";

    let mut s = String::from(s1);
    s.push_str(s2);

    println!("{}", s);
    println!("{}", s2);
}
