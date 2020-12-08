use std::io::{self, Write};
use std::error::Error;
use std::fs;

use regex::Regex;

use advent::Config;

type FieldValidator = fn(data: &str) -> bool;

fn byr_valid(data: &str) -> bool {
    // byr (Birth Year) - four digits; at least 1920 and at most 2002.
    let re = Regex::new(r"^[0-9]{4}$").unwrap();
    if !re.is_match(data) {
        return false;
    }

    let year: i32 = data.parse().unwrap();
    if year < 1920 || year > 2002 {
        return false;
    }

    return true;
}

fn ecl_valid(data: &str) -> bool {
    // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
    let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    return re.is_match(data);
}

fn eyr_valid(data: &str) -> bool {
    // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
    let re = Regex::new(r"^[0-9]{4}$").unwrap();
    if !re.is_match(data) {
        return false;
    }

    let year: i32 = data.parse().unwrap();
    if year < 2020 || year > 2030 {
        return false;
    }

    return true;
}

fn hcl_valid(data: &str) -> bool {
    // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
    let re = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    return re.is_match(data);
}

fn hgt_valid(data: &str) -> bool {
    // hgt (Height) - a number followed by either cm or in:
    //     If cm, the number must be at least 150 and at most 193.
    //     If in, the number must be at least 59 and at most 76.
    let re = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
    let cap = re.captures(data);
    if cap.is_none() {
        return false;
    }

    // Calling unwrap will move the data, so grab it here to be used later
    let raw_cap = cap.unwrap();

    // Parse the units and height
    let height: i32 = raw_cap[1].parse().unwrap();
    let unit = &raw_cap[2];

    if unit == "cm" {
        if height < 150 || height > 193 {
            return false;
        }
    } else {
        if height < 59 || height > 76 {
            return false;
        }
    }

    return true;
}

fn iyr_valid(data: &str) -> bool {
    // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
    let re = Regex::new(r"^[0-9]{4}$").unwrap();
    if !re.is_match(data) {
        return false;
    }

    let year: i32 = data.parse().unwrap();
    if year < 2010 || year > 2020 {
        return false;
    }

    return true;
}

