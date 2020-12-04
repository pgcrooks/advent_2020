use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = parse_config(&args);

    println!("Advent 2020");
    println!("Input file: {:?}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong while reading the file");

    println!("Contents:\n{}", contents);
}

struct Config {
    filename: String,
}

fn parse_config(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("Filename argument required");
    }
    let filename = args[1].clone();
    Config { filename }
}
