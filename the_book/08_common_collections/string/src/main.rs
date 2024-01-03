fn main() {
    // Create a new empty string
    let mut _s = String::new();

    // Convert a string literal into String by `to_string`
    let data = "initial contents";
    let s = data.to_string();
    println!("{s}");

    let s = "initial contents".to_string();
    println!("{s}");

    // Convert a string literal into String by `String::from`
    let s = String::from("initial contents");
    println!("{s}");

    // Use different languages
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");

    // Append a string slice to String by `push_str`
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{s}");

    // `push_str` takes a slice, therefore the appendee can be used after appending
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    // Add a single character to a String by `push`
    let mut s = String::from("lo");
    s.push('l');
    println!("{s}");

    // Combine 2 String values
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
                       // it uses a method whose signature looks something like:
                       // fn add(self, s: &str) -> String {
    println!("{s3}");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Combine multiple string values
    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");

    let s1 = String::from("tic");

    // Combine multiple string values by `format!` macro
    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");

    // Compilation error: `the type `String` cannot be indexed by `{integer}``
    // let s1 = String::from("hello");
    // let h = s1[0];

    // String is a wrapper over a `Vec<u8>`,
    // therefore a below string is 24 bytes long
    let _hello = String::from("Здравствуйте");

    // Slice a string with a range of partucular bytes
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("{s}");

    // Iterate for each `char` value
    // -> З, д
    for c in "Зд".chars() {
        println!("{c}");
    }

    // Iterate for each row byte
    // -> 208, 151, 208, 180
    for b in "Зд".bytes() {
        println!("{b}");
    }
}
