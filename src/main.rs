use std::io;

fn square(n:i32)->i32{
	return n*n;
}

fn main(){
	let mut input = String::new();
	io::stdin().read_line(&mut input).expect("Failed to read line");
	let x:i32 = input.trim().parse().expect("Invalid input");
	
	println!("Square of {} is {}",x,square(x));
}
