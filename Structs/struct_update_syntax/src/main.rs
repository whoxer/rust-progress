fn main() {
    struct User {
        username: String,
        email: String,
        active: bool,
    }

    let user1 = User {
        email: String::from("usuario@exemplo.com"),
        username: String::from("userexample"),
        active: true,
    };

    let user2 = User {
        email: String::from("otheruser@email.com"),
        username: String::from("otherUser"),
        ..user1
    };

    println!("\t\n<listing all users online>\n");
    println!("------------------------------------------");
    println!("<@{} is {}>", user1.username, user1.active);
    println!("<email: {}", user1.email);

    println!("------------------------------------------");
    println!("<@{} is {}>", user2.username, user2.active);
    println!("<email: {}", user2.email);
    println!("------------------------------------------");
}