fn pid_valid(data: &str) -> bool {
    // pid (Passport ID) - a nine-digit number, including leading zeroes.
    let re = Regex::new(r"^[0-9]{9}$").unwrap();
    return re.is_match(data);
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let token_re = Regex::new(r"([a-z]{3}):").unwrap();

    let mandatory_fields: Vec<(&str, FieldValidator)> = vec![
        ("byr", byr_valid),
        ("ecl", ecl_valid),
        ("eyr", eyr_valid),
        ("hcl", hcl_valid),
        ("hgt", hgt_valid),
        ("iyr", iyr_valid),
        ("pid", pid_valid),
    ];

    // Build a list of the expected tokens
    let expected_tokens: Vec<&str> = mandatory_fields.iter().filter_map(
        |item| Some(item.0)).collect();

    let mut valid_ids = 0;

    let contents = fs::read_to_string(config.filename)?;
    let ids = contents.split("\n\n");
    println!("Processing IDs");
    for id in ids {
        let mut found_tokens: Vec<String> = token_re.captures_iter(id).filter_map(
            |token| Some(String::from(&token[1]))
        ).collect();

        // Remove 'cid' from the found list since it's optional
        found_tokens.retain(|x| x != "cid");
        // Sort into natural order
        found_tokens.sort();
    
        if found_tokens == expected_tokens {
            // All mandatory tokens are present, check that they are valid
            let mut valid = true;
            for token in &mandatory_fields {
                let formatted = format!(r"{}:([a-z0-9#]+)", token.0);
                let val_re = Regex::new(formatted.as_str()).unwrap();
                let result = val_re.captures(id);
                let value = &result.unwrap()[1];
                if !token.1(value) {
                    valid = false;
                    break;
                }
            }
            if valid {
                valid_ids += 1;
            }
        }
        print!(".");
        io::stdout().flush().unwrap();
    }

    println!("\nDone\n\nAnswer = {}", valid_ids);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byr_valid_when_letters_will_return_false() {
        assert_eq!(byr_valid("abdc"), false);
    }

    #[test]
    fn test_byr_valid_when_short_will_return_false() {
        assert_eq!(byr_valid("123"), false);
    }

    #[test]
    fn test_byr_valid_when_long_will_return_false() {
        assert_eq!(byr_valid("12345"), false);
    }

    #[test]
    fn test_byr_valid_when_too_old_will_return_false() {
        assert_eq!(byr_valid("1919"), false);
    }

    #[test]
    fn test_byr_valid_when_too_young_will_return_false() {
        assert_eq!(byr_valid("2003"), false);
    }

    #[test]
    fn test_byr_valid_when_oldest_will_return_true() {
        assert_eq!(byr_valid("1920"), true);
    }

    #[test]
    fn test_byr_valid_when_youngest_will_return_true() {
        assert_eq!(byr_valid("2002"), true);
    }

    #[test]
    fn test_ecl_valid_when_not_color_will_return_false() {
        let test_data = vec!["foo", "123"];
        for data in test_data {
            assert_eq!(ecl_valid(data), false);
        }
    }

    #[test]
    fn test_ecl_valid_when_color_will_return_true() {
        let test_data = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        for data in test_data {
            assert_eq!(ecl_valid(data), true);
        }
    }

    #[test]
    fn test_eyr_valid_when_letters_will_return_false() {
        assert_eq!(eyr_valid("abdc"), false);
    }

    #[test]
    fn test_eyr_valid_when_short_will_return_false() {
        assert_eq!(eyr_valid("123"), false);
    }

    #[test]
    fn test_eyr_valid_when_long_will_return_false() {
        assert_eq!(eyr_valid("12345"), false);
    }

    #[test]
    fn test_eyr_valid_when_too_old_will_return_false() {
        assert_eq!(eyr_valid("2019"), false);
    }

    #[test]
    fn test_eyr_valid_when_too_young_will_return_false() {
        assert_eq!(eyr_valid("2031"), false);
    }

    #[test]
    fn test_eyr_valid_when_oldest_will_return_true() {
        assert_eq!(eyr_valid("2020"), true);
    }

    #[test]
    fn test_eyr_valid_when_youngest_will_return_true() {
        assert_eq!(eyr_valid("2030"), true);
    }

    #[test]
    fn test_hcl_valid_when_not_hex_will_return_false() {
        assert_eq!(hcl_valid("xyz"), false);
    }

    #[test]
    fn test_hcl_valid_when_hex_will_return_false() {
        assert_eq!(hcl_valid("#123abc"), true);
    }

    #[test]
    fn test_hgt_valid_when_invalid_will_return_false() {
        let test_data = vec!["foo", "123"];
        for data in test_data {
            assert_eq!(hgt_valid(data), false);
        }
    }

    #[test]
    fn test_hgt_valid_when_invalid_cm_will_return_false() {
        let test_data = vec!["149cm", "194cm"];
        for data in test_data {
            assert_eq!(hgt_valid(data), false);
        }
    }

    #[test]
    fn test_hgt_valid_when_valid_cm_will_return_true() {
        let test_data = vec!["150cm", "193cm"];
        for data in test_data {
            assert_eq!(hgt_valid(data), true);
        }
    }

    #[test]
    fn test_hgt_valid_when_invalid_inch_will_return_false() {
        let test_data = vec!["58in", "77in"];
        for data in test_data {
            assert_eq!(hgt_valid(data), false);
        }
    }

    #[test]
    fn test_hgt_valid_when_valid_inch_will_return_true() {
        let test_data = vec!["59in", "76in"];
        for data in test_data {
            assert_eq!(hgt_valid(data), true);
        }
    }

    #[test]
    fn test_iyr_valid_when_letters_will_return_false() {
        assert_eq!(iyr_valid("abdc"), false);
    }

    #[test]
    fn test_iyr_valid_when_short_will_return_false() {
        assert_eq!(iyr_valid("123"), false);
    }

    #[test]
    fn test_iyr_valid_when_long_will_return_false() {
        assert_eq!(iyr_valid("12345"), false);
    }

    #[test]
    fn test_iyr_valid_when_too_old_will_return_false() {
        assert_eq!(iyr_valid("2009"), false);
    }

    #[test]
    fn test_iyr_valid_when_too_young_will_return_false() {
        assert_eq!(iyr_valid("2021"), false);
    }

    #[test]
    fn test_iyr_valid_when_oldest_will_return_true() {
        assert_eq!(iyr_valid("2010"), true);
    }

    #[test]
    fn test_iyr_valid_when_youngest_will_return_true() {
        assert_eq!(iyr_valid("2020"), true);
    }

    #[test]
    fn test_pid_valid_when_letters_will_return_false() {
        assert_eq!(pid_valid("abc"), false);
    }

    #[test]
    fn test_pid_valid_when_too_short_will_return_false() {
        assert_eq!(pid_valid("12345678"), false);
    }

    #[test]
    fn test_pid_valid_when_too_long_will_return_false() {
        assert_eq!(pid_valid("1234567890"), false);
    }

    #[test]
    fn test_pid_valid_when_valid_will_return_true() {
        assert_eq!(pid_valid("123456789"), true);
    }
}
