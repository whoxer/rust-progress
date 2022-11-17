fn main() {
    my_name_is("Wh0xer");
    my_age(17);
    calc(2, 2);
    
    let year = year();
    println!("we are in the year of {year}");
    
    let less_one = less_one(4);
    println!("the result of 4 - 2 = {less_one}");
}
fn my_age(x: i32) {
    println!("and I have {x} years old when I wrote this code");
}
fn my_name_is(name: &str) {
    println!("my name is {name}");
}
fn calc(a: i32, b: i32) {
    let res = {
         let multiply = 2;
         let res_to_m = a + b;
         multiply * res_to_m
    };
    
    println!("\n");
    println!("the result of {a} + {b} * 2 = {res}");
}
fn year() -> i32 {
    2022
}
fn less_one(a: i32 ) -> i32 {
    a - 2
}
