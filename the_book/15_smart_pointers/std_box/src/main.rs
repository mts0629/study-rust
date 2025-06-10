// Cons list
enum List {
    // Size of recursive type can't be known at compile time,
    // so hold a Box's pointer
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    // Box<T>, data stored on the heap
    let b = Box::new(5);

    println!("b = {b}");

    // Cons list
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // Iterate the cons list
    let mut cell = &list;
    while let List::Cons(car, cdr) = &cell {
        println!("{}", car);
        cell = &cdr;
    }
}
