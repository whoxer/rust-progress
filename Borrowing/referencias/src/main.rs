fn main() {
    let string1 = String::from("oi, eu sou uma string e esse é o meu tamanho: ");

    let tamanho = calcula_tamanho(&string1);
    
    println!("{string1}{tamanho}");
}

fn calcula_tamanho(s: &String) -> usize {
    s.len()
}
