// >, <, >=, <=, ==, !=
// &&, ||

use std::io;

fn main() {
    println!("Enter the result:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result: i32 = input.trim().parse().expect("Please enter a valid number");

    if result > 100 {
        println!("invalid result: bigger than total result");
    } else if result >= 90 {
        println!("A");
    } else if result >= 75 {
        println!("B");
    } else if result >= 60 {
        println!("C");
    } else if result >= 50 {
        println!("D");
    } else if result >= 0 {
        println!("F");
    } else {
        println!("invalid result: negative number");
    }
}
