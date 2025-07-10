use std::error::Error;
use std::fs;

pub fn run(paths: &[String]) -> Result<Vec<String>, Box<dyn Error>> {
    let entries = fs::read_dir(&paths[0]).unwrap();

    let mut paths: Vec<String> = Vec::new();
    for entry in entries {
        paths.push(String::from(entry.unwrap().file_name().to_str().unwrap()));
    }

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_entries() {
        let args = vec![String::from("./src")];

        match run(&args) {
            Ok(entries) => {
                assert_eq!(entries[0], "main.rs");
                assert_eq!(entries[1], "lib.rs");
            },
            Err(_) => assert!(false)
        }
    }
}