use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::collections::{HashSet, VecDeque};
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let map = read_data("./data/input.txt");
    let total_energized_tiles = part_1(&map);
    println!("Day 16, Part 1: {:?}", total_energized_tiles);
}

type Map = Vec<Vec<Tile>>;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Empty,
    LeftMirror,
    RightMirror,
    HorizontalSplitter,
    VerticalSplitter,
}

impl Tile {
    fn new(symbol: &u8) -> Self {
        match symbol {
            b'\\' => Self::LeftMirror,
            b'/' => Self::RightMirror,
            b'-' => Self::HorizontalSplitter,
            b'|' => Self::VerticalSplitter,
            _ => Self::Empty,
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::LeftMirror => write!(f, "\\"),
            Self::RightMirror => write!(f, "/"),
            Self::HorizontalSplitter => write!(f, "-"),
            Self::VerticalSplitter => write!(f, "|"),
            Self::Empty => write!(f, "."),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Up => write!(f, "^"),
            Self::Down => write!(f, "v"),
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
        }
    }
}

struct Beam {
    id: String,
    x: i32,
    y: i32,
    tile: Tile,
    direction: Direction,
    completed: bool,
    seen: HashSet<(usize, usize)>,
}

impl Hash for Beam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for Beam {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Beam {}

impl Beam {
    fn new(x: i32, y: i32, tile: Tile, direction: Direction) -> Self {
        Beam {
            id: rand_id(),
            x,
            y,
            tile,
            direction,
            completed: false,
            seen: HashSet::new(),
        }
    }

    fn has_split(&self, splits: &mut HashSet<(usize, usize)>) -> bool {
        ((self.tile == Tile::VerticalSplitter
            && (self.direction == Direction::Right || self.direction == Direction::Left))
            || (self.tile == Tile::HorizontalSplitter
                && (self.direction == Direction::Up || self.direction == Direction::Down)))
            && splits.contains(&(self.x as usize, self.y as usize))
    }

    fn update(
        &mut self,
        beams: &mut VecDeque<Beam>,
        splits: &mut HashSet<(usize, usize)>,
        map: &Map,
    ) {
        self.seen.insert((self.x as usize, self.y as usize));
        let mut new_beam: Option<Beam> = None;
        // get the new direction given the current direction and current tile
        let new_dir = match self.direction {
            Direction::Up => match self.tile {
                Tile::Empty => Direction::Up,
                Tile::LeftMirror => Direction::Left,
                Tile::RightMirror => Direction::Right,
                Tile::VerticalSplitter => Direction::Up,
                Tile::HorizontalSplitter => {
                    new_beam = Some(Beam::new(self.x, self.y, self.tile, Direction::Left));
                    Direction::Right
                }
            },
            Direction::Down => match self.tile {
                Tile::Empty => Direction::Down,
                Tile::LeftMirror => Direction::Right,
                Tile::RightMirror => Direction::Left,
                Tile::VerticalSplitter => Direction::Down,
                Tile::HorizontalSplitter => {
                    new_beam = Some(Beam::new(self.x, self.y, self.tile, Direction::Left));
                    Direction::Right
                }
            },
            Direction::Left => match self.tile {
                Tile::Empty => Direction::Left,
                Tile::LeftMirror => Direction::Up,
                Tile::RightMirror => Direction::Down,
                Tile::VerticalSplitter => {
                    new_beam = Some(Beam::new(self.x, self.y, self.tile, Direction::Down));
                    Direction::Up
                }
                Tile::HorizontalSplitter => Direction::Left,
            },
            Direction::Right => match self.tile {
                Tile::Empty => Direction::Right,
                Tile::LeftMirror => Direction::Down,
                Tile::RightMirror => Direction::Up,
                Tile::VerticalSplitter => {
                    new_beam = Some(Beam::new(self.x, self.y, self.tile, Direction::Down));
                    Direction::Up
                }
                Tile::HorizontalSplitter => Direction::Right,
            },
        };
        let (new_x, new_y) = match new_dir {
            Direction::Up => (self.x, self.y - 1),
            Direction::Down => (self.x, self.y + 1),
            Direction::Left => (self.x - 1, self.y),
            Direction::Right => (self.x + 1, self.y),
        };

        if !self.has_split(splits) {
            if let Some(b) = new_beam {
                splits.insert((self.x as usize, self.y as usize));
                beams.push_back(b);
            }
        }

        if in_bounds(new_x, new_y, map) {
            self.x = new_x;
            self.y = new_y;
            self.direction = new_dir;
            self.tile = map[new_y as usize][new_x as usize];
        } else {
            self.completed = true;
        }
    }
}

fn in_bounds(x: i32, y: i32, map: &Map) -> bool {
    let (nrow, ncol) = (map.len() as i32, map[0].len() as i32);
    x >= 0 && x < ncol && y >= 0 && y < nrow
}

fn rand_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(16)
        .map(char::from)
        .collect()
}

fn part_1(map: &Map) -> usize {
    let mut beams: VecDeque<Beam> = VecDeque::new();
    beams.push_back(Beam::new(0, 0, map[0][0], Direction::Right));

    let mut splits: HashSet<(usize, usize)> = HashSet::new();

    let mut energized_tiles: HashSet<(usize, usize)> = HashSet::new();

    while !beams.is_empty() {
        println!("Num active beams: {}", beams.len());
        let mut beam = beams.pop_front().unwrap();
        println!("Num active beams after pop_front: {}", beams.len());
        loop {
            println!(
                "{} ({},{}) {} {} [{}, {}]",
                beam.id,
                beam.x,
                beam.y,
                beam.tile,
                beam.direction,
                beam.seen.len(),
                beam.completed
            );
            beam.update(&mut beams, &mut splits, map);
            println!("Num active beams after update: {}", beams.len());
            if beam.completed {
                energized_tiles.extend(beam.seen.iter());
                println!(
                    "Beam {} completed, total energized tiles: {}",
                    beam.id,
                    energized_tiles.len()
                );
                break;
            }
        }
    }
    energized_tiles.len()
}

fn read_data(filepath: &str) -> Map {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .as_bytes()
                .iter()
                .map(Tile::new)
                .collect::<Vec<Tile>>()
        })
        .collect()
}
