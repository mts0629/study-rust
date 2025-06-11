use std::ops::Deref;

// My smart pointer
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Implement Deref trait
impl<T> Deref for MyBox<T> {
    type Target = T; // Associated type

    // deref method:
    // borrows self and returns a reference to the inner data
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // *(y.deref())

    let m = MyBox::new(String::from("Rust"));
    // Deref coercion:
    // deref converts &MyBox<String> -> &String, and then &String -> &str
    hello(&m);
    // hello(&(*m)[..]); // Works samely
}
