use std::fs;

// Get arguments
pub fn get_args(args: &Vec<String>) -> Result<&[String], &'static str> {
    if args.len() >= 2 {
        Ok(&args[1..])
    } else {
        Err("Not enough arguments")
    }
}

// Just a wrapper of fs::read_to_string
// Handling all I/O errors is difficult, so leave it to stdlib X(
fn get_file_content(file_path: &String) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path)
}

// Print a file content to STDOUT
fn print_file_content(file_path: &String) -> Option<()> {
    match get_file_content(file_path) {
        Ok(content) => {
            print!("{content}");
            Some(())
        }
        Err(err) => {
            eprintln!("{file_path}: {err}");
            None
        }
    }
}

// Print file contents to STDOUT
pub fn run(file_paths: &[String]) -> Result<i32, i32> {
    let mut num_errs = 0;

    for file_path in file_paths {
        num_errs += match print_file_content(&file_path) {
            None => 1,
            _ => 0,
        };
    }

    if num_errs > 0 {
        Err(num_errs)
    } else {
        Ok(0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_argument() {
        let args = vec![String::from("command"), String::from("file_path_1")];

        match get_args(&args) {
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

        match get_args(&args) {
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

        match get_args(&args) {
            Ok(_) => assert!(false),
            Err(err_msg) => {
                assert_eq!(err_msg, "Not enough arguments");
            }
        }
    }

    #[test]
    fn get_text_file_content() {
        let file_path = vec![String::from("./test/test.txt")];

        match get_file_content(&file_path[0]) {
            Ok(content) => assert_eq!(content, "hello\n"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn get_binary_file_content() {
        let file_path = vec![String::from("./test/test.bin")];

        match get_file_content(&file_path[0]) {
            Ok(content) => assert_eq!(content, "hello\0"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn try_to_access_non_existing_file() {
        let file_path = vec![String::from("./test/missing.txt")];

        match get_file_content(&file_path[0]) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.kind(), std::io::ErrorKind::NotFound),
        }
    }

    #[test]
    fn try_to_access_directory() {
        let file_path = vec![String::from("./test/")];

        match get_file_content(&file_path[0]) {
            Ok(_) => assert!(false),
            Err(err) => assert_eq!(err.kind(), std::io::ErrorKind::IsADirectory),
        }
    }

    // Test just a few of error patterns...

    #[test]
    fn cat_a_file() {
        let file_path = vec![String::from("./test/test.txt")];

        match run(&file_path[0..]) {
            Ok(n_errs) => assert_eq!(n_errs, 0), // Doesn't check STDOUT
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn cat_files() {
        let file_paths = vec![
            String::from("./test/test.txt"),
            String::from("./test/test.bin"),
        ];

        match run(&file_paths[0..]) {
            Ok(n_errs) => assert_eq!(n_errs, 0),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn cat_invalid_file() {
        let file_path = vec![String::from("./test/missing.txt")];

        match run(&file_path[0..]) {
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

        match run(&file_paths[0..]) {
            Ok(_) => assert!(false),
            Err(n_errs) => assert_eq!(n_errs, 1),
        }
    }
}
