use std::io;

fn main() {
    println!("Enter the result:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let result: i32 = input.trim().parse().expect("Please enter a valid number");

    match result {
        80..100 => println!("A"),
        70..79 => println!("B"),
        60..69 => println!("C"),
        50..59 => println!("D"),
        0..49 => println!("F"),
        _ => println!("invalid result"),
    }

    println!("Enter the name of student:");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let name: String = input.trim().parse().expect("Please enter a valid name");

    match name.to_lowercase().as_str() {
        "ahmed" => println!("First Class Student"),
        "ali" | "hassan" => println!("Second Class Student"),
        _ => println!("Unknown Student"),
    }
}
