use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.len() {
        2 => String::from(args[1].clone()),
        _ => {
            eprintln!("Error: file is not specified");
            process::exit(1);
        }
    };

    match fs::read_to_string(file_path) {
        Ok(content) => {
            print!("{content}");
        }
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    }
}
