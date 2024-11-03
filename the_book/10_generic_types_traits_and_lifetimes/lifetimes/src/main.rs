// Lifetime annotations in a function signature
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // -> Returned reference will be valid
    //    as long as both the parameters are valid
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Invalid
// fn longest<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// } // Lifetime of result finishes here

// This function is compiled without lifetime annoations
// because of lifetime elision rules
fn _first_word(s: &str) -> &str {
    // -> fn _first_word<'a>(s: &'a str) ->'a str { ... }
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn _level(&self) -> i32 {
        3
    }

    fn _announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {announcement}");
        self.part
    }
}

fn main() {
    // {                        // Borrow checker checks lifetimes
    //     let r;               // --------+- r
    //                          //         |
    //     {                    //         |
    //         let x = 5;       // ---+- x |
    //         r = &x;          //    |    |
    //     }                    // ---+    |
    //                          //         |
    //     println!("r: {r}");  // NG      |
    // }                        // --------+
    {                        // 
        let x = 5;           // --------+- r
                             //         |
        let r = &x;          // ---+- x |
                             //    |    |
        println!("r: {r}");  // OK |    |
                             // ---+    |
    }                        // --------+

    let string1 = String::from("abcd");
    let string2 = "xyz";

    // `longest` takes two slices and returns a slice
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");

    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    } // string2 finishes its lifetime

    // Invalid
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // } // string2 finishes its lifetime -> invalid for lifetime constraints
    // println!("The longest string is {result}");

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let _i = ImportantExcerpt {
        part: first_sentence,
    };

    // Static lifetime, can live for the entire duration on the program
    let _st: &'static str = "I have a static lifetime.";
}
