use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let paths = minicat::get_paths(&args);

    if let Err(_) = minicat::run(paths) {
        process::exit(1);
    }
}
