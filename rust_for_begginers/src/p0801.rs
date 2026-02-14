pub fn execute() {
    let mut v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    v.push(6);
    let sixth: &i32 = &mut v[5];
    println!("The sixth element is {}", sixth + 1);

    for i in &v {
        println!("{}", i);
    }
}
