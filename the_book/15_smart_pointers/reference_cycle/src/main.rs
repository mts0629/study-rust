use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Cons list
#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

// Tree node
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    // Create a reference cycle
    // a -- b
    // \____/
    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    // Both of references are 2
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // It will cause stack overflow
    // println!("a next item = {:?}", a.tail());

    // Tree structure
    // branch (parent)
    //   \\
    //   leaf (child)
    let leaf = Rc::new(Node {
        value: 3,
        // Child node shouldn't own its parent
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong count = {}, weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            // Parent node should own its children
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        // Set an weak reference of branch as a parent of leaf
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong count = {}, weak count = {}",
            Rc::strong_count(&branch), // branch
            Rc::weak_count(&branch)    // leaf.parent
        );
        println!(
            "leaf strong count = {}, weak count = {}",
            Rc::strong_count(&leaf), // leaf, branch.children
            Rc::weak_count(&leaf)
        );
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong count = {}, weak count = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
