use std::error::Error;
use std::fs;
use std::io::{Read, Write};

// Get a list of input from arguments
pub fn get_inputs(args: &Vec<String>) -> Vec<String> {
    if args.len() >= 2 {
        args[1..].to_vec()
    } else {
        vec![]
    }
}

// Print a file content
fn print_file_content<W: Write>(mut writer: W, file_path: &String) -> Result<(), Box<dyn Error>> {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            write!(writer, "{content}")?;
        }
        Err(err) => return Err(Box::new(err)),
    }

    Ok(())
}

// Print STDIN or file contents
fn cat<W: Write>(mut writer: W, inputs: &Vec<String>) -> Result<(), i32> {
    if inputs.len() == 0 {
        // Print STDIN
        let mut buffer = String::new();
        if let Err(err) = std::io::stdin().read_to_string(&mut buffer) {
            eprintln!("{err}");
            return Err(1);
        }

        if let Err(err) = write!(writer, "{buffer}") {
            eprintln!("{err}");
            return Err(1);
        }
    } else {
        // Print files
        let mut num_errs = 0;

        for input in inputs {
            if let Err(err) = print_file_content(&mut writer, &input) {
                eprintln!("{input}: {err}");
                num_errs += 1;
            }
        }
        if num_errs > 0 {
            return Err(num_errs);
        }
    }

    Ok(())
}

pub fn run(file_paths: Vec<String>) -> Result<(), ()> {
    match cat(std::io::stdout(), &file_paths) {
        Ok(_) => Ok(()),
        Err(_) => Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_input() {
        let args = vec![String::from("command"), String::from("file_path_1")];

        let file_paths = get_inputs(&args);
        assert_eq!(file_paths[0], "file_path_1");
    }

    #[test]
    fn get_multiple_inputs() {
        let args = vec![
            String::from("command"),
            String::from("file_path_1"),
            String::from("file_path_2"),
        ];

        let file_paths = get_inputs(&args);
        assert_eq!(file_paths[0], "file_path_1");
        assert_eq!(file_paths[1], "file_path_2");
    }

    #[test]
    fn no_inputs() {
        let args = vec![String::from("command")];

        let file_paths = get_inputs(&args);
        assert_eq!(file_paths.len(), 0);
    }

    #[test]
    fn cat_a_file() {
        let file_path = vec![String::from("./test/test.txt")];
        let mut buf = Vec::new();

        match cat(&mut buf, &file_path) {
            Ok(_) => {
                assert_eq!(String::from_utf8(buf).unwrap(), "hello\n");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn cat_a_binary_file() {
        let file_path = vec![String::from("./test/test.bin")];
        let mut buf = Vec::new();

        match cat(&mut buf, &file_path) {
            Ok(_) => {
                assert_eq!(String::from_utf8(buf).unwrap(), "hello\0");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn cat_files() {
        let file_paths = vec![
            String::from("./test/test.txt"),
            String::from("./test/test.bin"),
        ];
        let mut buf = Vec::new();

        match cat(&mut buf, &file_paths) {
            Ok(_) => {
                assert_eq!(String::from_utf8(buf).unwrap(), "hello\nhello\0");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn cat_invalid_file() {
        let file_path = vec![String::from("./test/missing.txt")];
        let mut buf = Vec::new();

        match cat(&mut buf, &file_path) {
            Ok(_) => assert!(false),
            Err(n_errs) => assert_eq!(n_errs, 1),
        }
    }

    #[test]
    fn cat_with_invalid_files() {
        let file_paths = vec![
            String::from("./test/test.txt"),
            String::from("./test/missing.txt"),
            String::from("./test/test.bin"),
        ];
        let mut buf = Vec::new();

        match cat(&mut buf, &file_paths) {
            Ok(_) => assert!(false),
            Err(n_errs) => {
                assert_eq!(n_errs, 1);
                // Output all printable contents
                assert_eq!(String::from_utf8(buf).unwrap(), "hello\nhello\0");
            }
        }
    }
}
