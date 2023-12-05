use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let cards = scan("./data/input.txt");
    let sum = sum_winning_points(&cards);
    println!("Day 4, Part 1: {}", sum);

    let total = total_scratchcards(&cards);
    println!("Day 4, Part 2: {}", total);
}

fn total_scratchcards(cards: &[Card]) -> u64 {
    let mut card_copies: Vec<u64> = vec![1; cards.len()];
    let mut total = 0;
    for (i, curr_card) in cards.iter().enumerate() {
        for j in (i + 1)..(i + curr_card.matching as usize + 1) {
            card_copies[j] += card_copies[i];
        }
        total += card_copies[i];
    }
    total
}

fn sum_winning_points(cards: &[Card]) -> u64 {
    cards.iter().map(|c| c.points()).sum()
}

fn scan(filepath: &str) -> Vec<Card> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| Card::from_str(l.unwrap().as_str()))
        .collect::<Vec<Card>>()
}

#[derive(Debug, PartialEq)]
struct Card {
    id: u64,
    win: HashSet<u64>,
    own: HashSet<u64>,
    matching: u64,
}

impl Card {
    fn from_str(s: &str) -> Self {
        let (card_s, numbers) = s.split_once(':').unwrap();
        let id = card_s
            .split_whitespace()
            .collect::<Vec<&str>>()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let (win_s, own_s) = numbers.split_once('|').unwrap();
        let win = str_nums_to_set(win_s);
        let own = str_nums_to_set(own_s);
        let matching: usize = win.intersection(&own).count();
        Card {
            id,
            win,
            own,
            matching: matching as u64,
        }
    }

    fn points(&self) -> u64 {
        let nums = self.win.intersection(&self.own).count() as u32;
        match nums {
            1 => 1,
            2.. => (2u64).pow(nums - 1),
            _ => 0,
        }
    }
}

fn str_nums_to_set(s: &str) -> HashSet<u64> {
    s.split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .collect::<HashSet<u64>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nums_from_str() {
        assert_eq!(
            str_nums_to_set("   1 48 83 86 17 "),
            HashSet::from_iter(vec![1, 48, 83, 86, 17])
        );
    }

    #[test]
    fn parse_card_from_str() {
        assert_eq!(
            Card::from_str("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"),
            Card {
                id: 1,
                win: HashSet::from_iter(vec![41, 48, 83, 86, 17]),
                own: HashSet::from_iter(vec![83, 86, 6, 31, 17, 9, 48, 53]),
                matching: 4,
            }
        );
        assert_eq!(
            Card::from_str("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19"),
            Card {
                id: 2,
                win: HashSet::from_iter(vec![13, 32, 20, 16, 61]),
                own: HashSet::from_iter(vec![61, 30, 68, 82, 17, 32, 24, 19]),
                matching: 2,
            }
        );
        assert_eq!(
            Card::from_str("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1"),
            Card {
                id: 3,
                win: HashSet::from_iter(vec![1, 21, 53, 59, 44]),
                own: HashSet::from_iter(vec![69, 82, 63, 72, 16, 21, 14, 1]),
                matching: 2,
            }
        );
        assert_eq!(
            Card::from_str("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83"),
            Card {
                id: 4,
                win: HashSet::from_iter(vec![41, 92, 73, 84, 69]),
                own: HashSet::from_iter(vec![59, 84, 76, 51, 58, 5, 54, 83]),
                matching: 1,
            }
        );
        assert_eq!(
            Card::from_str("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36"),
            Card {
                id: 5,
                win: HashSet::from_iter(vec![87, 83, 26, 28, 32]),
                own: HashSet::from_iter(vec![88, 30, 70, 12, 93, 22, 82, 36]),
                matching: 0,
            }
        );
        assert_eq!(
            Card::from_str("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"),
            Card {
                id: 6,
                win: HashSet::from_iter(vec![31, 18, 13, 56, 72]),
                own: HashSet::from_iter(vec![74, 77, 10, 23, 35, 67, 36, 11]),
                matching: 0,
            }
        );
        assert_eq!(1 + 1, 2);
    }

    #[test]
    fn part1_total_sample() {
        let cards = scan("./data/test_part1.txt");
        let sum = sum_winning_points(&cards);
        assert_eq!(sum, 13);
    }

    #[test]
    fn part1_total_final() {
        let cards = scan("./data/input.txt");
        let sum = sum_winning_points(&cards);
        assert_eq!(sum, 24160);
    }

    #[test]
    fn part2_total_sample() {
        let cards = scan("./data/test_part1.txt");
        let total = total_scratchcards(&cards);
        assert_eq!(total, 30);
    }

    #[test]
    fn part2_total_final() {
        let cards = scan("./data/input.txt");
        let total = total_scratchcards(&cards);
        assert_eq!(total, 5659035);
    }
}
