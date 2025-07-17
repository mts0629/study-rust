use std::error::Error;
use std::fs;
use std::io::Write;

// Get path strings from arguments
pub fn get_paths(args: &Vec<String>) -> Result<Vec<String>, &'static str> {
    if args.len() >= 2 {
        Ok(args[1..].to_vec())
    } else {
        Err("Not enough arguments")
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

// Print file contents
fn cat<W: Write>(mut writer: W, file_paths: &Vec<String>) -> Result<(), i32> {
    let mut num_errs = 0;

    for file_path in file_paths {
        if let Err(err) = print_file_content(&mut writer, &file_path) {
            eprintln!("{file_path}: {err}");
            num_errs += 1;
        }
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(())
    }
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
    fn get_argument() {
        let args = vec![String::from("command"), String::from("file_path_1")];

        match get_paths(&args) {
            Ok(file_paths) => {
                assert_eq!(file_paths[0], "file_path_1");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_arguments() {
        let args = vec![
            String::from("command"),
            String::from("file_path_1"),
            String::from("file_path_2"),
        ];

        match get_paths(&args) {
            Ok(file_paths) => {
                assert_eq!(file_paths[0], "file_path_1");
                assert_eq!(file_paths[1], "file_path_2");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn arguments_are_not_enough() {
        let args = vec![String::from("command")];

        match get_paths(&args) {
            Ok(_) => assert!(false),
            Err(err_msg) => {
                assert_eq!(err_msg, "Not enough arguments");
            }
        }
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
