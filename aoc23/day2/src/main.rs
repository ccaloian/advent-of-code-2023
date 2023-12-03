use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let part1_total = get_possible_games_sum("./data/input.txt");
    println!("Day1, Part One: {part1_total}");

    let part2_total = get_sum_of_powers("./data/input.txt");
    println!("Day1, Part Two: {part2_total}");
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl Game {
    /// Create a game object from a string set, _e.g._ ' 3 blue, 4 red', ' 3 green, 4 blue, 1 red'.
    pub fn from_set(set: &str) -> Self {
        let mut game_map: HashMap<String, u64> = HashMap::new();
        for cube in set.split(',') {
            if let Some((n, col)) = cube.trim_start().split_once(' ') {
                game_map.insert(col.to_string(), n.parse::<u64>().unwrap());
            }
        }
        Game {
            red: match game_map.get("red") {
                Some(n) => *n,
                None => 0,
            },
            green: match game_map.get("green") {
                Some(n) => *n,
                None => 0,
            },
            blue: match game_map.get("blue") {
                Some(n) => *n,
                None => 0,
            },
        }
    }

    /// Return true if the game is possible given the `bag`.
    pub fn possible(&self, bag: &Game) -> bool {
        self.red <= bag.red && self.green <= bag.green && self.blue <= bag.blue
    }

    /// Return the power of the cubes in the set, defined as the product of all cubes.
    pub fn power(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

fn get_game_id(game: &str) -> Option<u64> {
    match game.split_once(':') {
        Some((game_id, _)) => match game_id.split_once(' ') {
            Some(("Game", n)) => Some(n.parse::<u64>().unwrap()),
            _ => None,
        },
        _ => None,
    }
}

fn get_game_sets(game: &str) -> Option<Vec<&str>> {
    match game.split_once(':') {
        Some((_, game_sets)) => Some(game_sets.split(';').collect()),
        _ => None,
    }
}

fn all_game_sets_possible(game_set: &str, bag: &Game) -> Option<bool> {
    if let Some(game_sets) = get_game_sets(game_set) {
        return Some(
            game_sets
                .into_iter()
                .all(|gs| Game::from_set(gs).possible(bag)),
        );
    }
    None
}

fn get_possible_games_sum(filepath: &str) -> u64 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let ref_game = get_game_bag();

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        match all_game_sets_possible(&line, &ref_game) {
            Some(true) => {
                if let Some(game_id) = get_game_id(&line) {
                    total += game_id;
                }
            }
            _ => continue,
        }
    }
    total
}

fn get_sum_of_powers(filepath: &str) -> u64 {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);

    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let game_vec = get_game_sets(&line)
            .unwrap()
            .into_iter()
            .map(Game::from_set)
            .collect::<Vec<Game>>();
        let power = get_min_required_cubes(&game_vec).power();
        total += power;
    }
    total
}

/// Return a game containing the minimum number of cubes of each color that would make all game
/// sets possible.
fn get_min_required_cubes(game_set: &[Game]) -> Game {
    let min_req_red = game_set.iter().map(|g| g.red).max();
    let min_req_green = game_set.iter().map(|g| g.green).max();
    let min_req_blue = game_set.iter().map(|g| g.blue).max();
    Game {
        red: min_req_red.unwrap_or(0),
        green: min_req_green.unwrap_or(0),
        blue: min_req_blue.unwrap_or(0),
    }
}

/// Return the reference game bag.
fn get_game_bag() -> Game {
    Game {
        red: 12,
        green: 13,
        blue: 14,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_ids() {
        assert_eq!(
            get_game_id("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Some(1)
        );
        assert_eq!(
            get_game_id("Game 12: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Some(12)
        );
    }

    #[test]
    fn game_sets() {
        assert_eq!(
            get_game_sets("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            Some(vec![
                " 3 blue, 4 red",
                " 1 red, 2 green, 6 blue",
                " 2 green"
            ])
        );
        assert_eq!(
            get_game_sets("Game 12: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
            Some(vec![" 6 red, 1 blue, 3 green", " 2 blue, 1 red, 2 green"])
        );
    }

    #[test]
    fn game_from_set() {
        assert_eq!(
            Game::from_set(" 3 blue, 4 red"),
            Game {
                red: 4,
                green: 0,
                blue: 3,
            }
        );
        assert_eq!(
            Game::from_set(" 6 red, 1 blue, 3 green"),
            Game {
                red: 6,
                green: 3,
                blue: 1,
            }
        );
        assert_eq!(
            Game::from_set(" 3 green"),
            Game {
                red: 0,
                green: 3,
                blue: 0,
            }
        );
    }

    #[test]
    fn possible_games() {
        let ref_game = get_game_bag();
        assert!(all_game_sets_possible(
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            &ref_game
        )
        .unwrap());
        assert!(all_game_sets_possible(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            &ref_game
        )
        .unwrap());
        assert!(all_game_sets_possible(
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            &ref_game
        )
        .unwrap());
    }

    #[test]
    fn impossible_games() {
        let ref_game = get_game_bag();
        assert!(!all_game_sets_possible(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            &ref_game
        )
        .unwrap());
        assert!(!all_game_sets_possible(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            &ref_game
        )
        .unwrap());
    }

    #[test]
    fn part1_total_sample() {
        assert_eq!(get_possible_games_sum("./data/test_part1.txt"), 8);
    }

    #[test]
    fn part1_total_final() {
        assert_eq!(get_possible_games_sum("./data/input.txt"), 2720);
    }

    #[test]
    fn part2_total_sample() {
        assert_eq!(get_sum_of_powers("./data/test_part2.txt"), 2286);
    }

    #[test]
    fn part2_total_final() {
        assert_eq!(get_sum_of_powers("./data/input.txt"), 71535);
    }
}
