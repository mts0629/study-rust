use std::fs;
use std::process;

pub fn run(file_paths: &[String]) {
    for file_path in file_paths {
        match fs::read_to_string(file_path) {
            Ok(content) => print!("{content}"),
            Err(e) => {
                eprintln!("Error: \"{file_path}\": {e}");
                process::exit(1);
            }
        }
    }
}
