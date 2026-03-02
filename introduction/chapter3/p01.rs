pub fn execute() {
    string_types();
    tuple_types();
    array_types();
    struct_types();
    enum_types();
}

fn string_types() {
    // String types
    let s1: String = String::from("Hello");
    let s2: &str = &s1;
    let s3: String = s2.to_string();

    println!("String types:");
    println!("s1: {}, s2: {}, s3: {}\n", s1, s2, s3);
}

fn tuple_types() {
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "2";

    println!("Tuple types:");
    println!("t: ({}, {})\n", t.0, t.1);
}

fn array_types() {
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];

    a[1] = b[1];
    a[2] = b[2];

    println!("Array types:");
    println!("a: {:?}\n", &a[1..3]);
}

fn struct_types() {
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };

    println!("Struct types:");
    println!("Name: {}, Age: {}\n", p.name, p.age);
}

fn enum_types() {
    #[derive(Debug)]
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }

    let e1 = Event::Quit;
    let e2 = Event::KeyDown(40);
    let e3 = Event::MouseDown { x: 10, y: 20 };

    println!("Enum types:");
    println!("e1: {:?}, e2: {:?}, e3: {:?}\n", e1, e2, e3);
}
