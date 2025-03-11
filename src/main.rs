use std::io;

struct Rectangle{
    h:i32,
    w:i32,
}

fn area(rect:Rectangle)->i32{
    return rect.h*rect.w;
}

fn main(){
    let mut s = String::new();
    println!("Enter height of rectangle:");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let height: i32 = s.trim().parse().expect("Failed to parse");

    let mut s=String::new();
    println!("Enter width of rectangle");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let width:i32=s.trim().parse().expect("Failed to parse");

    let mut rect=Rectangle{h:0,w:0};
    rect.h=height;
    rect.w=width;

    println!("Area of rectangle is {}",area(rect));
}