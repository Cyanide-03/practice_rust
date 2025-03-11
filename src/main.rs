use std::io;

fn main(){
	let arr=[1,2,3,4,5];
	// for i in 0..5{
	// 	println!("{}",arr[i]);
	// }
	for val in arr.iter(){
		println!("{}",val);
	}
}
