use std::env;
use ripgrep::*;
use std::process;

fn main() {    
    let args: Vec<String> = env::args().collect();

    let c = parse_config(&args).unwrap_or_else(|err| {
        println!("Problem parsing the arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(&c) {
        println!("Application error: {}", e);

        process::exit(1);
    }
}