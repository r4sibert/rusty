// First exercises from The Rust Language book
// 12/2025

use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Variable shadowing. Converts from string-type to a 32-bit unsigned number.
        // Trim whitespace with .trim(), parse the remaining number to u32,
        // and use a match case to handle the two possible Result types from parse:
        // Ok and Err. This will prevent the program from crashing with non-numeric 
        // input. 
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // match all error values with '_'
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too biiiiig!"),
            Ordering::Equal => {
                println!("OK, you win.");
                break;
            }
        }
    }
}
