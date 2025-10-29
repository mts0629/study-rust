// Diverging functions with never type
fn bar() -> ! {
    // Infinite loop
    loop {
        println!("forever");
    }
}

fn main() {
    // Type alias: Kilometers, synonym for i32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Reduce repetition by type alias
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk = Box::new(|| println!("hi"));

    // str is Dynamically sized types (DST)
    // Its size is not known at compile time, so use a reference
    let s1: &str = "Hello there!";
    let s2: &str = "How's it going?";
}

// Sized trait: size is known at compile time
fn generic<T: Sized>(t: T) {
    // ...
}

// Sized trait: size is known at compile time
fn generic_fuzzy<T: ?Sized>(t: &T) {
    // ...
}
