fn main() {
    // Type alias: Kilometers, synonym for i32
    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x + y);

    // Reduce repetition by type alias
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // let f: Box<dyn Fn() + Send + 'static> = Box::new(|| println!("hi"));
    let f: Thunk = Box::new(|| println!("hi"));
}
