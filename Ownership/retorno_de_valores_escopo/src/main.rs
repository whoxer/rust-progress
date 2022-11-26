fn main() {
    let v1 = entrega_v();
    println!("{v1}");
    
    println!("e\n");    
    let v2 = String::from("Olá universo\n");
    let v3 = pega_e_entrega(v2);
    println!("{v3}");
    
    let v4 = String::from("Esse é um texto");
    let (v5, tamanho) = calcula_t(v4);
    println!("{v5}, e o tamanho dele é {tamanho}");
}

fn entrega_v() -> String {
    let uma_string = String::from("Olá mundo\n");
    uma_string
}

fn pega_e_entrega(uma_string: String) -> String {
    uma_string
} 

fn calcula_t(texto: String) -> (String, usize) {
    let tamanho = texto.len();
    
    (texto, tamanho)
}
