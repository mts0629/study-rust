use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let paths = minicat::get_args(&args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if let Err(_) = minicat::run(paths) {
        process::exit(1);
    }
}
