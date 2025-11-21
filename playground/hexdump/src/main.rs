use std::env;
use std::fs;
use std::io::{BufReader, Read};
use std::process;

fn get_flag(args: &Vec<String>, opt: &str) -> bool {
    for arg in args {
        if arg == opt {
            return true;
        }
    }

    false
}

fn get_file_path(args: &Vec<String>) -> Option<String> {
    if args.len() < 2 {
        None
    } else {
        Some(String::from(args[1].clone()))
    }
}

fn print_chars(bytes: &Vec<u8>) {
    print!("| ");

    for b in bytes {
        let c = *b as char;
        if c.is_control() {
            print!(".");
        } else {
            print!("{c}");
        }
    }
}

#[derive(PartialEq)]
enum ByteFormat {
    Octal,
    Hexadecimal,
    Char,
}

fn print_hex(
    file_path: &str,
    col_size: usize,
    fmt: ByteFormat,
    print_char: bool,
) -> Result<(), String> {
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

        if fmt == ByteFormat::Octal {
            print!("{b:03o} ");
        } else if fmt == ByteFormat::Hexadecimal {
            print!("{b:02x} ");
        } else {
            // ByteFormat::Char
            let c = b as char;
            if c.is_control() {
                print!(". ");
            } else {
                print!("{c} ");
            }
        }

        col += 1;

        bytes.push(b);

        if col == col_size {
            if print_char {
                print_chars(&bytes);
            }
            println!("");
            bytes.clear();

            ofs += col_size;
            col = 0
        }
    }

    // Fill columns
    while col < col_size {
        print!("   ");
        col += 1;
    }

    if print_char {
        print_chars(&bytes);
    }

    println!(""); // Line break at the last

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let print_char = get_flag(&args, "-C");

    let fmt = if get_flag(&args, "-b") {
        ByteFormat::Octal
    } else if get_flag(&args, "-c") {
        ByteFormat::Char
    } else {
        ByteFormat::Hexadecimal
    };

    let file_path = match get_file_path(&args) {
        Some(file_path) => file_path,
        None => {
            eprintln!("Error: file is not specified");
            process::exit(1);
        }
    };

    match print_hex(&file_path, 16, fmt, print_char) {
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
        _ => {}
    };
}
