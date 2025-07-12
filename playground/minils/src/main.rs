use minils::run;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(&args[1..]) {
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
        _ => {}
    }
}
