use std::env;

use rand::Rng;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let rolls: u32 = if args.len() == 0 {
        1
    } else {
        match &args[0].trim().parse() {
            Ok(num) => *num,
            Err(_) => return,
        }
    };

    let sides: u32 = 6;

    for _ in 0..rolls {
        println!("{}", rand::thread_rng().gen_range(1..(sides + 1)));
    }
}
