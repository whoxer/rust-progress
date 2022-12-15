fn main() {
    struct User {
        nomeusuario: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }
    
    
    let mut usuario01 = User {
        nomeusuario: String::from("Wh0xer"),
        email: String::from("woxerexampleemail@outlook"),
        sign_in_count: 1,
        active: true,
    };
    
    //mudando nome de usuario
    usuario01.nomeusuario = String::from("Wh0x3rrr.dat");
    
    println!("usuario: {}", usuario01.nomeusuario);
    println!("email: {}", usuario01.email);
    println!("logou: {} vez", usuario01.sign_in_count);
    println!("atividade: {}", usuario01.active);
}


