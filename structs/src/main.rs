struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_struct(user: &User) {
    println!("name:{}\n email: {}", user.username, user.email);
}

fn main() {
    let mut user1 = build_user(String::from("Aryan"), String::from("asd@gmail.com"));
    print_struct(&user1);
    let user2 = User {
        active: false,
        sign_in_count: 2,
        ..user1
    };
    user1.email = String::from("asdiauhsidhaishduiahs");
    print_struct(&user2);
    println!(
        "user1 username: {}, user2 username: {}",
        user1.username, user2.username
    );
}
