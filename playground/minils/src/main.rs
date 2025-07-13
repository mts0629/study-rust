use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let paths = minils::get_paths(&args);

    if let Err(err) = minils::run(paths) {
        eprintln!("{err}");
        process::exit(1);
    }
}
