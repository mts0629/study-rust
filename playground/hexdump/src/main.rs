use std::env;
use std::fs;
use std::io::{BufReader, Read};
use std::process;

fn get_file_path() -> Option<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        None
    } else {
        Some(String::from(args[1].clone()))
    }
}

fn print_hex(file_path: &str, col_size: usize) -> Result<(), String> {
    let f = match fs::File::open(file_path) {
        Ok(f) => f,
        Err(err) => return Err(err.to_string()),
    };

    let reader = BufReader::new(f);

    let mut bytes: Vec<u8> = Vec::with_capacity(col_size);

    let mut ofs = 0;
    let mut col = 0;
    for byte in reader.bytes() {
        let b = match byte {
            Ok(byte) => byte,
            Err(err) => {
                return Err(err.to_string());
            }
        };

        if col == 0 {
            print!("0x{ofs:04x} | ");
        }

        print!("{b:02x} ");
        col += 1;

        bytes.push(b);

        if col == col_size {
            print!("| ");

            // Print characters
            for b in &bytes {
                let c = *b as char;
                if c.is_control() {
                    print!(".");
                } else {
                    print!("{c}");
                }
            }
            bytes.clear();
            println!("");

            ofs += col_size;
            col = 0
        }
    }

    // Fill columns
    while col < col_size {
        print!("   ");
        col += 1;
    }

    print!("| ");
    for b in &bytes {
        let c = *b as char;
        if c.is_control() {
            print!(".");
        } else {
            print!("{c}");
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
