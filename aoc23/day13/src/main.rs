use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::vec;

fn main() {
    let patterns = read_data("./data/input.txt");
    let total_part_1 = part_1(&patterns);
    println!("Day 13, Part 1: {:?}", total_part_1);

    let total_part_2 = part_2(&patterns);
    println!("Day 13, Part 2: {:?}", total_part_2);
}

struct Matrix {
    data: Vec<Vec<u8>>,
}

impl Matrix {
    fn new() -> Matrix {
        Matrix { data: vec![] }
    }

    fn from_str(s: &str) -> Self {
        Matrix {
            data: s.split('\n').map(|line| line.as_bytes().to_vec()).collect(),
        }
    }

    fn add(&mut self, elem: Vec<u8>) {
        self.data.push(elem);
    }
}

impl FromIterator<Vec<u8>> for Matrix {
    fn from_iter<I: IntoIterator<Item = Vec<u8>>>(iter: I) -> Self {
        let mut c = Matrix::new();

        for i in iter {
            c.add(i);
        }
        c
    }
}

impl Matrix {
    fn dim(&self) -> (usize, usize) {
        if self.data.is_empty() {
            (self.data.len(), 0)
        } else {
            (self.data.len(), self.data[0].len())
        }
    }

    fn nrow(&self) -> usize {
        self.dim().0
    }

    fn ncol(&self) -> usize {
        self.dim().1
    }

    /// Return true if row i and row j are equal.
    fn equal_rows(&self, i: usize, j: usize) -> bool {
        self.data[i]
            .iter()
            .zip(self.data[j].iter().as_slice())
            .all(|(left, right)| *left == *right)
    }

    fn equal_cols(&self, i: usize, j: usize) -> bool {
        self.data
            .iter()
            .map(|row| (row[i], row[j]))
            .all(|(left, right)| left == right)
    }

    fn candidate_horizontal_reflections(&self) -> Vec<usize> {
        (0..self.nrow() - 1)
            .filter(|i| self.equal_rows(*i, *i + 1))
            .map(|i| i + 1)
            .collect()
    }

    fn perfect_horizontal_reflection(&self) -> Option<usize> {
        let candidates = self.candidate_horizontal_reflections();
        candidates.into_iter().find(|i| {
            (0..*i)
                .rev()
                .zip(*i..self.nrow())
                .all(|(i, j)| self.equal_rows(i, j))
        })
    }

    fn candidate_vertical_reflections(&self) -> Vec<usize> {
        (0..self.ncol() - 1)
            .filter(|i| self.equal_cols(*i, *i + 1))
            .map(|i| i + 1)
            .collect()
    }

    fn perfect_vertical_reflection(&self) -> Option<usize> {
        let candidates = self.candidate_vertical_reflections();
        candidates.into_iter().find(|i| {
            (0..*i)
                .rev()
                .zip(*i..self.ncol())
                .all(|(i, j)| self.equal_cols(i, j))
        })
    }

    /// Same as `equal_rows()` but allow for one smudge. Return a tuple with
    /// first element a bool indicating if rows are equal, and the second
    /// element is the length of mismatches. For `true` only `(true, 0)` and
    /// `(true, 1)` can happen.
    fn equal_rows_with_smudge(&self, i: usize, j: usize) -> (bool, usize) {
        let matches = self.data[i]
            .iter()
            .zip(self.data[j].iter().as_slice())
            .filter(|(left, right)| *left == *right)
            .count();
        (matches >= self.ncol() - 1, self.ncol() - matches)
    }

    /// Same as `equal_cols()` but allow for one smudge.  Return a tuple with
    //     /// first element a bool indicating if cols are equal, and the second
    //     /// element is the length of mismatches. For `true` only `(true, 0)` and
    //     /// `(true, 1)` can happen.
    fn equal_cols_with_smudge(&self, i: usize, j: usize) -> (bool, usize) {
        let matches = self
            .data
            .iter()
            .map(|row| (row[i], row[j]))
            .filter(|(left, right)| left == right)
            .count();
        (matches >= self.nrow() - 1, self.nrow() - matches)
    }

    fn candidate_horizontal_reflections_with_smudge(&self) -> Vec<usize> {
        (0..self.nrow() - 1)
            .filter(|i| self.equal_rows_with_smudge(*i, *i + 1).0)
            .map(|i| i + 1)
            .collect()
    }

    fn candidate_vertical_reflections_with_smudge(&self) -> Vec<usize> {
        (0..self.ncol() - 1)
            .filter(|i| self.equal_cols_with_smudge(*i, *i + 1).0)
            .map(|i| i + 1)
            .collect()
    }

