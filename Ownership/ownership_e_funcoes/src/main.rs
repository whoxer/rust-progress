fn main() {
    let string = String::from("Esse texto está em posse de uma função\n");
    
    possui(string);
    
    let inteiro = 256;
    
    copia(inteiro);
}

fn possui(uma_string: String) {
	println!("{uma_string}");
}

fn copia(um_inteiro: i32) {
    println!("número inteiro copiado: {um_inteiro}");
}
