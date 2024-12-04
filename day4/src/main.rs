use std::collections::HashSet;
use std::env;
use std::io::{self};
use utils::{read_lines, write_day_input_to_file};

fn main() -> io::Result<()> {
    let session_cookie = env::var("AOC_SESSION_COOKIE").unwrap_or_else(|_| "0".to_string());

    // Query day 4 for input
    let input_path = write_day_input_to_file("4", session_cookie.as_str()).unwrap();
    let lines = read_lines(input_path)?.collect::<Result<Vec<String>, io::Error>>()?;

    // Part 1
    println!("Part 1: {}", part_one(&lines));
    // Part 2
    println!("Part 2: {}", part_two(&lines));

    Ok(())
}

fn lines_to_matrix(lines: &Vec<String>) -> Vec<Vec<char>> {
    lines
        .iter()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn part_one(input: &Vec<String>) -> i64 {
    let mut total_sum = 0;
    let matrix = lines_to_matrix(input);
    let directions: Vec<(i16, i16)> = vec![
        (0, 1),   // right
        (0, -1),  // left
        (1, 0),   // down
        (-1, 0),  // up
        (-1, 1),  // up-right
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
    ];
    let word = vec!['X', 'M', 'A', 'S'];
    for (y, line) in matrix.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'X' {
                for direction in &directions {
                    total_sum += search(&matrix, &word, x as i16, y as i16, direction);
                }
            }
        }
    }
    total_sum
}

fn part_two(input: &Vec<String>) -> i64 {
    let mut total_sum = 0;
    let matrix = lines_to_matrix(input);
    let directions: Vec<(i16, i16)> = vec![
        (-1, 1),  // up-right
        (1, 1),   // down-right
        (-1, -1), // up-left
        (1, -1),  // down-left
    ];
    // aa represents the position of the 'a' char in the forward diagonal "/"
    // ab represents the position of the 'a' char in the backward diagonal "\"
    // They are only set if the word MAS is on the diagonal line, so the opposite diagonal can check for X
    let mut aa_positions: HashSet<(usize, usize)> = HashSet::new();
    let mut ab_positions: HashSet<(usize, usize)> = HashSet::new();
    let word = vec!['M', 'A', 'S'];

    for (y, line) in matrix.iter().enumerate() {
        for (x, char) in line.iter().enumerate() {
            if *char == 'M' {
                for direction in &directions {
                    if search(&matrix, &word, x as i16, y as i16, direction) == 1 {
                        let a_pos_x = (x as i16 + direction.1) as usize;
                        let a_pos_y = (y as i16 + direction.0) as usize;
                        // The "/" diagonal
                        if direction == &(-1, 1) || direction == &(1, -1) {
                            aa_positions.insert((a_pos_x, a_pos_y));
                            if ab_positions.contains(&(a_pos_x, a_pos_y)) {
                                total_sum += 1
                            }
                        } else {
                            ab_positions.insert((a_pos_x, a_pos_y));
                            if aa_positions.contains(&(a_pos_x, a_pos_y)) {
                                total_sum += 1
                            }
                        }
                    }
                }
            }
        }
    }
    total_sum
}

fn search(input: &Vec<Vec<char>>, word: &Vec<char>, x: i16, y: i16, dir: &(i16, i16)) -> i64 {
    let (y_move, x_move) = dir;
    let x_bound = x + x_move * (word.len() as i16 - 1); // We've accounted for the first letter in x/y
    let y_bound = y + y_move * (word.len() as i16 - 1);

    // Validate bounds
    if x_bound < 0
        || x_bound >= input[0].len() as i16
        || y_bound < 0
        || y_bound >= input.len() as i16
    {
        return 0;
    }

    let mut xx = x;
    let mut yy = y;
    for c in word {
        if xx < 0 || xx >= input[0].len() as i16 || yy < 0 || yy >= input.len() as i16 {
            return 0;
        }

        if !input[yy as usize][xx as usize].eq(&c) {
            return 0;
        }
        xx += x_move;
        yy += y_move;
    }
    1
}

const TEST_INPUT: [&str; 10] = [
    "MMMSXXMASM",
    "MSAMXMSMSA",
    "AMXSXMAAMM",
    "MSAMASMSMX",
    "XMASAMXAMM",
    "XXAMMXXAMA",
    "SMSMSASXSS",
    "SAXAMASAAA",
    "MAMMMXMMMM",
    "MXMXAXMASX",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let expected = 18;
        assert_eq!(
            expected,
            part_one(&TEST_INPUT.map(|s| s.to_string()).to_vec())
        );
    }

    #[test]
    fn test_part_2() {
        let expected = 9;
        assert_eq!(
            expected,
            part_two(&TEST_INPUT.map(|s| s.to_string()).to_vec())
        );
    }
}
