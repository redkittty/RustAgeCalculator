use std::io;
fn main() {
    println!("Enter Your Birth Year");
    let mut year = String::new();
    io::stdin()
        .read_line(&mut year)
        .expect("Failed to read line");
    let year: i32 = year.trim().parse().expect("Please insert a Number");
    let current = 2024;
    let age = current - year; 
    println!("Age = {age}");
 }
