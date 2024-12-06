use itertools::Itertools;
use std::env;
use std::io::{self};
use utils::{read_lines, write_day_input_to_file};

fn main() -> io::Result<()> {
    let session_cookie = env::var("AOC_SESSION_COOKIE").unwrap_or_else(|_| "0".to_string());

    // Query day 6 for input
    let input_path = write_day_input_to_file("6", session_cookie.as_str()).unwrap();
    let input = read_lines(input_path)?.collect::<Result<Vec<String>, io::Error>>()?;

    // Part 1
    println!("Part 1: {}", part_one(&input));
    // Part 2
    println!("Part 2: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &Vec<String>) -> int64 {
    let mut total_sum = 0;
    total_sum
}

fn part_two(input: &Vec<String>) -> int64 {
    let mut total_sum = 0;
    total_sum
}

const TEST_INPUT: &str = "";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_one() {
        let expected = 0;
        assert_eq!(expected, part_one(&TEST_INPUT.to_string()));
    }

    #[test]
    fn test_day_two() {
        let expected = 0;
        assert_eq!(expected, part_two(&TEST_INPUT.to_string()));
    }
}
