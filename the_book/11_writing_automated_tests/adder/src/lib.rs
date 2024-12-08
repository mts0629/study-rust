pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

fn add_two(a: usize) -> usize {
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
}
