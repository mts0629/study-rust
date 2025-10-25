use std::fmt;
use std::ops::Add;

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    // Associated type, decide a type on implementation
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

// Add trait has default generic type parameter
// for Rhs (righthand-side operand)
// trait Add<Rhs=Self> { ... }
impl Add for Point {
    type Output = Point;

    // Overload '+' operator
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    // Overload '+' operator for Millimeters, to add Meters
    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// OutlinePrint trait, which requires fmt::Display as a supertrait
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string(); // From Display trait
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {output} *");
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point2D {
    x: i32,
    y: i32,
}

// Implement fmt::Display trait:
// supertrait of OutlinePrint to Point2D
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Implement OutlinePrint trait to Point2D
impl OutlinePrint for Point2D {}

// Newtype pattern:
// implement trait to a local wrapper tuple struct,
// which has a type defined outside our crate
struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let mut counter = Counter { count: 0 };

    for i in 1..=5 {
        let c = counter.next();
        match c {
            Some(i) => println!("{i}"),
            None => {}
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 },
    );

    let m = Meters { 0: 1 };
    let mm = Millimeters { 0: 100 };
    println!("{:?}", mm + m);

    // Call specific method implemented on each trait
    let person = Human;
    person.fly(); // Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);

    // Associated function on Dog type will be called
    println!("A baby dog is called a {}", Dog::baby_name());
    // Call an associated function on Animal trait by fully qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p = Point2D { x: 1, y: 2 };
    p.outline_print();

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
