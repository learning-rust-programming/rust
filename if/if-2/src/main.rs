// &&, ||

use std::io;

fn main() {
    println!("Enter the result:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result: i32 = input.trim().parse().expect("Please enter a valid number");

    if result < 0 || result > 100 {
        println!("invalid result");
    } else {
        println!("valid result");
    }
}
