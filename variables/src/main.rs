fn main() {
    println!("\t<mutable variables>");
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");
    
    println!("\n");
/*
    let mut spaces = "   "; if u use it in that  way, u will received a 
                            error message when compile.
    spaces = spaces.len();
*/
    println!("\t<constant variables>");
    const TRIANGLE_PERIMETER: u32 = 30 + 40 + 10;
    const PI: u32 = 355 / 113;
    
    println!("PI value is: {PI}");
    println!("The Triangle Perimeter value with side A = 30, side B = 40,
and side C = 10 is: {TRIANGLE_PERIMETER}");
    println!("\n");
    
    println!("\t<shadowing variables>");
    let spaces = "     ";
    println!("spaces value here in string: {spaces}");
    let spaces = spaces.len();
    println!("spaces value here in number: {spaces}");
}
