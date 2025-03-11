use std::io;

// For passing array as reference
fn change_array(arr: &mut [i32;5]){
	arr[0]=100;
	println!("Array inside change_array function: {:?}", arr);
}

// For passing array as value
// fn change_array(mut arr:[i32;5]){
// 	arr[0]=100;
// 	println!("Array inside change_array function: {:?}", arr);
// }

fn main(){
	// let arr=[1,2,3,4,5]; // For passing array as value
	let mut arr=[1,2,3,4,5]; // For passing array as reference
	println!("Array before change_array function: {:?}", arr);

	// change_array(arr); // For passing array as value
	change_array(&mut arr); // For passing array as reference


	println!("Array after change_array function: {:?}", arr);
}
