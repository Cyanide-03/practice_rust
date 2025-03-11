use std::io;

fn count_vowels(s:&str)->i32{
    let mut count=0;
    for c in s.chars(){
        // if c=='a' || c=='e' || c=='i' || c=='o' || c=='u'{
        //     count+=1;
        // }
        if matches!(c,'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U'){
            count+=1;
        }
    }
    return count;
}

fn main(){
    let mut s=String::new();
    println!("Enter a string:");
    io::stdin().read_line(&mut s).expect("Failed to read line");
    let trimmed=s.trim();
    println!("The number of vowels in {} are {}",trimmed,count_vowels(trimmed));
}