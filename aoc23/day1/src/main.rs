use std::collections::HashMap;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let part1_total = first_last_digit_total("./data/input.txt");
    println!("Day1, Part One: {part1_total}"); // Answer part 1: 53921

    let part2_total = first_last_spelled_total("./data/input.txt");
    println!("Day1, Part Two: {part2_total}"); // Answer part 2: 54676
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
        let line= line.unwrap();
        let first = first_spelled_digit(&line, &numbers);
        let last = last_spelled_digit(&line, &numbers);
        let num = format!("{first}{last}").parse::<i64>().unwrap();
        total += num;
    }
    total
}

fn first_spelled_digit(line: &str, numbers: &HashMap<&str, &str>) -> String {
    for i in 0..line.len() {
        if line.chars().nth(i).unwrap().is_ascii_digit() {
            return line.chars().nth(i).unwrap().to_string();
        }
        if i + 3 <= line.len() {
            if let Some(&s) = numbers.get(&line[i..i+3]) { return s.to_string() }
        }
        if i + 4 <= line.len() {
            if let Some(&s) = numbers.get(&line[i..i+4]) { return s.to_string() }
        }
        if i + 5 <= line.len() {
            if let Some(&s) = numbers.get(&line[i..i+5]) { return s.to_string() }
        }
    }
    "".to_string()
}

fn last_spelled_digit(line: &str, numbers: &HashMap<&str, &str>) -> String {
    for i in (0..line.len()).rev() {
        if line.chars().nth(i).unwrap().is_ascii_digit() {
            return line.chars().nth(i).unwrap().to_string();
        }
        if i + 3 <= line.len() {
            if let Some(&s) = numbers.get(&line[i..i+3]) { return s.to_string() }
        }
        if i + 4 <= line.len() {
            if let Some(&s) = numbers.get(&line[i..i+4]) { return s.to_string() }
        }
        if i + 5 <= line.len() {
            if let Some(&s) = numbers.get(&line[i..i+5]) { return s.to_string() }
        }
    }
    "".to_string()
}

fn first_last_digit_total(filepath: &str) -> i64 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    let mut total = 0;
    for line in reader.lines() {
        let line= line.unwrap();
        let first = first_digit(&line);
        let last = last_digit(&line);
        let num = format!("{first}{last}").parse::<i64>().unwrap();
        total += num;
    }
    total
}

fn first_digit(s: &str) -> String {
    for char in s.chars() {
        if char.is_ascii_digit() {
            return char.to_string();
        }
    }
    "".to_string()
}

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

    #[test]
    fn part1_total() {
        assert_eq!(first_last_digit_total("./data/test_part1.txt"), 142);
    }

    #[test]
    fn part2_total() {
        assert_eq!(first_last_spelled_total("./data/test_part2.txt"), 281);
    }

    #[test]
    fn first_spelled_digit_exists() {
        let numbers = spelled_to_digit();
        assert_eq!(first_spelled_digit("two1nine", &numbers), "2".to_string());
        assert_eq!(first_spelled_digit("eightwothree", &numbers), "8".to_string());
        assert_eq!(first_spelled_digit("abcone2threexyz", &numbers), "1".to_string());
        assert_eq!(first_spelled_digit("xtwone3four", &numbers), "2".to_string());
        assert_eq!(first_spelled_digit("4nineeightseven2", &numbers), "4".to_string());
        assert_eq!(first_spelled_digit("zoneight234", &numbers), "1".to_string());
        assert_eq!(first_spelled_digit("7pqrstsixteen", &numbers), "7".to_string());
        assert_eq!(first_spelled_digit("abcdefgone", &numbers), "1".to_string());
    }

    #[test]
    fn last_spelled_digit_exists() {
        let numbers = spelled_to_digit();
        assert_eq!(last_spelled_digit("two1nine", &numbers), "9".to_string());
        assert_eq!(last_spelled_digit("eightwothree", &numbers), "3".to_string());
        assert_eq!(last_spelled_digit("abcone2threexyz", &numbers), "3".to_string());
        assert_eq!(last_spelled_digit("xtwone3four", &numbers), "4".to_string());
        assert_eq!(last_spelled_digit("4nineeightseven2", &numbers), "2".to_string());
        assert_eq!(last_spelled_digit("zoneight234", &numbers), "4".to_string());
        assert_eq!(last_spelled_digit("7pqrstsixteen", &numbers), "6".to_string());
        assert_eq!(last_spelled_digit("abcdefgone", &numbers), "1".to_string());
    }
}