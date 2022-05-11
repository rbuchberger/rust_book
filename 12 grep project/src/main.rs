use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("{:#?}", args);
    println!("Searching for {}", query);
    println!("In {}", filename);

    let contents = fs::read_to_string(filename).expect("Could not read file");

    println!("File contents:\n{}", contents)
}
