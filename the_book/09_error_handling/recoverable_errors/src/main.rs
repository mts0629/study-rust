use std::fs::File;
use std::io::{self, Read, ErrorKind};

fn panic_on_err() {
    // Open a file: "hello.txt"
    let greeting_file_result = File::open("hello.txt");

    // Panic when the opening failed
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

fn matching_on_different_errors() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            // When the "hello.txt" is not found (ErrorKind::NotFound),
            // create it newly
            ErrorKind::NotFound => match File::create("hello.txt") {
                // Panic if the file creation failed
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };

    // The same behavior with the above code by using upwrap_or_else
    // let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("Problem creating the file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {:?}", error);
    //     }
    // });
}

fn unwrap_and_expect() {
    // `unwrap`: call panic! when the Result is Err
    // let greeting_file = File::open("hello.txt").unwrap();

    // `expect`: call panic! when the Result is Err
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}

// Show a username from a file
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    // If the file doesn't exist, return the error
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    // If the file can't be read, return the error
    match username_file.read_to_string(&mut username) {
        // Omit `return`, because this is the last expression in the function
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// The same behavior with the above implementation by `?` operator
// ? operator calls `from` function and convert received error type into the error type
// defined in the return type of the current function
// fn read_username_from_file() {
//     let mut username_file = File::open("hello.txt")?;
//     let mut username = String::new();
//     username_file.read_to_string(&mut username)?;
//     Ok(username)
// }

// Shortened code by chaining method calls
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut username = String::new();
// 
//     File::open("hello.txt")?.read_to_string(&mut username)?;
// 
//     Ok(username)
// }

// More shorter code
// fn read_username_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

fn propagating_errors() {
    // Propagating the error from a user function: `read_username_from_file`
    let username = match read_username_from_file() {
        Ok(name) => println!("{}", name),
        Err(e) => panic!("{:?}", e),
    };
}

// `?` operator can be used in a function which returns:
// - Result
// - Option
// - another type that implements FromResidual

// Return `None` if `text` is empty
fn last_char_or_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

fn main() {
    // panic_on_err();

    // matching_on_different_errors();

    // unwrap_and_expect();

    propagating_errors();
}

// Change the return type of `main` to use the `?` operator
// main returns any types which implement a trait: `std::process::Termination`
fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
