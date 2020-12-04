use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Filename argument required");
        }
        if args.len() > 2 {
            return Err("Too many arguments");
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}
