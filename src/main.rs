use std::io;

fn fibonacci(n:i32)->i32{
    if n<=2 {
        return 1;
    }
    else{
        return fibonacci(n-1)+fibonacci(n-2);
    }
}

fn main(){
    let mut s=String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");

    let n:i32=s.trim().parse().expect("Failed to parse");

    println!("The {}th fibonacci number is {}",n,fibonacci(n));
}