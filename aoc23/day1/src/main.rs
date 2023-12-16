use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let part1_total = first_last_digit_total("./data/input.txt");
    println!("Day1, Part One: {part1_total}");

    let part2_total = first_last_spelled_total("./data/input.txt");
    println!("Day1, Part Two: {part2_total}");
    Ok(())
}

fn spelled_to_digit<'a>() -> HashMap<&'a str, &'a str> {
    HashMap::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ])
}

fn first_last_spelled_total(filepath: &str) -> i64 {
    let numbers = spelled_to_digit();
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let first = first_spelled_digit(&line, &numbers);
        let last = last_spelled_digit(&line, &numbers);
        let num = format!("{first}{last}").parse::<i64>().unwrap();
        total += num;
    }
    total
}

/// If `sub_line` slice starts with a spelled digit, return the corresponding digit as a string,
/// otherwise return None.
fn scan_first_spelled_digit(sub_line: &str, numbers: &HashMap<&str, &str>) -> Option<String> {
    if sub_line.len() >= 3 {
        if let Some(&s) = numbers.get(&sub_line[..3]) {
            return Some(s.to_string());
        }
    }
    if sub_line.len() >= 4 {
        if let Some(&s) = numbers.get(&sub_line[..4]) {
            return Some(s.to_string());
        }
    }
    if sub_line.len() >= 5 {
        if let Some(&s) = numbers.get(&sub_line[..5]) {
            return Some(s.to_string());
        }
    }
    None
}

/// If `sub_line` slice starts with a digit, return it as a string, otherwise return None.
fn scan_first_digit(sub_line: &str) -> Option<String> {
    if sub_line.chars().next().unwrap().is_ascii_digit() {
        return Some(sub_line.chars().next().unwrap().to_string());
    }
    None
}

/// Return the first occurring digit in `line`. The digit can be an ascii digit, _e.g._ '1', '2',
/// or spelled out, _e.g._ 'one', 'two'. If no digit can be parsed, return the empty string.
fn first_spelled_digit(line: &str, numbers: &HashMap<&str, &str>) -> String {
    for i in 0..line.len() {
        if let Some(d) = scan_first_digit(&line[i..]) {
            return d;
        }
        if let Some(d) = scan_first_spelled_digit(&line[i..], numbers) {
            return d;
        }
    }
    "".to_string()
}

/// Return the last occurring digit in `line`. The digit can be an ascii digit, _e.g._ '1', '2',
/// or spelled out, _e.g._ 'one', 'two'. If no digit can be parsed, return the empty string.
fn last_spelled_digit(line: &str, numbers: &HashMap<&str, &str>) -> String {
    for i in (0..line.len()).rev() {
        if let Some(d) = scan_first_digit(&line[i..]) {
            return d;
        }
        if let Some(d) = scan_first_spelled_digit(&line[i..], numbers) {
            return d;
        }
    }
    "".to_string()
}

fn first_last_digit_total(filepath: &str) -> i64 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let first = first_digit(&line);
        let last = last_digit(&line);
        let num = format!("{first}{last}").parse::<i64>().unwrap();
        total += num;
    }
    total
}

/// Return the first occurring ascii digit in `s` as a string, or empty string if no digit if found.
fn first_digit(s: &str) -> String {
    for char in s.chars() {
        if char.is_ascii_digit() {
            return char.to_string();
        }
    }
    "".to_string()
}

/// Return the last occurring ascii digit in `s` as a string, or empty string if no digit if found.
fn last_digit(s: &str) -> String {
    for char in s.chars().rev() {
        if char.is_ascii_digit() {
            return char.to_string();
        }
    }
    "".to_string()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn first_digit_exists() {
        assert_eq!(first_digit("1abc2"), "1".to_string());
        assert_eq!(first_digit("pqr3stu8vwx"), "3".to_string());
        assert_eq!(first_digit("a1b2c3d4e5f"), "1".to_string());
        assert_eq!(first_digit("treb7uchet"), "7".to_string());
    }

    #[test]
    fn last_digit_exists() {
        assert_eq!(last_digit("1abc2"), "2".to_string());
        assert_eq!(last_digit("pqr3stu8vwx"), "8".to_string());
        assert_eq!(last_digit("a1b2c3d4e5f"), "5".to_string());
        assert_eq!(last_digit("treb7uchet"), "7".to_string());
    }

    // #[test]
    // fn part1_total_sample() {
    //     assert_eq!(first_last_digit_total("./data/test_part1.txt"), 142);
    // }
    //
    // #[test]
    // fn part1_total_final() {
    //     assert_eq!(first_last_digit_total("./data/input.txt"), 53921);
    // }
    //
    // #[test]
    // fn part2_total_sample() {
    //     assert_eq!(first_last_spelled_total("./data/test_part2.txt"), 281);
    // }
    //
    // #[test]
    // fn part2_total_final() {
    //     assert_eq!(first_last_spelled_total("./data/input.txt"), 54676);
    // }

    #[test]
    fn first_spelled_digit_exists() {
        let numbers = spelled_to_digit();
        assert_eq!(first_spelled_digit("two1nine", &numbers), "2".to_string());
        assert_eq!(
            first_spelled_digit("eightwothree", &numbers),
            "8".to_string()
        );
        assert_eq!(
            first_spelled_digit("abcone2threexyz", &numbers),
            "1".to_string()
        );
        assert_eq!(
            first_spelled_digit("xtwone3four", &numbers),
            "2".to_string()
        );
        assert_eq!(
            first_spelled_digit("4nineeightseven2", &numbers),
            "4".to_string()
        );
        assert_eq!(
            first_spelled_digit("zoneight234", &numbers),
            "1".to_string()
        );
        assert_eq!(
            first_spelled_digit("7pqrstsixteen", &numbers),
            "7".to_string()
        );
        assert_eq!(first_spelled_digit("abcdefgone", &numbers), "1".to_string());
    }

    #[test]
    fn last_spelled_digit_exists() {
        let numbers = spelled_to_digit();
        assert_eq!(last_spelled_digit("two1nine", &numbers), "9".to_string());
        assert_eq!(
            last_spelled_digit("eightwothree", &numbers),
            "3".to_string()
        );
        assert_eq!(
            last_spelled_digit("abcone2threexyz", &numbers),
            "3".to_string()
        );
        assert_eq!(last_spelled_digit("xtwone3four", &numbers), "4".to_string());
        assert_eq!(
            last_spelled_digit("4nineeightseven2", &numbers),
            "2".to_string()
        );
        assert_eq!(last_spelled_digit("zoneight234", &numbers), "4".to_string());
        assert_eq!(
            last_spelled_digit("7pqrstsixteen", &numbers),
            "6".to_string()
        );
        assert_eq!(last_spelled_digit("abcdefgone", &numbers), "1".to_string());
    }
}
