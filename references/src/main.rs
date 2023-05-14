fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); // Borrowing

    println!("The length of '{s1}' is {len}.");

    let _s = String::from("hello");

    // change(&_s); // Compilation error

    let mut s = String::from("hello");

    change_mut(&mut s); // Using mutable reference

    println!("{s}");

    // let r1 = &mut s;
    // let r2 = &mut s;
    // println!("{r1}, {r2}"); // Compilation error
                            // Only one mutable reference can borrow at a time
                            // It can prevent "data races" at a compile time

    {
        let _r1 = &mut s;
    } // r1 goes out of scope here

    let _r2 = &mut s; // No problem

    // let r1 = &s; // No problem
    // let r2 = &s; // No problem
    // let r3 = &mut s;

    // println!("{}, {}, and {}", r1, r2, r3); // Compilation error
                                            // We cannot have a mutable reference
                                            // while we have an immutable one to the
                                            // same value

    let mut s = String::from("hello");

    let r1 = &s; // No problem
    let r2 = &s; // No problem
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point

    let r3 = &mut s; // No problem
    println!("{r3}");

    // let reference_to_nothing = dangle();    // Get a reference created inside `dangle`
                                            // Dangling pointer, not worked

    let s = no_dangle(); // Get the String directly
    println!("{s}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a string
    s.len()
} // Here, s goes out of scope.  But because it does not have ownership of what
  // it refers to, it is not dropped.

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change_mut(some_string: &mut String) {
    some_string.push_str(", world");
}

// fn dangle() -> &String { // Compilation error: missing lifetime specifier
//     let s = String::from("hello");
//
//     &s
// } // Here, s goes out of scope, and is dropped.  Its memory goes away

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
