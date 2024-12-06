use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;
use std::io::{self};
use utils::write_day_input_to_file;

fn main() -> io::Result<()> {
    let session_cookie = env::var("AOC_SESSION_COOKIE").unwrap_or_else(|_| "0".to_string());

    // Query day 5 for input
    let input_path = write_day_input_to_file("5", session_cookie.as_str()).unwrap();
    let text = read_to_string(input_path)?;

    // Part 1
    println!("Part 1: {}", part_one(&text));
    // Part 2
    println!("Part 2: {}", part_two(&text));

    Ok(())
}

fn part_one(input: &String) -> i64 {
    let mut total_sum = 0;
    let (rules, page_set) = split_data_to_rules(input);

    for page in page_set {
        let parts: Vec<&str> = page.split(",").collect();
        if is_following_the_rules(&rules, parts.clone()) {
            total_sum += middle_number(parts);
        }
    }
    total_sum
}

fn part_two(input: &String) -> i64 {
    let mut total_sum = 0;
    let (rules, page_set) = split_data_to_rules(input);

    for page in page_set {
        let mut parts: Vec<&str> = page.split(",").collect();
        if is_following_the_rules(&rules, parts.clone()) {
            continue; // Only care about the offenders
        }

        loop {
            let (new_parts, valid) = swap_and_validate_once(&rules, parts.clone());
            parts = new_parts;
            if valid {
                let middle = middle_number(parts.clone());
                total_sum += middle;
                break;
            }
        }
    }
    total_sum
}

fn swap_and_validate_once<'a>(
    rules: &HashMap<&str, HashSet<&str>>,
    parts: Vec<&'a str>,
) -> (Vec<&'a str>, bool) {
    if is_following_the_rules(&rules, parts.clone()) {
        return (parts, true);
    }

    for i in 0..parts.len() {
        let rule_set = rules.get(&parts[i]);
        if rule_set.is_none() {
            continue;
        }

        for &rule in rule_set.unwrap() {
            if parts[0..i].contains(&rule) {
                let mut clone = parts.clone();
                let j = parts.iter().position(|&x| x == rule).unwrap();
                clone.swap(i, j);
                return (clone, false);
            }
        }
    }

    (parts, true)
}

fn split_data_to_rules(input: &String) -> (HashMap<&str, HashSet<&str>>, Vec<&str>) {
    let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();

    let mut parts = input.split("\n\n");
    let rule_part = parts.next().unwrap_or("");
    let page_part = parts.next().unwrap_or("");

    for line in rule_part.lines() {
        if let Some((left, right)) = line.split_once("|") {
            rules.entry(left).or_insert_with(HashSet::new).insert(right);
        }
    }

    let pages: Vec<&str> = page_part.lines().collect();

    (rules, pages)
}

fn is_following_the_rules(rules: &HashMap<&str, HashSet<&str>>, parts: Vec<&str>) -> bool {
    for (i, &part) in parts.iter().enumerate() {
        if let Some(rule) = rules.get(part) {
            for j in 0..i {
                if rule.contains(parts[j]) {
                    return false;
                }
            }
        }
    }

    true
}

fn middle_number(line: Vec<&str>) -> i64 {
    let index = line.len() / 2;
    line[index].parse::<i64>().unwrap_or_else(|_| 0)
}

const TEST_INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_following_the_rules() {
        let mut rules: HashMap<&str, HashSet<&str>> = HashMap::new();
        rules.insert("61", ["29", "13", "53", "29"].iter().cloned().collect());
        rules.insert(
            "47",
            ["13", "61", "29", "53", "13", "61", "29"]
                .iter()
                .cloned()
                .collect(),
        );
        rules.insert(
            "75",
            ["47", "61", "13", "29", "53", "47", "61", "13"]
                .iter()
                .cloned()
                .collect(),
        );
        rules.insert(
            "97",
            ["75", "13", "61", "47", "29", "53", "75"]
                .iter()
                .cloned()
                .collect(),
        );
        rules.insert("53", ["13", "29", "13"].iter().cloned().collect());
        rules.insert("29", ["13"].iter().cloned().collect());

        assert!(is_following_the_rules(
            &rules,
            "75,47,61,53,29".split(",").collect()
        ));
        assert!(is_following_the_rules(
            &rules,
            "97,61,53,29,13".split(",").collect()
        ));
        assert!(is_following_the_rules(
            &rules,
            "75,29,13".split(",").collect()
        ));
        assert!(!is_following_the_rules(
            &rules,
            "75,97,47,61,53".split(",").collect()
        ));
        assert!(!is_following_the_rules(
            &rules,
            "61,13,29".split(",").collect()
        ));
        assert!(!is_following_the_rules(
            &rules,
            "97,13,75,29,47".split(",").collect()
        ));
    }
    #[test]
    fn test_part_1() {
        let expected = 143;
        assert_eq!(expected, part_one(&TEST_INPUT.to_string()));
    }

    #[test]
    fn test_part_2() {
        let expected = 123;
        assert_eq!(expected, part_two(&TEST_INPUT.to_string()));
    }
}
