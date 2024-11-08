use clap::Parser;
use rand::Rng;

/// Rolling dices
#[derive(Parser, Debug)]
struct Args {
    /// Number of sides
    #[arg(short, long, default_value_t = 6)]
    sides: u32,
    /// Number of rolls
    #[arg(short, long, default_value_t = 1)]
    rolls: u32,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.rolls {
        println!("{}", rand::thread_rng().gen_range(1..(args.sides + 1)));
    }
}
