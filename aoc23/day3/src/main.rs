use std::cmp::max;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // let (nums, syms) = scan_file("./data/input.txt");
    let (nums, syms) = scan_file("./data/input.txt");

    let sum = sum_part_nums(&nums, &syms);
    println!("Day 3, Part 1: {}", sum);

    let sum_gears = sum_gear_ratios(&nums, &syms);
    println!("Day 3, Part 2: {}", sum_gears);
}

fn scan_file(filepath: &str) -> (Vec<Number>, Vec<Symbol>) {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut numbers: Vec<Number> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();

    let mut deq: VecDeque<u8> = VecDeque::new();

    let mut prev = b'.';
    let mut prev_row = 0;
    let mut prev_col = 0;
    for (row, line) in reader.lines().enumerate() {
        // check if prev row ended with a number
        if !deq.is_empty() {
            let num = get_num_from_deque(&prev_row, &prev_col, &mut deq);
            numbers.push(num);
        }
        for (col, ch) in line.unwrap().as_bytes().iter().enumerate() {
            if ch.is_ascii_digit() {
                deq.push_back(*ch);
            } else if !ch.is_ascii_digit() && prev.is_ascii_digit() && !deq.is_empty() {
                let num = get_num_from_deque(&row, &col, &mut deq);
                numbers.push(num);
                if *ch != b'.' {
                    symbols.push(Symbol {
                        val: *ch as char,
                        loc: SymbolLoc { row, col },
                    });
                }
            } else if *ch != b'.' {
                symbols.push(Symbol {
                    val: *ch as char,
                    loc: SymbolLoc { row, col },
                });
            }
            prev = *ch;
            prev_col = col;
        }
        prev_row = row;
    }

    (numbers, symbols)
}

fn get_num_from_deque(row: &usize, col: &usize, deq: &mut VecDeque<u8>) -> Number {
    let loc = NumberLoc {
        row: *row,
        cols: (
            max(0, *col as i64 - deq.len() as i64) as usize,
            max(0, *col as i64 - 1) as usize,
        ), // curr col is one further than the num
    };
    let mut num_s = String::new();
    while !deq.is_empty() {
        num_s.push(deq.pop_front().unwrap() as char);
    }
    let val = num_s.parse::<u64>().unwrap();
    Number { val, loc }
}

fn sum_part_nums(numbers: &[Number], symbols: &[Symbol]) -> u64 {
    numbers
        .iter()
        .filter(|num| symbols.iter().any(|sym| num.adjacent(sym)))
        .map(|num| num.val)
        .sum()
}

fn sum_gear_ratios(numbers: &[Number], symbols: &[Symbol]) -> u64 {
    let mut total = 0;
    symbols.iter().for_each(|s| {
        if s.val == '*' {
            let part_nums = numbers
                .iter()
                .filter(|n| n.adjacent(s))
                .map(|n| n.val)
                .collect::<Vec<u64>>();
            if part_nums.len() == 2 {
                total += part_nums.get(0).unwrap() * part_nums.get(1).unwrap();
            }
        }
    });
    total
}

#[derive(Debug, PartialEq)]
struct NumberLoc {
    row: usize,
    cols: (usize, usize),
}

#[derive(Debug, PartialEq)]
struct Number {
    val: u64,
    loc: NumberLoc,
}

impl Number {
    fn adjacent(&self, sym: &Symbol) -> bool {
        let row_range = max(0, self.loc.row as i64 - 1)..=(self.loc.row as i64 + 1);
        let col_range = max(0, self.loc.cols.0 as i64 - 1)..=(self.loc.cols.1 as i64 + 1);
        row_range.contains(&(sym.loc.row as i64)) && col_range.contains(&(sym.loc.col as i64))
    }
}

#[derive(Debug, PartialEq)]
struct SymbolLoc {
    row: usize,
    col: usize,
}

