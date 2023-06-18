// - Each value in Rust has an owner
// - There can only be one owner at a time
// - When the owner goes out of scope, the value will be dropped
fn main() {
    {                       // s is not valid here, it's not yet declared
        let s = "hello";    // s is valid from this point forward
        println!("{s}");
    }                       // This scope is now over, and s is no longer valid

    {
        let mut s = String::from("hello");
        s.push_str(", world!"); // String::push_str() append a literal to a String
        println!("{s}");        // Print `hello, world!`
    }                           // This scope is now over, and s is no longer valid
                                // The `drop` function is called automatically

    let s1 = String::from("hello");
    let s2 = s1;                    // A pointer which indicates "hello" is shared with s1

    println!("{s2}");
    // println!("{s1}");    // Compilation error: `borrow of moved value`
                            // s1 is "moved" to s2 and no longer valid
                            // It avoids the "double free" of the heap data

    let s1 = String::from("hello");
    let s2 = s1.clone(); // Copy deeply the heap data

    println!("s1 = {s1}, s2 = {s2}");

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);   // The types which are stored on the stack
                                        // implement the `Copy` trait and don't be moved
    // Some types that implement `Copy`:
    //  integer (e.g. i32), bool, floating-point (e.g. f64), char,
    //  tuples only contain types that also implement `Copy`

    {
        let s = String::from("hello");

        takes_ownership(s); // s's value moves int the function
                            // and so is no longer valid here

        // println!("{s}");    // Compliation error

        let x = 5;

        makes_copy(x);  // x would move into the function,
                        // but i32 is Copy, so it's okey to still
                        // use x afterward

        println!("{x}");
    } // x goes out of scope, then s.
      // But because s's value was moved, nothing special happens.

    {
        let s1 = gives_ownership();         // gives_ownership moves its return value
                                            // into s1

        let s2 = String::from("hello");

        let s3 = takes_and_gives_back(s2);  // s2 is moved into takes_and_gives_back,
                                            // which also moves its return value into s3
    } // s3 and s1 are dropped.  s2 was moved

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1); // Get multiple return values

    println!("The length of '{s2}' is {len}.");
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
} // `drop` is called.  The backing memory is freed

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
} // some_integer goes out of scope, but nothing special happens

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string // some_string is returned and moves out to the calling function
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length) // Return multiple values using a tuple
}
