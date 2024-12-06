use std::collections::{BinaryHeap, HashMap, HashSet};
use std::env;
use std::io::{self};
use utils::{read_lines, write_day_input_to_file};

fn main() -> io::Result<()> {
    let session_cookie = env::var("AOC_SESSION_COOKIE").unwrap_or_else(|_| "0".to_string());

    // Query day 1 for input
    let input_path = write_day_input_to_file("1", session_cookie.as_str()).unwrap();
    let lines = read_lines(input_path)?.collect::<Result<Vec<String>, io::Error>>()?;

    // Part 1
    // Create BinaryHeaps for left and right number - implicitly sorted
    let mut left_side = BinaryHeap::new();
    let mut right_side = BinaryHeap::new();

    for line in &lines {
        let mut split = line.split_whitespace();

        left_side.push(split.next().unwrap().parse::<u32>().unwrap());
        right_side.push(split.next().unwrap().parse::<u32>().unwrap());
    }

    // Calculate sum of all differences in Part 1
    let mut sum = 0;
    while !left_side.is_empty() && !right_side.is_empty() {
        let left = left_side.pop().unwrap();
        let right = right_side.pop().unwrap();

        sum += left.abs_diff(right);
    }

    println!("Part 1 Sum: {}", sum);

    // Part 2
    // Create a hashset for all unique numbers on left
    // And hashset for counting occurrences on right side
    let mut exists_left = HashSet::new();
    let mut count_dict = HashMap::new();
    for line in lines {
        let mut split = line.split_whitespace();
        let left = split.next().unwrap().parse::<u32>().unwrap();
        let right = split.next().unwrap().parse::<u32>().unwrap();

        // Insert left side, and increase right side by 1, if it doesn't exist give it a default value of 0
        exists_left.insert(left);
        *count_dict.entry(right).or_insert(0) += 1;
    }

    // Go through all existing numbers on left, multiply them with occurrences on right
    let mut part_2_sum = 0;
    for exist in exists_left.iter() {
        match count_dict.get(exist) {
            Some(count) => part_2_sum += exist * count,
            None => {}
        }
    }
    println!("Part 2 Sum: {}", part_2_sum);

    Ok(())
}
