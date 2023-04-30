fn main() {
    println!("Hello, world!");

    another_function(5, 'h');

    // An expression returns an evaluated value, but a statement doesn't
    // Error: an expression is expected, but `let` is a statement
    // let x = (let y = 6);

    // Function calling, macro calling and block `{}` are expressions
    let y = {
        let x = 3;
        // A statement has `;`, but an expression doesn't
        x + 1 // An expression, evaluated as 4
    };
    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);

    let x = plus_one(5);
    println!("The value of x is: {}", x);
}

// Function terminated with a statement
fn another_function(x: i32, unit_label: char) {
    println!("The value of x is: {}{}", x, unit_label);
}

// Function terminated with an expression
fn five() -> i32 { // Return a value of i32 type
    5 // Return 5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
