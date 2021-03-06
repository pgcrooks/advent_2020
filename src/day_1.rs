use rand::{
    thread_rng,
    seq::SliceRandom,
};
use std::error::Error;

fn find_sum_2(numbers: &[String], desired: i32) -> Option<(i32, i32)> {
    for win in numbers.windows(2) {
        let a: i32 = win[0].parse().unwrap();
        let b: i32 = win[1].parse().unwrap();
        if a + b == desired {
            return Some((a, b))
        }
    }
    None
}

fn find_sum_3(numbers: &[String], desired: i32) -> Option<(i32, i32, i32)> {
    for win in numbers.windows(3) {
        let a: i32 = win[0].parse().unwrap();
        let b: i32 = win[1].parse().unwrap();
        let c: i32 = win[2].parse().unwrap();
        if a + b + c == desired {
            return Some((a, b, c))
        }
    }
    None
}

pub fn run(config: advent::Config) -> Result<(), Box<dyn Error>> {
    let mut lines = advent::lines_from_file(config.filename)?;

    let desired = 2020;

    let mut rng = thread_rng();
    let mut done = false;
    while !done {
        let result = find_sum_2(&lines, desired);
        if let Some(i) = result {
            println!("Found 2 numbers! {} + {} = {}", i.0, i.1, desired);
            println!("Answer = {}", i.0 * i.1);
            done = true;
        }
        &lines.shuffle(&mut rng);
    }
    
    let mut done = false;
    while !done {
        let result = find_sum_3(&lines, desired);
        if let Some(i) = result {
            println!("Found 3 numbers! {} + {} + {} = {}", i.0, i.1, i.2, desired);
            println!("Answer = {}", i.0 * i.1 * i.2);
            done = true;
        }
        &lines.shuffle(&mut rng);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generate_vector() -> Vec<String> {
        return vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
        ];
    }

    #[test]
    fn test_find_sum_2_when_found_will_return_tuple() {
        let data = generate_vector();
        assert_eq!(find_sum_2(&data[..], 7), Some((3, 4)));
    }
    #[test]
    fn test_find_sum_2_when_not_found_will_return_none() {
        let data = generate_vector();
        assert_eq!(find_sum_2(&data[..], 11), None);
    }

    #[test]
    fn test_find_sum_3_when_found_will_return_tuple() {
        let data = generate_vector();
        assert_eq!(find_sum_3(&data[..], 6), Some((1, 2, 3)));
    }
    #[test]
    fn test_find_sum_3_when_not_found_will_return_none() {
        let data = generate_vector();
        assert_eq!(find_sum_3(&data[..], 11), None);
    }
}
