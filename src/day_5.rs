use std::error::Error;
use std::fmt;

use advent::Config;

#[derive(Debug)]
struct Seat {
    row: i32,
    column: i32,
}

// Implement '==' for Seat class
impl PartialEq for Seat {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.column == other.column
    }
}
impl Eq for Seat {}

// Implement display writter for Seat class
impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Seat: row={} column={}", self.row, self.column)
    }
}


#[derive(Debug)]
struct BinarySearch {
    lower: i32,
    upper: i32,
}


fn generate_seat_id(seat: &Seat) -> i32 {
    return seat.row * 8 + seat.column;
}

fn find_seat(pass: &str) -> Seat {
    let row_def = &pass[..7];
    let mut current = BinarySearch { lower: 0, upper: 127 };
    for c in row_def.chars() {
        let middle = (current.lower + current.upper) / 2;
        if c == 'F' {
            current.upper = middle;
        } else {
            current.lower = middle + 1;
        }
    }
    let row = current.lower;


    let column_def = &pass[7..];
    current.lower = 0;
    current.upper = 7;
    for c in column_def.chars() {
        let middle = (current.lower + current.upper) / 2;
        if c == 'L' {
            current.upper = middle;
        } else {
            current.lower = middle + 1;
        }
    }
    let column = current.lower;

    return Seat{ row: row, column: column };
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let passes = advent::lines_from_file(config.filename)?;

    let mut seats: Vec<i32> = vec![];

    for pass in passes {
        let seat = find_seat(&pass);
        seats.push(generate_seat_id(&seat));
    }

    // Find the missing seat. Sort into natural order first.
    seats.sort();
    let mut i = 1;
    while i < seats.len() {
        if seats[i] != seats[i-1] + 1 {
            break;
        }
        i += 1;
    }

    println!("The highest seat ID is {}.", seats.last().unwrap());
    println!("My seat ID is {}.", seats[i-1] + 1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_seat_will_return_seat() {
        let test_data = vec![
            ("FBFBBFFRLR", Seat{ row: 44, column: 5 }),
            ("BFFFBBFRRR", Seat{ row: 70, column: 7 }),
            ("FFFBBBFRRR", Seat{ row: 14, column: 7 }),
            ("BBFFBBFRLL", Seat{ row: 102, column: 4 }),
        ];
        for test in test_data {
            assert_eq!(find_seat(test.0), test.1);
        }
    }

    #[test]
    fn test_generate_seat_id_will_return_id() {
        let test_data = vec![
            (Seat{ row: 44, column: 5 }, 357),
            (Seat{ row: 70, column: 7 }, 567),
            (Seat{ row: 14, column: 7 }, 119),
            (Seat{ row: 102, column: 4 }, 820),
        ];
        for test in test_data {
            assert_eq!(generate_seat_id(&test.0), test.1);
        }
    }
}
