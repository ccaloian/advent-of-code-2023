use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let universe = read_data("./data/input.txt");
    let dist_expansion_1 = get_total_galaxy_distances(&universe, 2);
    println!("Day 11, Part 1: {:?}", dist_expansion_1);

    let dist_expansion_1mil = get_total_galaxy_distances(&universe, 1_000_000);
    println!("Day 11, Part 2: {:?}", dist_expansion_1mil);
}

struct Expansions {
    rows: HashSet<i64>,
    cols: HashSet<i64>,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Position {
    x: i64,
    y: i64,
}

#[derive(Debug, PartialEq, Hash, Eq)]
struct Galaxy {
    position: Position,
}

impl Galaxy {
    fn dist(&self, other: &Galaxy, expansions: &Expansions, expansion_factor: i64) -> i64 {
        let x_range = if self.position.x < other.position.x {
            self.position.x..other.position.x
        } else {
            other.position.x..self.position.x
        };
        let num_row_expansions: HashSet<i64> = HashSet::from_iter(x_range)
            .intersection(&expansions.rows)
            .cloned()
            .collect();

        let y_range = if self.position.y < other.position.y {
            self.position.y..other.position.y
        } else {
            other.position.y..self.position.y
        };
        let num_col_expansions: HashSet<i64> = HashSet::from_iter(y_range)
            .intersection(&expansions.cols)
            .cloned()
            .collect();
        (self.position.x - other.position.x).abs()
            + (self.position.y - other.position.y).abs()
            + num_row_expansions.len() as i64 * expansion_factor
            - num_row_expansions.len() as i64
            + num_col_expansions.len() as i64 * expansion_factor
            - num_col_expansions.len() as i64
    }
}

fn get_total_galaxy_distances(universe: &[Vec<u8>], expansion_factor: i64) -> i64 {
    let galaxies = get_galaxies(universe);
    let expansions = get_expansions(&galaxies, universe);
    get_pairwise_distances(&galaxies, &expansions, expansion_factor)
}

fn get_pairwise_distances(
    galaxies: &HashSet<Galaxy>,
    expansions: &Expansions,
    expansion_factor: i64,
) -> i64 {
    let mut seen: HashSet<(&Galaxy, &Galaxy)> = HashSet::new();
    let mut total = 0;
    for g1 in galaxies {
        for g2 in galaxies {
            if g1 != g2 && !seen.contains(&(g2, g1)) {
                total += g1.dist(g2, expansions, expansion_factor);
            }
            seen.insert((g1, g2));
        }
    }
    total
}

fn get_expansions(galaxies: &HashSet<Galaxy>, universe: &[Vec<u8>]) -> Expansions {
    let (nrow, ncol) = (universe.len() as i64, universe[0].len() as i64);
    let occupied_rows: HashSet<i64> = galaxies.iter().map(|g| g.position.x).collect();
    let occupied_cols: HashSet<i64> = galaxies.iter().map(|g| g.position.y).collect();
    let empty_rows = HashSet::from_iter(1..nrow)
        .difference(&occupied_rows)
        .cloned()
        .collect();
    let empty_cols = HashSet::from_iter(1..ncol)
        .difference(&occupied_cols)
        .cloned()
        .collect();
    Expansions {
        rows: empty_rows,
        cols: empty_cols,
    }
}

fn get_galaxies(universe: &[Vec<u8>]) -> HashSet<Galaxy> {
    let mut galaxies: HashSet<Galaxy> = HashSet::new();
    for (i, row) in universe.iter().enumerate() {
        for (j, item) in row.iter().enumerate() {
            if *item == b'#' {
                galaxies.insert(Galaxy {
                    position: Position {
                        x: i as i64,
                        y: j as i64,
                    },
                });
            }
        }
    }
    galaxies
}

fn read_data(filepath: &str) -> Vec<Vec<u8>> {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|l| l.unwrap().as_bytes().to_vec())
        .collect()
}
