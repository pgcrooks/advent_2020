pub struct Config {
    pub day: i32,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Day and filename arguments required");
        }
        if args.len() > 3 {
            return Err("Too many arguments");
        }

        let default_day = 1;
        let day = args[1].parse().unwrap_or(default_day);

        let filename = args[2].clone();

        Ok(Config { day, filename })
    }
}
