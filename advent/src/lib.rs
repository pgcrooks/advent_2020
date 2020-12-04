use rand::{
    thread_rng,
    seq::SliceRandom,
};
use std::error::Error;
use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
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

fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut lines = lines_from_file(config.filename)?;

    let desired = 2020;

    let mut rng = thread_rng();
    let mut done = false;
    while !done {
        &lines.shuffle(&mut rng);

        for win in lines.windows(2) {
            let a: i32 = win[0].parse().unwrap();
            let b: i32 = win[1].parse().unwrap();
            if a + b == desired {
                println!("Found it! {} + {} = {}", a, b, desired);
                println!("Answer = {}", a * b);
                done = true;
            }
        }
    }

    Ok(())
}