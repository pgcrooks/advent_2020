use std::env;
use std::error::Error;
use std::process;

mod day_1;
use advent::Config;

fn run_help(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Unrecognised day: {}", config.day);
    println!("Current supported days: [1]");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Advent 2020\n=====\n");
    println!("Running Day {}", config.day);
    println!("Reading from {}\n", config.filename);

    type DayRunner = fn(Config) -> Result<(), Box<dyn Error>>;

    let runner: DayRunner = match config.day {
        1 => day_1::run,
        _ => run_help,
    };

    if let Err(e) = runner(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
