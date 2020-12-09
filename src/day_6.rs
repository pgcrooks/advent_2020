use std::error::Error;
use std::fs;
use std::io::{self, Write};

use advent::Config;

fn count_group_answers(group: &str) -> usize {
    // Remove newlines from the char list
    let mut questions: Vec<char> = group.chars().collect();
    questions.retain(|x| x != &'\n');
    questions.sort();
    questions.dedup();
    return questions.len();
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let groups = contents.split("\n\n");
    println!("Processing groups");
    let mut sum = 0;
    for group in groups {
        print!(".");
        io::stdout().flush().unwrap();
        sum += count_group_answers(group);
    }

    println!("\nSum of questions are {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_group_answers_will_return_count() {
        let test_data = vec![
            ("abc", 3),
            ("a\nb\nc", 3),
            ("ab\nac", 3),
            ("a\na\na\na", 1),
            ("b", 1),
            ("aceg\nbdf\nab\ncf", 7),
        ];
        for test in test_data {
            assert_eq!(count_group_answers(test.0), test.1);
        }
    }
}
