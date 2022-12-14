fn main() {
    let mut frase = String::from("eu... ");
    
    println!("{frase}");

    modifica(&mut frase);

    println!("{frase}");
}
fn modifica(resto_frase: &mut String) {
    resto_frase.push_str("eu amo... batata frita...");
}
