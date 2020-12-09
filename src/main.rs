use std::env;
use std::error::Error;
use std::process;

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;

use advent::Config;

fn run_help(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Unrecognised day: {}", config.day);
    println!("Current supported days: [1, 2, 3, 4, 5]");
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Advent 2020");
    println!("===========\n");
    println!("Running Day {}", config.day);

    type DayRunner = fn(Config) -> Result<(), Box<dyn Error>>;

    let runner: DayRunner = match config.day {
        1 => day_1::run,
        2 => day_2::run,
        3 => day_3::run,
        4 => day_4::run,
        5 => day_5::run,
        _ => run_help,
    };

    if let Err(e) = runner(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
