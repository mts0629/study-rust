use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::net::IpAddr;

// Creating custom types for validation
pub struct Guess {
    value: i32, // Private field
}

impl Guess {
    // A kind of "constructor"
    pub fn new(value: i32) -> Guess {
        // A `Guess` type that will only continue with values between 1 and 100
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    // "Getter" of the value
    pub fn value(&self) -> i32 {
        self.value
    }
}

fn guessing_game() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Input a number (1 to 100):");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: Guess = match guess.trim().parse() {
            // panic if `num` is not in range of [1, 100]
            // by the restriction of the `Guess` type
            Ok(num) => Guess::new(num),
            Err(_) => continue,
        };

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    // `unwrap` and `expect` are appropriate when
    // the program has some other logic that ensures the Result have an `Ok`
    // but the logic isn't something that compiler understands

    // "127.0.0.1" is a hard-coded, valid IP address
    let home: IpAddr = "127.0.0.1"
        .parse() // `Err` is possible even if the string is valid
        .expect("Hardcoded IP address should be valid");

    // Guide for error handling:
    // - The bad state is something that is unexpected, as opposed to something
    //   that will likely happen occasionaly (e.g. user input in the wrong format)
    // - Your code after this point needs to rely on not being in this bad state,
    //   rather than checking for the problem at every step
    // - There's not a good way to encode this information in the types you use

    guessing_game();
}
