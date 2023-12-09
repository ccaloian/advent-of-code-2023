use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let (dirs, nav) = read_data("./data/input.txt");
    println!("Num directions: {:?}", dirs.len());
    let steps = navigate_steps_simultaneously(nav, dirs, |s: &str| s.ends_with('Z'));
    println!("Day 8, Part 2: {:?}", steps);
}

type Navigator = HashMap<String, (String, String)>;
type Node = (String, (String, String));

fn navigate_steps_simultaneously(
    navigator: Navigator,
    directions: Vec<u8>,
    dest: fn(&str) -> bool,
) -> u64 {
    let start_nodes = navigator
        .keys()
        .filter(|k| k.ends_with('A'))
        .collect::<Vec<&String>>();

    let num_z_final = start_nodes.len();

    let mut steps_each: Vec<u64> = vec![0; num_z_final];

    for (i, node) in start_nodes.iter().enumerate() {
        let steps = navigate_steps(&navigator, &directions, node, dest);
        steps_each[i] = steps as u64;
    }

    lcmm(&steps_each)
}

fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcmm(nums: &[u64]) -> u64 {
    nums.iter().fold(1, |a, b| lcm(a, *b))
}

fn navigate_steps(
    navigator: &Navigator,
    directions: &Vec<u8>,
    start: &str,
    dest: fn(&str) -> bool,
) -> usize {
    let mut total = 0;
    let mut curr_loc = start;
    for direction in directions.iter().cycle() {
        if direction == &b'L' {
            curr_loc = &navigator.get(curr_loc).unwrap().0;
        } else {
            curr_loc = &navigator.get(curr_loc).unwrap().1;
        }
        // println!("{:?} {:?}", *direction as char, curr_loc);
        total += 1;
        if dest(curr_loc) {
            break;
        }
    }

    total
}

fn read_data(filepath: &str) -> (Vec<u8>, Navigator) {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut directions_str = String::new();
    let _ = reader.read_line(&mut directions_str);
    let directions: Vec<u8> = directions_str.trim().to_string().into_bytes();

    let mut navigator: Navigator = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let node = process_line(&line);
        navigator.insert(node.0, node.1);
    }
    (directions, navigator)
}

fn process_line(line: &str) -> Node {
    let (loc, dirs) = line.split_once('=').unwrap();
    let (left, right) = dirs.split_once(',').unwrap();

    (
        loc.trim().to_string(),
        (
            left.trim().trim_start_matches('(').to_string(),
            right.trim().trim_end_matches(')').to_string(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_line_works() {
        assert_eq!(
            process_line("AAA = (BBB, CCC)"),
            ("AAA".to_string(), ("BBB".to_string(), "CCC".to_string()))
        );
    }

    #[test]
    fn lcm_works() {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(12, 4), 12);
        assert_eq!(lcm(3, 7), 21);
    }

    #[test]
    fn lcmm_works() {
        assert_eq!(lcmm(&[3, 4]), 12);
        assert_eq!(lcmm(&[3, 4, 6, 8]), 24);
        assert_eq!(lcmm(&[12, 18, 3, 4, 9, 6]), 36);
    }
}
