fn main() {
    let mut s = String::from("isso é um texto!");
    
    let palavra = primeira_palavra(&s);
    
    s.clear();
    
    println!("{palavra}");
}

fn primeira_palavra(s: &String) -> usize {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        } 
    }
    
    s.len()
}

