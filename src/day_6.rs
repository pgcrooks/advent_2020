use std::error::Error;
use std::fs;

use advent::Config;

fn count_newlines(s: &str) -> usize {
    s.as_bytes().iter().filter(|&&c| c == b'\n').count() + 1
}

fn count_group_answers(group: &str) -> usize {
    let group_size = count_newlines(group);

    // Remove newlines from the char list
    let mut questions: Vec<char> = group.chars().collect();
    questions.retain(|x| x != &'\n');
    questions.sort();
    let mut count = 0;
    let mut unique_questions: Vec<char> = questions.clone();
    unique_questions.dedup();
    for qu in &unique_questions {
        let number = questions.iter().filter(|&n| n == qu).count();
        if number == group_size {
            count += 1;
        }
    }
    println!("Group={} size={} count={}\n\n", group, group_size, count);
    return count;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let groups = contents.split("\n\n");
    println!("Processing groups");
    let mut sum = 0;
    for group in groups {
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
            ("a\nb\nc", 0),
            ("ab\nac", 1),
            ("a\na\na\na", 1),
            ("b", 1),
        ];
        for test in test_data {
            assert_eq!(count_group_answers(test.0), test.1);
        }
    }
}
