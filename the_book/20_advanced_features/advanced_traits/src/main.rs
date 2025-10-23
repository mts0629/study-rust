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
}
