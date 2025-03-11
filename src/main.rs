use std::io;

fn main(){
	let mut s=String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    println!("{}", s); // For string input

    // let x:i32=s.trim().parse().expect("Failed to parse"); // For integer input
    // println!("{}", x);

}
