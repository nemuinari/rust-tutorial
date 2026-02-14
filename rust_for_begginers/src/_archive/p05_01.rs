// structure and instance
#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

pub fn execute() {
    let user1: User = build_user(
        "original@example.com".to_string(),
        "origin_username".to_string(),
    );
    println!("{:?}", &user1);

    let user2: User = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        ..user1
    };
    println!("{:?}", &user2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
