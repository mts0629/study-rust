fn fizzbuzz(value: i32) {
    match value % 15 {
        0 => println!("fizzbuzz"),
        12 | 9 | 6 | 3 => println!("fizz"),
        10 | 5 => println!("buzz"),
        _ => println!("{}", value),
    }
}

fn main() {
    for n in 1..=30 {
        fizzbuzz(n);
    }
}
