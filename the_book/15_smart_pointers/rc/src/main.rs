// Cons list
#[derive(Debug)]
enum List {
    // Reference counted smart pointer
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    // clone increases the number of references
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        // b and c are sharing a
        let c = Cons(4, Rc::clone(&a));
        println!("{:?}", c);
        println!("count after creating c = {}", Rc::strong_count(&a));
    } // Reference count is decreased automatically

    println!("{:?}", b);
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
} // Rc<T> is cleaned up completely
