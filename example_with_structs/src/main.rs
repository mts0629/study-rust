#[derive(Debug)]    // Attribute for debugging
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixel.",
        area(&rect1)
    );

    println!("rect1 is {:?}", rect1);   // Pretty printing with 'Debug' trait
    println!("rect1 is {:#?}", rect1);  // With newlines

    // `dbg!` macro: print an expression value, its codes and line number to stderr
    // It doesn't take ownership of the expression
    let scale = 2;
    let rect2 = Rectangle {
        width: dbg!(30 * scale), // Print field
        height: 50,
    };

    dbg!(&rect2); // Print struct (pretty-printing)
}

// Calculate the area of the rectangle by its width and height
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

