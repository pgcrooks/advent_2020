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

fn find_sum_2(numbers: &[String], desired: i32) -> Option<i32> {
    for win in numbers.windows(2) {
        let a: i32 = win[0].parse().unwrap();
        let b: i32 = win[1].parse().unwrap();
        if a + b == desired {
            println!("find_sum_2 {} + {} = {}", a, b, desired);
            return Some(a * b)
        }
    }
    None
}

fn find_sum_3(numbers: &[String], desired: i32) -> Option<i32> {
    for win in numbers.windows(3) {
        let a: i32 = win[0].parse().unwrap();
        let b: i32 = win[1].parse().unwrap();
        let c: i32 = win[2].parse().unwrap();
        if a + b + c == desired {
            println!("find_sum_3 {} + {} + {} = {}", a, b, c, desired);
            return Some(a * b * c)
        }
    }
    None
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut lines = lines_from_file(config.filename)?;

    let desired = 2020;

    let mut rng = thread_rng();
    let mut done = false;
    while !done {
        let result = find_sum_2(&lines, desired);
        if let Some(i) = result {
            println!("Found 2 numbers! Answer = {}", i);
            done = true;
        }
        &lines.shuffle(&mut rng);
    }
    
    let mut done = false;
    while !done {
        let result = find_sum_3(&lines, desired);
        if let Some(i) = result {
            println!("Found 3 numbers! Answer = {}", i);
            done = true;
        }
        &lines.shuffle(&mut rng);
    }

    Ok(())
}