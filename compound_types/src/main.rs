fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, c) = tup;
    
    println!("<The Tuple Type>\n");
    println!("_inside tup variable_\n");
    println!("* var int   'a' = {a}");
    println!("* var float 'b' = {b}");
    println!("* var float 'c' = {c}");
    
    let x: (i32, f64, u8) = (500, 23.45, 0);
    
    let n0 = x.0;
    let n1 = x.1;
    let n2 = x.2;
    
    println!("\n");
    println!("_inside x variable (acessing by '.' period)_\n");
    println!("* var int   'n0' = {n0}");
    println!("* var float 'n1' = {n1}");
    println!("* var float 'n2' = {n2}");
}
