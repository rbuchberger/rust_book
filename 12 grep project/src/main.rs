use std::env;
use std::process;

use minigrep::Config;

fn main() {
    let args = env::args().collect();

    let config = Config::parse_args(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1)
    });

    if let Err(e) = minigrep::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
