use std::error::Error;

use advent::Config;

struct NumberAllowed {
    pub min: usize,
    pub max: usize,
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
        let allowed_count = NumberAllowed {
            min: number_vec[0].parse().unwrap(),
            max: number_vec[1].parse().unwrap()
        };

        let character = String::from(split_vec[1]).remove(0);
        let data = String::from(split_vec[2]);

        let char_count = data.matches(character).count();
        if char_count >= allowed_count.min && char_count <= allowed_count.max {
            valid_password_count += 1;
        }
    }

    println!("Found {} valid passwords", valid_password_count);

    Ok(())
}
