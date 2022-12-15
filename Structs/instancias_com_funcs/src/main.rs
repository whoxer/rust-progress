fn main() {
    
    struct User {
        username: String,
        email: String,
        active: bool,
    }
    
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
        }
    }

    let build = build_user(String::from("wox.example.email@outlook.com"),String::from("Wh0x3r"));
    
    println!("\n\t<{} profile>\n", build.username);
    println!("-----------------------------------");
    println!("email: {}", build.email);
    println!("activity: {}", build.active);

}
