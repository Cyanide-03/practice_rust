use std::io;
use rand::Rng;

fn main(){
	println!("Guess the number!");

	let sec_no=rand::thread_rng().gen_range(1..=100);

	println!("The secret number is: {sec_no}");

    println!("Please input your guess.");

	let mut x=String::new();

	io::stdin()
		.read_line(&mut x)
		.expect("Failed to read line");

	println!("You guessed: {}", x);
}
