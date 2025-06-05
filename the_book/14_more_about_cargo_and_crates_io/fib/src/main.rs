// Time consuming process:
// Return n-th term of Fibonacci sequence
fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2),
    }
}

fn main() {
    for i in 0..42 {
        println!("{}", fib(i));
    }
}
