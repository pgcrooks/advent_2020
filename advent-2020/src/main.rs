use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Advent 2020");
    println!("Input file: {:?}", config.filename);

    let contents = fs::read_to_string(config.filename)
        .expect("Something went wrong while reading the file");

    println!("Contents:\n{}", contents);
}

struct Config {
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 2 {
            panic!("Filename argument required");
        }
        if args.len() > 2 {
            panic!("Too many arguments");
        }
        let filename = args[1].clone();
        Config { filename }
    }
}
