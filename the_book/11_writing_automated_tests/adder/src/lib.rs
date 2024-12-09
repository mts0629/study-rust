use std::{thread, time};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn add_two(a: usize) -> usize {
    a + 2
    // a + 3 // Bug
}

#[cfg(test)]
mod tests {
    use super::*; // Bring outer modules

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn it_works() -> Result<(), String> {
        let result = add(2, 2);

        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    /*#[test]
    fn another() {
        panic!("Make this test fail");
    }*/

    #[test]
    fn it_adds_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_two_and_two() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn add_three_and_two() {
        let result = add_two(3);
        assert_eq!(result, 5);
    }

    #[test]
    fn one_hundred() {
        let result = add_two(100);
        assert_eq!(result, 102);
    }

    #[test]
    // Ignore this test,
    // run ignored tests with `cargo test -- --ignored` and
    // run all tests with `cargo test -- --include-ignored`
    #[ignore]
    fn expensive_test() {
        // Code that takes time...
        let one_hundred_sec = time::Duration::from_secs(100);
        let now = time::Instant::now();

        thread::sleep(one_hundred_sec);

        assert!(now.elapsed() >= one_hundred_sec);
    }
}
