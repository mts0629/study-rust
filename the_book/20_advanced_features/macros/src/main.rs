use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;
use hello_macro_derive::prologue;
use hello_macro_derive::uppercase;

// Declarative macro
// Initialization of vector like `vec!` macro
#[macro_export]
macro_rules! vec_new {
    ($($x:expr),*) => { // Comma-separated expressions
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )* // Repeated for expressions
            temp_vec
        }
    };
}

// Implement standard behavior of `HelloMacro` trait
// to `Pancakes` struct
#[derive(HelloMacro)]
struct Pancakes;

// Attribute-like macro
#[prologue]
fn hello_after_prologue() {
    println!("Hello");
}

fn main() {
    let v: Vec<u32> = vec_new![1, 2, 3];

    for item in v {
        println!("{item}");
    }

    // Call a `hello_macro` function from `HelloMacro` trait
    Pancakes::hello_macro();

    hello_after_prologue();

    // Call a function-like macro
    println!("{}", uppercase!("hello"));
}
