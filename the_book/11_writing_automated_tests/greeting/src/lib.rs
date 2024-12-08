pub fn greeting(name: &str) -> String {
    // format!("Hello {name}!")
    String::from("Hello!") // Bug
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        // Assertion with the custom error message
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{result}`"
        );
    }
}
