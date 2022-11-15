use std::io;

fn main() {
    let a = [64, 86, 256, 16, 32];
    
    println!("<please enter an array index></>");
    
    let mut index = String::new();
    
    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");
        
    let index: usize = index
        .trim()
        .parse()
        .expect("index entered was not a number");
        
    let element = a[index];
    println!("\n");
    println!("the value of the element at index {index} is = {element}");
}
