use std::error::Error;
use std::fs;
use std::io::Write;

// Get path strings from arguments
pub fn get_paths(args: &[String]) -> Vec<String> {
    if args.len() == 1 {
        vec![String::from(".")]
    } else {
        args[1..].to_vec()
    }
}

// Get a list of entries in the specified path
fn get_dir_entries(path: &String) -> Result<Vec<String>, Box<dyn Error>> {
    let entries = match fs::read_dir(path) {
        Ok(result) => result,
        Err(err) => {
            return Err(Box::new(err));
        }
    };

    let mut dir_entries: Vec<String> = Vec::new();
    for entry in entries {
        dir_entries.push(String::from(entry?.file_name().to_string_lossy()));
    }

    dir_entries.sort();

    Ok(dir_entries)
}

// Print entries
fn print_entries<W: Write>(
    mut writer: W,
    path: &String,
    entries: Vec<String>,
    print_path: bool,
) -> Result<(), Box<dyn Error>> {
    if print_path {
        writeln!(writer, "{}:", path)?;
    }
    writeln!(writer, "{}", entries.join("  "))?;

    Ok(())
}

// Main process
fn ls<W: Write>(mut writer: W, paths: &Vec<String>) -> Result<(), Box<dyn Error>> {
    let print_path = paths.len() > 1;

    let mut iter = paths.iter().peekable();

    while let Some(path) = iter.next() {
        let dir_entries = get_dir_entries(&path)?;

        print_entries(&mut writer, &path, dir_entries, print_path)?;

        if !iter.peek().is_none() {
            writeln!(writer, "")?;
        }
    }

    Ok(())
}

pub fn run(paths: Vec<String>) -> Result<(), Box<dyn Error>> {
    ls(std::io::stdout(), &paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_entries() {
        match get_dir_entries(&String::from("./src")) {
            Ok(entries) => {
                assert_eq!(entries[0], "lib.rs");
                assert_eq!(entries[1], "main.rs");
            }
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn error_from_read_dir() {
        match get_dir_entries(&String::from("./not_exist")) {
            Ok(_) => assert!(false),
            Err(err) => {
                assert_eq!(err.to_string(), "No such file or directory (os error 2)");
            }
        }
    }

    #[test]
    fn formatted_print() {
        let list = vec![String::from("a.txt"), String::from("b.txt")];
        let mut buf = Vec::new();

        match print_entries(&mut buf, &String::from("."), list, false) {
            Ok(_) => assert_eq!(String::from_utf8(buf).unwrap(), "a.txt  b.txt\n"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn formatted_print_with_path() {
        let list = vec![String::from("a.txt"), String::from("b.txt")];
        let mut buf = Vec::new();

        match print_entries(&mut buf, &String::from("."), list, true) {
            Ok(_) => assert_eq!(String::from_utf8(buf).unwrap(), ".:\na.txt  b.txt\n"),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn print_paths() {
        let paths = vec![String::from("."), String::from("src")];
        let mut buf = Vec::new();

        match ls(&mut buf, &paths) {
            Ok(_) => assert_eq!(
                String::from_utf8(buf).unwrap(),
                ".:\nCargo.lock  Cargo.toml  src  target\n\nsrc:\nlib.rs  main.rs\n"
            ),
            Err(_) => assert!(false),
        }
    }
}
