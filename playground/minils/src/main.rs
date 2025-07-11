use minils::run;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match run(&args[1..]) {
        Ok(paths) => {
            for path in paths {
                println!("{path}")
            }
        }
        Err(err) => {
            eprintln!("{err}");
            process::exit(1);
        }
    }
}
