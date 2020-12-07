use std::env;
use std::process;

use advent::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Advent 2020\n=====\n");
    println!("Running Day {}", config.day);
    println!("Reading from {}\n", config.filename);

    if let Err(e) = advent::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
