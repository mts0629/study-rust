use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let paths = minicat::get_inputs(&args);

    if let Err(_) = minicat::run(paths) {
        process::exit(1);
    }
}
