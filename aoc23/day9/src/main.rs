use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn main() -> Result<()> {
    let lines = read_data("./data/input.txt")?;
    let (sum_front, sum_back) = sum_predicted_values(lines);
    println!("Day 9, Part 1: {:?}", sum_back);
    println!("Day 9, Part 2: {:?}", sum_front);
    Ok(())
}

fn sum_predicted_values(lines: Vec<String>) -> (i64, i64) {
    let sum_front = lines.iter().map(|l| process_line(l).0).sum();
    let sum_back = lines.iter().map(|l| process_line(l).1).sum();
    (sum_front, sum_back)
}

fn process_line(line: &str) -> (i64, i64) {
    let nums = line
        .split_whitespace()
        .map(str::parse::<i64>)
        .collect::<std::result::Result<Vec<i64>, _>>()
        .unwrap();
    let ncol = nums.len();
    let nrow = nums.len();
    let mut mat = vec![vec![0; nrow]; ncol];

    mat[0][..ncol].copy_from_slice(&nums[..ncol]);

    let mut last_row = nrow;
    'outer: for i in 1..nrow {
        for j in i..ncol {
            mat[i][j] = mat[i - 1][j] - mat[i - 1][j - 1];
        }
        if mat[i].iter().all(|n| *n == 0) {
            last_row = i;
            break 'outer;
        }
    }

    let mut total_back = 0;
    for i in (0..=last_row).rev() {
        total_back += mat[i][ncol - 1];
    }

    let mut total_front = 0;
    for i in (1..=last_row).rev() {
        total_front = mat[i - 1][i - 1] - total_front;
    }

    (total_front, total_back)
}

fn read_data(filepath: &str) -> Result<Vec<String>> {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader.lines().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_extrapolated_values_back() {
        assert_eq!(process_line("0 3 6 9 12 15").1, 18);
        assert_eq!(process_line("1 3 6 10 15 21").1, 28);
        assert_eq!(process_line("10 13 16 21 30 45").1, 68);
    }

    #[test]
    fn sum_of_extrapolated_values_front() {
        assert_eq!(process_line("0 3 6 9 12 15").0, -3);
        assert_eq!(process_line("1 3 6 10 15 21").0, 0);
        assert_eq!(process_line("10 13 16 21 30 45").0, 5);
    }
}
