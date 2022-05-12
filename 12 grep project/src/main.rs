use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn parse_args(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config { query, filename })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_args(&args).unwrap_or_else(|error| {
        println!("Problem parsing arguments: {}", error);
        process::exit(1)
    });

    println!("{:#?}", args);
    println!("Searching for {}", config.query);
    println!("In {}", config.filename);

    let contents = fs::read_to_string(config.filename).expect("Could not read file");

    println!("File contents:\n{}", contents)
}

