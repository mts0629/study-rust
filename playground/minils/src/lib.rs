use std::error::Error;
use std::fs;

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

    Ok(dir_entries)
}

// Print entries
fn print_entries(entries: Vec<String>) {
    println!("{}", entries.join("  "));
}

pub fn run(paths: &[String]) -> Result<(), Box<dyn Error>> {
    let dir_entries = get_dir_entries(&paths[0])?;

    print_entries(dir_entries);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashSet;

    // Compare Vec<String> without order
    fn compare_unordered_items(actual: Vec<String>, expected: Vec<String>) {
        let a: HashSet<_> = actual.into_iter().collect();
        let e: HashSet<_> = expected.into_iter().collect();
        assert_eq!(a, e);
    }

    // Get Vec<String> for comparison
    fn get_vec_of_str(v_s: Vec<&str>) -> Vec<String> {
        v_s.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn get_entries() {
        match get_dir_entries(&String::from("./src")) {
            Ok(entries) => {
                // Order of entries depends on filesystem/platform environment
                // so compare them w/o order
                compare_unordered_items(get_vec_of_str(vec!["main.rs", "lib.rs"]), entries);
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
}