    fn perfect_horizontal_reflection_with_smudge(&self) -> Option<usize> {
        let candidates = self.candidate_horizontal_reflections_with_smudge();
        let diffs = candidates.iter().map(|i| {
            (0..*i)
                .rev()
                .zip(*i..self.nrow())
                .map(|(i, j)| self.equal_rows_with_smudge(i, j).1)
                .sum()
        });
        let ix: Option<usize> = diffs.into_iter().position(|d: usize| d == 1);
        ix.map(|i| candidates[i])
    }

    fn perfect_vertical_reflection_with_smudge(&self) -> Option<usize> {
        let candidates = self.candidate_vertical_reflections_with_smudge();
        let diffs = candidates.iter().map(|i| {
            (0..*i)
                .rev()
                .zip(*i..self.ncol())
                .map(|(i, j)| self.equal_cols_with_smudge(i, j).1)
                .sum()
        });
        let ix: Option<usize> = diffs.into_iter().position(|d: usize| d == 1);
        ix.map(|i| candidates[i])
    }
}

fn part_1(patterns: &[Matrix]) -> u32 {
    let mut total = 0;
    for (n, pat) in patterns.iter().enumerate() {
        if let Some(i) = pat.perfect_horizontal_reflection() {
            total += 100 * i as u32;
            continue;
        }
        if let Some(j) = pat.perfect_vertical_reflection() {
            total += j as u32;
        }
    }
    total
}

fn part_2(patterns: &[Matrix]) -> u32 {
    let mut total = 0;
    for (n, pat) in patterns.iter().enumerate() {
        if let Some(i) = pat.perfect_horizontal_reflection_with_smudge() {
            total += 100 * i as u32;
            continue;
        }
        if let Some(j) = pat.perfect_vertical_reflection_with_smudge() {
            total += j as u32;
        }
    }
    total
}

fn read_data(filename: &str) -> Vec<Matrix> {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut content = String::new();
    let _ = reader.read_to_string(&mut content).unwrap();

    content
        .split("\n\n")
        .map(Matrix::from_str)
        .collect::<Vec<Matrix>>()

    // content
    //     .split("\n\n")
    //     .map(|pat| {
    //         pat.split('\n')
    //             .map(|line| line.as_bytes().to_vec())
    //             .collect::<Matrix>()
    //     })
    //     .collect::<Vec<Matrix>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const MAT_STR_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";

    const MAT_STR_2: &str = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";

    #[test]
    fn matrix_cols_equal() {
        let m = Matrix::from_str(MAT_STR_1);
        assert!(m.equal_cols(4, 5));
        assert!(m.equal_cols(3, 6));
        assert!(m.equal_cols(2, 7));
        assert!(m.equal_cols(1, 8));
        assert!(!m.equal_cols(0, 1));
    }

    #[test]
    fn matrix_rows_equal() {
        let m = Matrix::from_str(MAT_STR_2);
        assert!(m.equal_rows(3, 4));
        assert!(m.equal_rows(2, 5));
        assert!(m.equal_rows(1, 6));
        assert!(!m.equal_rows(0, 1));
    }

    #[test]
    fn matrix_horizontal_reflections() {
        let m1 = Matrix::from_str(MAT_STR_1);
        let m2 = Matrix::from_str(MAT_STR_2);
        assert_eq!(m1.candidate_horizontal_reflections(), vec![3]);
        assert_eq!(m2.candidate_horizontal_reflections(), vec![4]);
    }

    #[test]
    fn matrix_vertical_reflections() {
        let m1 = Matrix::from_str(MAT_STR_1);
        let m2 = Matrix::from_str(MAT_STR_2);
        assert_eq!(m1.candidate_vertical_reflections(), vec![5]);
        assert_eq!(m2.candidate_vertical_reflections(), vec![3, 7]);
    }

    #[test]
    fn matrix_perfect_horizontal_reflections() {
        let m1 = Matrix::from_str(MAT_STR_1);
        let m2 = Matrix::from_str(MAT_STR_2);
        assert_eq!(m1.perfect_horizontal_reflection(), None);
        assert_eq!(m2.perfect_horizontal_reflection(), Some(4));
    }

    #[test]
    fn matrix_perfect_vertical_reflections() {
        let m1 = Matrix::from_str(MAT_STR_1);
        let m2 = Matrix::from_str(MAT_STR_2);
        assert_eq!(m1.perfect_vertical_reflection(), Some(5));
        assert_eq!(m2.perfect_vertical_reflection(), None);
    }
}
