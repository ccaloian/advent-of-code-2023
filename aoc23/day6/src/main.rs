use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let (times, dists) = read_data("./data/input.txt");
    let num_winnings = get_num_winning_all_races_analytically(times, dists);
    println!("Day 6, Part 1: {}", num_winnings);

    let (times, dists) = read_data("./data/input.txt");
    let num_winnings = get_num_winning_big_race_analytically(times, dists);
    println!("Day 6, Part 2: {}", num_winnings);
}

fn get_num_winning_big_race_analytically(times: Vec<i64>, dists: Vec<i64>) -> i64 {
    let ts = times
        .into_iter()
        .map(|n| format!("{}", n))
        .collect::<Vec<String>>()
        .join("");
    let t = str::parse::<i64>(&ts).unwrap();

    let ds = dists
        .into_iter()
        .map(|n| format!("{}", n))
        .collect::<Vec<String>>()
        .join("");
    let d = str::parse::<i64>(&ds).unwrap();

    get_num_winning_analytically(&t, &d)
}

// fn get_num_winning_all_races(times: Vec<i64>, dists: Vec<i64>) -> i64 {
//     times
//         .iter()
//         .zip(&dists)
//         .map(|(t, d)| get_num_winning(t, d))
//         .product()
// }

fn get_num_winning_all_races_analytically(times: Vec<i64>, dists: Vec<i64>) -> i64 {
    times
        .iter()
        .zip(&dists)
        .map(|(t, d)| get_num_winning_analytically(t, d))
        .product()
}

// fn get_num_winning(t: &i64, d: &i64) -> i64 {
//     (0..=*t).filter(|i| (t - i) * i > *d).count() as i64
// }

fn get_num_winning_analytically(t: &i64, d: &i64) -> i64 {
    let x1 = -0.5 * (-t as f64 + ((t.pow(2) - 4 * d) as f64).sqrt());
    let x2 = -0.5 * (-t as f64 - ((t.pow(2) - 4 * d) as f64).sqrt());

    let mut x1_int = x1.ceil() as i64;
    if x1.floor() as i64 == x1.ceil() as i64 {
        x1_int = x1.ceil() as i64 + 1;
    }

    let mut x2_int = x2.floor() as i64;
    if x2.floor() as i64 == x2.ceil() as i64 {
        x2_int = x2.floor() as i64 - 1;
    }

    x2_int - x1_int + 1
}

/// Read input data and return a tuple of times and distances vectors.
fn read_data(filepath: &str) -> (Vec<i64>, Vec<i64>) {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut times_str = String::new();
    let _ = reader.read_line(&mut times_str).unwrap();
    let times: Vec<i64> = parse_nums(&times_str);

    let mut dists_str = String::new();
    let _ = reader.read_line(&mut dists_str).unwrap();
    let dists: Vec<i64> = parse_nums(&dists_str);

    (times, dists)
}

/// Return a vector of parsed number from string.
/// The numbers occur after ':' in `num_str`.
fn parse_nums(num_str: &str) -> Vec<i64> {
    num_str
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .map(str::parse::<i64>)
        .collect::<Result<Vec<i64>, _>>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_parse_nums() {
        assert_eq!(parse_nums("Time:      7  15   30"), vec![7, 15, 30]);
        assert_eq!(parse_nums("Distance:  9  40  200"), vec![9, 40, 200]);
    }

    #[test]
    fn part1_get_num_winning() {
        assert_eq!(get_num_winning_analytically(&7, &9), 4);
        assert_eq!(get_num_winning_analytically(&15, &40), 8);
        assert_eq!(get_num_winning_analytically(&30, &200), 9);
    }

    #[test]
    fn part1_get_num_winning_analytically() {
        assert_eq!(get_num_winning_analytically(&7, &9), 4);
        assert_eq!(get_num_winning_analytically(&15, &40), 8);
        assert_eq!(get_num_winning_analytically(&30, &200), 9);
    }

    #[test]
    fn part1_product() {
        let (times, dists) = read_data("./data/test_part1.txt");
        assert_eq!(get_num_winning_all_races_analytically(times, dists), 288);
    }
}
