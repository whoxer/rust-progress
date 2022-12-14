fn main() {
    let mut s = String::from("morte a burguesia");
    
    let string = primeira_palavra(&s);
    
    
    println!("{string}");
    s.clear();
}
fn primeira_palavra(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[0..]
}
