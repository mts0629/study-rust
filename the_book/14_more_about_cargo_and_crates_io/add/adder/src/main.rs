// use rand; // Dependency for rand is required in adder/Cargo.toml

fn main() {
    let num = 10;
    println!("Hello, world! {num} plus one is {}", add_one::add_one(num));
}
