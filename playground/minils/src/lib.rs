use std::error::Error;
use std::fs;

// Get a list of entries in the specified path
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
        let args = vec![String::from("./src")];

        match run(&args) {
            Ok(entries) => {
                // Order of entries depends on filesystem/platform environment
                // so compare them w/o order
                compare_unordered_items(get_vec_of_str(vec!["main.rs", "lib.rs"]), entries);
            }
            Err(_) => assert!(false),
        }
    }
}