#[derive(Debug, PartialEq)]
struct Symbol {
    val: char,
    loc: SymbolLoc,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan() {
        let numbers = vec![
            Number {
                val: 467,
                loc: NumberLoc {
                    row: 0,
                    cols: (0, 2),
                },
            },
            Number {
                val: 114,
                loc: NumberLoc {
                    row: 0,
                    cols: (5, 7),
                },
            },
            Number {
                val: 35,
                loc: NumberLoc {
                    row: 2,
                    cols: (2, 3),
                },
            },
            Number {
                val: 633,
                loc: NumberLoc {
                    row: 2,
                    cols: (6, 8),
                },
            },
            Number {
                val: 617,
                loc: NumberLoc {
                    row: 4,
                    cols: (0, 2),
                },
            },
            Number {
                val: 58,
                loc: NumberLoc {
                    row: 5,
                    cols: (7, 8),
                },
            },
            Number {
                val: 592,
                loc: NumberLoc {
                    row: 6,
                    cols: (2, 4),
                },
            },
            Number {
                val: 755,
                loc: NumberLoc {
                    row: 7,
                    cols: (6, 8),
                },
            },
            Number {
                val: 664,
                loc: NumberLoc {
                    row: 9,
                    cols: (1, 3),
                },
            },
            Number {
                val: 598,
                loc: NumberLoc {
                    row: 9,
                    cols: (5, 7),
                },
            },
        ];
        let symbols = vec![
            Symbol {
                val: '*',
                loc: SymbolLoc { row: 1, col: 3 },
            },
            Symbol {
                val: '#',
                loc: SymbolLoc { row: 3, col: 6 },
            },
            Symbol {
                val: '*',
                loc: SymbolLoc { row: 4, col: 3 },
            },
            Symbol {
                val: '+',
                loc: SymbolLoc { row: 5, col: 5 },
            },
            Symbol {
                val: '$',
                loc: SymbolLoc { row: 8, col: 3 },
            },
            Symbol {
                val: '*',
                loc: SymbolLoc { row: 8, col: 5 },
            },
        ];
        assert_eq!(scan_file("./data/test_part1.txt"), (numbers, symbols));
    }

    #[test]
    fn adjacency() {
        let num = Number {
            val: 592,
            loc: NumberLoc {
                row: 6,
                cols: (2, 4),
            },
        };
        let sym_1 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 5, col: 5 },
        };
        let sym_2 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 5, col: 1 },
        };
        let sym_3 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 7, col: 5 },
        };
        let sym_4 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 7, col: 1 },
        };
        let sym_5 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 6, col: 1 },
        };
        let sym_6 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 6, col: 5 },
        };
        let sym_7 = Symbol {
            val: '+',
            loc: SymbolLoc { row: 8, col: 3 },
        };
        assert!(num.adjacent(&sym_1));
        assert!(num.adjacent(&sym_2));
        assert!(num.adjacent(&sym_3));
        assert!(num.adjacent(&sym_4));
        assert!(num.adjacent(&sym_5));
        assert!(num.adjacent(&sym_6));
        assert!(!num.adjacent(&sym_7));
    }

    #[test]
    fn part1_total_sample() {
        let (nums, syms) = scan_file("./data/test_part1.txt");
        let sum = sum_part_nums(&nums, &syms);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn part1_total_sample_modified() {
        let (nums, syms) = scan_file("./data/test_part1.txt");
        let sum = sum_part_nums(&nums, &syms);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn part1_total_final() {
        let (nums, syms) = scan_file("./data/input.txt");
        let sum = sum_part_nums(&nums, &syms);
        assert_eq!(sum, 532428);
    }

    #[test]
    fn part2_total_sample() {
        let (nums, syms) = scan_file("./data/test_part1.txt");
        let sum = sum_gear_ratios(&nums, &syms);
        assert_eq!(sum, 467835);
    }

    #[test]
    fn part2_total_sample_modified() {
        let (nums, syms) = scan_file("./data/test_part1.txt");
        let sum = sum_gear_ratios(&nums, &syms);
        assert_eq!(sum, 467835);
    }

    #[test]
    fn part2_total_final() {
        let (nums, syms) = scan_file("./data/input.txt");
        let sum = sum_gear_ratios(&nums, &syms);
        assert_eq!(sum, 84051670);
    }
}
