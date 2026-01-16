// break
// continue

use std::io;
use std::io::Write;

fn main() {
    let mut counter = 1;

    loop {
        if counter == 3 {
            counter += 1;
            continue;
        }

        print!("Enter {counter}: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let number: i32 = input.trim().parse().expect("Please enter a valid number");

        if number % 2 == 0 {
            println!("The number is even\n");
        } else {
            println!("The number is odd\n");
        }

        counter += 1; // counter = counter + 1;

        if counter > 4 {
            break;
        }
    }
}
