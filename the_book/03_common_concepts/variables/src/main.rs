fn main() {
    // Mutable variable
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constant
    const MAX_POINTS: u32 = 100_000;
    println!("MAX_POINTS = {}", MAX_POINTS);

    let y = 5;
    // Shadowing
    let y = y + 1;
    {
        // Shadowing in the inner scope
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);
}
