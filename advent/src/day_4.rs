use std::error::Error;
use std::fs;

use regex::Regex;

use advent::Config;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let re = Regex::new(r"([a-z]{3}):").unwrap();

    let mandatory_fields = vec![
        "byr",
        "ecl",
        "eyr",
        "hcl",
        "hgt",
        "iyr",
        "pid",
    ];

    let mut valid_ids = 0;

    let contents = fs::read_to_string(config.filename)?;
    let ids = contents.split("\n\n");
    for id in ids {
        let mut found_tokens: Vec<String> = re.captures_iter(id).filter_map(
            |token| Some(String::from(&token[1]))
        ).collect();

        // Remove 'cid' from the found list since it's optional
        found_tokens.retain(|x| x != "cid");
        // Sort into natural order
        found_tokens.sort();

        if found_tokens == mandatory_fields {
            valid_ids += 1;
        }
    }

    println!("Answer = {}", valid_ids);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

//     fn create_position() -> Position {
//         return Position::new(3, 1);
//     }

//     #[test]
//     fn test_read_lines_into_map_will_return_map() {
//         let test_data = vec![
//             ".#.".to_string(),
//             "#.#".to_string(),
//             "...".to_string(),
//             "###".to_string()
//         ];

//         let expected = vec![
//             vec!['.', '#', '.'],
//             vec!['#', '.', '#'],
//             vec!['.', '.', '.'],
//             vec!['#', '#', '#'],
//         ];
//         assert_eq!(read_lines_into_map(test_data), expected);
//     }

//     #[test]
//     fn test_position_origin_is_0_0() {
//         let pos = create_position();

//         assert_eq!(pos.x, 0);
//         assert_eq!(pos.y, 0);
//     }

//     #[test]
//     fn test_position_slide_will_move() {
//         let mut pos = create_position();

//         assert_eq!(pos.x, 0);
//         assert_eq!(pos.y, 0);

//         pos.slide();
//         assert_eq!(pos.x, 3);
//         assert_eq!(pos.y, 1);
//     }

//     #[test]
//     fn test_toboggan_will_count_2_trees() {
//         let test_data = vec![
//             ".....".to_string(),
//             "#####".to_string(),
//             ".....".to_string(),
//             "#####".to_string()
//         ];

//         assert_eq!(toboggan(&read_lines_into_map(test_data), 3, 1), 2);
//     }

//     #[test]
//     fn test_toboggan_will_count_3_trees() {
//         let test_data = vec![
//             ".....".to_string(),
//             "#####".to_string(),
//             ".....".to_string(),
//             "#####".to_string(),
//             ".....".to_string(),
//             "#####".to_string(),
//             ".....".to_string(),
//         ];

//         assert_eq!(toboggan(&read_lines_into_map(test_data), 3, 1), 3);
//     }
}
