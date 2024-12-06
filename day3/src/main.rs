use regex::Regex;
use std::env;
use std::io::{self};
use utils::{read_lines, write_day_input_to_file};

fn main() -> io::Result<()> {
    let session_cookie = env::var("AOC_SESSION_COOKIE").unwrap_or_else(|_| "0".to_string());

    // Query day 3 for input
    let input_path = write_day_input_to_file("3", session_cookie.as_str()).unwrap();
    let lines = read_lines(input_path)?.collect::<Result<Vec<String>, io::Error>>()?;

    // Today the input we can treat the input as just one line.
    let line = lines.join("");

    // Part 1
    println!("Part 1: {}", part_one(&line));
    println!("Part 2: {}", part_two(&line));

    Ok(())
}

fn part_one(input: &str) -> i64 {
    let re_match_mul = r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)";
    let re = Regex::new(re_match_mul).unwrap();
    let mut total_sum = 0i64;

    for cap in re.captures_iter(input) {
        let first = cap.name("first").unwrap().as_str().parse::<i64>().unwrap();
        let second = cap.name("second").unwrap().as_str().parse::<i64>().unwrap();
        let sum = first * second;
        total_sum += sum;
    }
    total_sum
}

fn part_two(input: &str) -> i64 {
    let re_match_mul = r"mul\((?P<first>\d{1,3}),(?P<second>\d{1,3})\)";
    let re_match_dont = r"(?P<false>don't)";
    let re_match_do = r"(?P<true>do)";
    // Make sure to match don't first as do will always be true on a don't and consume the do
    let regex_string = format!("{}|{}|{}", re_match_mul, re_match_dont, re_match_do);
    let re = Regex::new(regex_string.as_str()).unwrap();
    let mut total_sum = 0i64;
    let mut do_bool = true;

    for cap in re.captures_iter(input) {
        if cap.name("first").is_some() {
            let first = cap.name("first").unwrap().as_str().parse::<i64>().unwrap();
            let second = cap.name("second").unwrap().as_str().parse::<i64>().unwrap();
            if do_bool {
                total_sum += first * second;
            }
        }

        if cap.name("false").is_some() {
            do_bool = false;
        } else if cap.name("true").is_some() {
            do_bool = true;
        }
    }
    total_sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part_1() {
        let test_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(161, part_one(test_input));
    }

    #[test]
    fn test_part_2() {
        let test_input =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(48, part_two(test_input));
    }
}
