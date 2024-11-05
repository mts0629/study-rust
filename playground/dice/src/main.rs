use rand::Rng;

fn main() {
    let sides: i32 = 6;

    println!("{}", rand::thread_rng().gen_range(1..(sides + 1)));
}
