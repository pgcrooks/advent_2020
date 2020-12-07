use std::convert::TryInto;
use std::error::Error;

use advent::Config;

mod position_mod {
    pub struct Position {
        pub x: usize,
        pub y: usize,
        right: usize,
        down: usize,
    }

    impl Position {
        pub fn new(right: usize, down: usize) -> Position {
            Position { x: 0, y: 0, right: right, down: down }
        }

        pub fn slide(&mut self) {
            self.x += self.right;
            self.y += self.down;
        }
    }
}

type Map = Vec<Vec<char>>;

fn read_lines_into_map(lines: Vec<String>) -> Map {
    let number_lines = lines.len();
    let line_length = lines[0].len();
    println!("Found {} lines", number_lines);
    println!("Line length = {}", line_length);

    let mut map: Map = vec![vec!['.'; line_length]; number_lines];

    print!("Read in the map");
    for (index, line) in lines.iter().enumerate() {
        print!(".");
        let current_line: Vec<char> = line.chars().collect();
        map[index] = current_line.clone();
    }
    println!(" Done!");

    return map;
}

fn toboggan(map: &Map, movement_right: usize, movement_down: usize) -> i32 {
    let mut pos = position_mod::Position::new(movement_right, movement_down);
    let width = map[0].len();
    let mut tree_count = 0;

    while pos.y < map.len() {
        if map[pos.y][pos.x] == '#' {
            tree_count += 1;
        }
        pos.slide();
        if pos.x >= width {
            // We've gone past the right boundary
            // Work out how far past we are, and reset
            pos.x = pos.x - width;
        }
    }

    return tree_count;
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let lines = advent::lines_from_file(config.filename)?;

    let map = read_lines_into_map(lines);

    let movements = vec![
        vec![1, 1],
        vec![3, 1],
        vec![5, 1],
        vec![7, 1],
        vec![1, 2],
    ];

    let mut total: u64 = 0;
    for m in movements {
        let result: u64 = toboggan(&map, m[0], m[1]).try_into().unwrap();
        if total == 0 {
            total = result;
        } else {
            total *= result;
        }
        println!("{},{} Found {} trees", m[0], m[1], result);
    }

    println!("Answer = {}", total);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use position_mod::Position;

    fn create_position() -> Position {
        return Position::new(3, 1);
    }

    #[test]
    fn test_read_lines_into_map_will_return_map() {
        let test_data = vec![
            ".#.".to_string(),
            "#.#".to_string(),
            "...".to_string(),
            "###".to_string()
        ];

        let expected = vec![
            vec!['.', '#', '.'],
            vec!['#', '.', '#'],
            vec!['.', '.', '.'],
            vec!['#', '#', '#'],
        ];
        assert_eq!(read_lines_into_map(test_data), expected);
    }

    #[test]
    fn test_position_origin_is_0_0() {
        let pos = create_position();

        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);
    }

    #[test]
    fn test_position_slide_will_move() {
        let mut pos = create_position();

        assert_eq!(pos.x, 0);
        assert_eq!(pos.y, 0);

        pos.slide();
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 1);
    }

    #[test]
    fn test_toboggan_will_count_2_trees() {
        let test_data = vec![
            ".....".to_string(),
            "#####".to_string(),
            ".....".to_string(),
            "#####".to_string()
        ];

        assert_eq!(toboggan(&read_lines_into_map(test_data), 3, 1), 2);
    }

    #[test]
    fn test_toboggan_will_count_3_trees() {
        let test_data = vec![
            ".....".to_string(),
            "#####".to_string(),
            ".....".to_string(),
            "#####".to_string(),
            ".....".to_string(),
            "#####".to_string(),
            ".....".to_string(),
        ];

        assert_eq!(toboggan(&read_lines_into_map(test_data), 3, 1), 3);
    }
}
