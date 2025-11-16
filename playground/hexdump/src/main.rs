use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process;

fn get_file_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        None
    } else {
        Some(String::from(args[1].clone()))
    }
}

fn print_hex(file_path: &str, col_size: u32) -> Result<(), String> {
    let f = match fs::File::open(file_path) {
        Ok(f) => f,
        Err(err) => return Err(err.to_string()),
    };

    let reader = BufReader::new(f);

    let mut ofs = 0;
    let mut col = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(line) => line,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        for byte in line.bytes() {
            if col == 0 {
                print!("0x{ofs:04x} | ");
            }

            print!("{byte:02x} ");
            col += 1;

            if col == col_size {
                println!("");
                ofs += col_size;
                col = 0
            }
        }
    }

    println!(""); // Line break at the last

    Ok(())
}

fn main() {
    let file_path = match get_file_path() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Error: file is not specified");
            process::exit(1);
        }
    };

    match print_hex(&file_path, 16) {
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
        _ => {}
    };
}
