use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // Generate a random integer [1, 100]
    let secret_number = rand::thread_rng().gen_range(1..101);

    // Infinite loop
    loop {
        println!("Please input your guess.");

        // Create a new mutable variable "guess" and bind it to 
        // a new instance of a String type
        let mut guess = String::new();

        // Read a content in the standard input and append it to
        // a string argument, with using a reference
        // Display error message if an instance of Result type is an Err
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Trim space and newline from an input string
        // and parse it to an unsigned 32bit integer
        // If Err, continue loop
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // Compare the guess to the secret number
        // Branch with a return value of cmp, Ordering type
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
