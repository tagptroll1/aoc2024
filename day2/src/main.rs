use itertools::Itertools;
use std::env;
use std::io::{self};
use utils::{read_lines, write_day_input_to_file};

fn main() -> io::Result<()> {
    let session_cookie = env::var("AOC_SESSION_COOKIE").unwrap_or_else(|_| "0".to_string());

    // Query day 2 for input
    let input_path = write_day_input_to_file("2", session_cookie.as_str()).unwrap();
    let lines = read_lines(input_path)?.collect::<Result<Vec<String>, io::Error>>()?;

    let test_lines = vec![
        "7 6 4 2 1",
        "1 2 7 8 9",
        "9 7 6 2 1",
        "1 3 2 4 5",
        "8 6 4 4 1",
        "1 3 6 7 9",
    ];

    // Part 1
    let mut count = 0;
    for line in lines.iter() {
        // Split "1 2 3" to [1, 2, 3]
        let numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let safe = is_entry_safe(numbers);

        if safe {
            count += 1;
            println!("safe:   {}", line);
        } else {
            println!("unsafe: {}", line);
        }
    }

    println!("Day 1 safe count: {}", count);
    // Day 2
    // The program is allowed to ignore one unsafe entry.
    // It should do so by simply removing it, and retrying the sequences
    let mut count_day2 = 0;
    for line in lines.iter() {
        let mut numbers = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let mut safe = is_entry_safe(numbers.clone());

        if safe {
            count_day2 = count_day2 + 1;
            println!("safe:   {}", line);
        } else {
            safe = try_every_damn_number(numbers.clone());
            if safe {
                count_day2 = count_day2 + 1;
            } else {
                println!("Unsafe: {}", line);
            }
        }
    }

    println!("Day 2 safe count: {}", count_day2);
    Ok(())
}

fn try_every_damn_number(numbers: Vec<i32>) -> bool {
    for index in 0..numbers.len() {
        let mut new_numbers = numbers.clone();
        new_numbers.remove(index);
        if is_entry_safe(new_numbers.clone()) {
            let line = new_numbers.iter().map(|n| n.to_string()).join(" ");
            println!("safe:   {}", line);
            return true;
        }
    }
    false
}
// is_entry_safe will return true if the entry is safe, and -1 as the second return value
// if there is an unsafe number in the entry the index of said number will be set to the second return value
fn is_entry_safe(line: Vec<i32>) -> bool {
    // Check the numbers in order if
    // ascending order is expected, check for ascending order
    // descending order is expected, check for descending order
    // any sequence of two equal numbers is unsafe
    // and jump between numbers besides 1-3 is unsafe
    let step_size = is_step_diff_small(&line);
    let is_asc = is_ascending(&line);
    let is_desc = is_descending(&line);

    step_size && (is_asc || is_desc)
}

fn is_step_diff_small(line: &Vec<i32>) -> bool {
    for (i, number) in line.iter().enumerate() {
        if i == line.len() - 1 {
            break;
        }
        let next = line[i + 1];
        let diff = number.abs_diff(next);
        if diff < 1 || diff > 3 {
            return false;
        }
    }
    true
}
fn is_ascending(line: &Vec<i32>) -> bool {
    for (i, number) in line.iter().enumerate() {
        if i == line.len() - 1 {
            return true;
        }
        let next = line[i + 1];
        if next < *number {
            return false;
        }
    }
    true
}

fn is_descending(line: &Vec<i32>) -> bool {
    for (i, number) in line.iter().enumerate() {
        if i == line.len() - 1 {
            return true;
        }
        let next = line[i + 1];
        if next > *number {
            return false;
        }
    }
    true
}
