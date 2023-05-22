#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Define methods for the Rectangle
    // Method to calculate an area
    // &self -> self: &Self -> rectangle: &Rectangle
    fn area(&self) -> u32 { // Borrowing
        self.width * self.height
    }

    // Getter method of width
    fn width(&self) -> bool {
        self.width > 0
    }

    // Method to check whether the rectangle can hold an another one
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle {
    // Associated function to create a square instance
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    // Call an associated function
    let sq = Rectangle::square(20);
    println!("{:#?}", sq);
}

