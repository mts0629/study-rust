fn main() {
    let s = String::from("hello world");

    // String slices: [starting_index..ending_index]
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello}");
    println!("{world}");

    let s = String::from("hello");

    let slice = &s[0..2];
    println!("{slice}");
    let slice = &s[..2];
    println!("{slice}");

    let len = s.len();
    let slice = &s[3..len];
    println!("{slice}");
    let slice = &s[3..];
    println!("{slice}");

    let slice = &s[0..len];
    println!("{slice}");
    let slice = &s[..];
    println!("{slice}");

    let mut s = String::from("hello world");
    let word = first_word(&s);

    // s.clear(); // Error

    println!("the first word is: {word}");

    let my_string = String::from("hello world\n");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);

    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literal, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // thie works too, without the slice syntax
    let word = first_word(my_string_literal);

    // Slice for an array
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    for i in slice {
        println!("{i}");
    }
}

// Function that returns the first word in a sentence,
// which is a string of words separated by spaces
fn first_word(s: &str) -> &str { // &str: &[u8], String: Vec<u8>
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // Loop for indices and items of the collection
        if item == b' ' { // Search a byte that represents the space
            return &s[0..i];
        }
    }

    &s[..]
}
