fn main() {
    let ope = 3;
  
	if ope == 0 {
		println!("condição caiu em {ope}\n");
		sum(4, 4);
	} else if ope == 1 {
		println!("condição caiu em {ope}\n");
		sub(8, 2);
	} else if ope == 2 {
		println!("condição caiu em {ope}\n");
		div(8, 2);
	} else if ope == 3 {
		println!("condição caiu em {ope}\n");
		mul(8, 2);
	} else if ope > 3 {
		println!("condição {ope} não existe");	
	}
}
fn sum(num1: i32, num2: i32) {
	let ope  = '+';
	let res = num1 + num2;
	println!("{num1} {ope} {num2} = {res}");
}
fn sub(num1: i32, num2: i32) {
	let ope  = '-';
	let res = num1 - num2;
	println!("{num1} {ope} {num2} = {res}");
}
fn div(num1: i32, num2: i32) {
	let ope  = '/';
	let res = num1 / num2;
	println!("{num1} {ope} {num2} = {res}");
}
fn mul(num1: i32, num2: i32) {
	let ope  = '*';
	let res = num1 * num2;
	println!("{num1} {ope} {num2} = {res}");
}
