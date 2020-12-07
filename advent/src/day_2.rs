use std::error::Error;
use std::convert::TryInto;

use advent::Config;

struct Positions {
    pub first: usize,
    pub second: usize,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Hello from day 2!");
    let lines = advent::lines_from_file(config.filename)?;

    let mut valid_password_count: i32 = 0;

    for line in lines {
        // Parse the line into interesting fields
        let split_vec = line.split(" ").collect::<Vec<&str>>();

        let number_str = String::from(split_vec[0]);
        let number_vec = number_str.split("-").collect::<Vec<&str>>();
        let positions = Positions {
            first: number_vec[0].parse().unwrap(),
            second: number_vec[1].parse().unwrap()
        };

        let character = String::from(split_vec[1]).remove(0);
        let data = String::from(split_vec[2]);

        // Ignore passwords that are too short
        // Data is 1-indexed
        if (data.len() + 1) >= positions.second.try_into().unwrap() {
            let string_as_bytes = data.as_bytes();
            let first_unicode: u8 = string_as_bytes[positions.first - 1];
            let first_char: char = first_unicode as char;
            let second_unicode: u8 = string_as_bytes[positions.second - 1];
            let second_char: char = second_unicode as char;

            if (first_char == character) ^ (second_char == character) {
                valid_password_count += 1;
            }
        }
    }

    println!("Found {} valid passwords", valid_password_count);

    Ok(())
}
