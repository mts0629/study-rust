use std::env;
use std::fs;
use std::process;

fn print_hex(content: Vec<u8>, col_size: u32) {
    let mut byte_index = 0;
    let mut col = 0;
    for byte in content {
        if col == 0 {
            print!("0x{byte_index:04x} | ");
        }

        print!("{byte:02x} ");
        col += 1;
        byte_index += 1;

        if col == col_size {
            println!("");
            col = 0
        }
    }

    println!("");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = match args.len() {
        2 => String::from(args[1].clone()),
        _ => {
            eprintln!("Error: file is not specified");
            process::exit(1);
        }
    };

    let content = match fs::read(file_path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("Error: {err}");
            process::exit(1);
        }
    };

    print_hex(content, 16);
}
