use std::io;

fn main() {
    loop {
        println!("Enter a number:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: i32 = input.trim().parse().expect("Please enter a valid number");

        if number % 2 == 0 {
            println!("The number is even");
        } else {
            println!("The number is odd");
        }
    }
}
