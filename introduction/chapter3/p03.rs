// Method syntax and method chaining
pub fn execute() {
    let p = Person {
        name: String::from("Alice"),
        age: 30,
    };

    p.say_name();
    p.say_age();

    // Method chaining example
    println!("\nMethod Chaining:");
    p.new_say_name().new_say_age();
}

struct Person {
    name: String,
    age: u32,
}

impl Person {
    fn say_name(&self) {
        println!("My name is {}", self.name);
    }
    fn say_age(&self) {
        println!("I am {} years old", self.age);
    }

    // Method chaining example
    fn new_say_age(&self) -> &Self {
        println!("I am {} years old", self.age);
        self
    }
    fn new_say_name(&self) -> &Self {
        println!("My name is {}", self.name);
        self
    }
}
