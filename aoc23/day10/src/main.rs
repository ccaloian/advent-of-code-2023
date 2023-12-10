use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let maze = read_data("./data/input.txt");
    let maze_loop = get_loop(&maze);
    println!("Day 10, Part 1: {:?}", maze_loop.len() / 2);

    let num_interior_tiles = count_interior_tiles(&maze_loop, &maze);
    println!("Day 10, Part 2: {:?}", num_interior_tiles);
}

#[derive(PartialEq)]
enum Direction {
    Interior,
    Exterior,
}

struct Orientation {
    direction: Direction,
}

impl Orientation {
    fn flip(&mut self) {
        match self.direction {
            Direction::Exterior => self.direction = Direction::Interior,
            Direction::Interior => self.direction = Direction::Exterior,
        }
    }
}

fn count_interior_tiles(loop_items: &HashSet<(i32, i32)>, maze: &[Vec<u8>]) -> u32 {
    let mut total = 0;

    for (i, row) in maze.iter().enumerate() {
        let mut corner: Option<u8> = None;
        let mut orientation = Orientation {
            direction: Direction::Exterior,
        };
        for (j, tile) in row.iter().enumerate() {
            let loop_item = loop_items.contains(&(i as i32, j as i32));
            let interior_point = !loop_item && orientation.direction == Direction::Interior;
            match tile {
                b'.' => {
                    if orientation.direction == Direction::Interior {
                        total += 1;
                    }
                }
                // does not change the orientation
                b'-' => {
                    if interior_point {
                        total += 1;
                    }
                }
                // flips the orientation
                b'|' => {
                    if interior_point {
                        total += 1;
                    } else if loop_item {
                        orientation.flip();
                    }
                }
                // flips the orientation if followed by a '7'
                b'L' => {
                    if interior_point {
                        total += 1;
                    } else if loop_item {
                        corner = Some(b'L');
                    }
                }
                // FIXME: handle this case in a generic way (now assume S=7)
                // flips the orientation if there is an 'L' on the stack
                b'7' | b'S' => {
                    if interior_point {
                        total += 1
                    } else if loop_item && corner.is_some() {
                        if corner.unwrap() == b'L' {
                            orientation.flip();
                            corner = None;
                        } else if corner.unwrap() == b'F' {
                            corner = None;
                        }
                    }
                }
                // flips the orientation if followed by a 'J'
                b'F' => {
                    if interior_point {
                        total += 1;
                    } else if loop_item {
                        corner = Some(b'F');
                    }
                }
                // flips the orientation if there is an 'F' on the stack
                b'J' => {
                    if interior_point {
                        total += 1;
                    } else if loop_item && corner.is_some() {
                        if corner.unwrap() == b'F' {
                            orientation.flip();
                            corner = None;
                        } else if corner.unwrap() == b'L' {
                            corner = None;
                        }
                    }
                }
                t => panic!("cannot handle tile {} at ({},{})", t, i, j),
            }
        }
    }

    total
}

fn get_loop(maze: &[Vec<u8>]) -> HashSet<(i32, i32)> {
    // Find start position ('S')
    let start_row = maze
        .iter()
        .position(|row| row.iter().any(|b| *b == b'S'))
        .unwrap();
    let start_col = maze[start_row].iter().position(|b| *b == b'S').unwrap();

    // Start position and character
    let mut curr_pos = (start_row as i32, start_col as i32);
    // let mut curr_char = b'S';

    let mut seen_pos: HashSet<(i32, i32)> = HashSet::new();
    seen_pos.insert(curr_pos);

    // Special case for getting the next position from the start
    curr_pos = get_first_move(curr_pos, maze);
    let mut curr_char = get_char(curr_pos, maze);
    seen_pos.insert(curr_pos);

    while let Some(next_pos) = get_next_pos(&curr_char, &curr_pos, &seen_pos, maze) {
        seen_pos.insert(next_pos);
        curr_char = get_char(next_pos, maze);
        curr_pos = next_pos;
    }
    seen_pos
}

// Return true if `next_pos` is within the maze boundaries, otherwise false.
fn can_move(next_pos: (i32, i32), maze: &[Vec<u8>]) -> bool {
    let maze_dim = (maze.len() as i32, maze[0].len() as i32);
    (0 <= next_pos.0 && next_pos.0 < maze_dim.0)
        && (0 <= next_pos.1 && next_pos.1 < maze_dim.1)
        && get_char(next_pos, maze) != b'.'
}

fn candidate_pos(curr_char: &u8, curr_pos: (i32, i32)) -> Vec<(i32, i32)> {
    match curr_char {
        b'-' => vec![(curr_pos.0, curr_pos.1 - 1), (curr_pos.0, curr_pos.1 + 1)],
        b'|' => vec![(curr_pos.0 - 1, curr_pos.1), (curr_pos.0 + 1, curr_pos.1)],
        b'J' => vec![(curr_pos.0 - 1, curr_pos.1), (curr_pos.0, curr_pos.1 - 1)],
        b'L' => vec![(curr_pos.0 - 1, curr_pos.1), (curr_pos.0, curr_pos.1 + 1)],
        b'F' => vec![(curr_pos.0 + 1, curr_pos.1), (curr_pos.0, curr_pos.1 + 1)],
        b'7' => vec![(curr_pos.0 + 1, curr_pos.1), (curr_pos.0, curr_pos.1 - 1)],
        _ => panic!("you should not reach here"),
    }
}

fn get_first_move(curr_pos: (i32, i32), maze: &[Vec<u8>]) -> (i32, i32) {
    if can_move((curr_pos.0 - 1, curr_pos.1), maze)
        && "|F7".contains(get_char((curr_pos.0 - 1, curr_pos.1), maze) as char)
    {
        (curr_pos.0 - 1, curr_pos.1)
    } else if can_move((curr_pos.0 + 1, curr_pos.1), maze)
        && "|LJ".contains(get_char((curr_pos.0 + 1, curr_pos.1), maze) as char)
    {
        (curr_pos.0 + 1, curr_pos.1)
    } else if can_move((curr_pos.0, curr_pos.1 - 1), maze)
        && "-FL".contains(get_char((curr_pos.0, curr_pos.1 - 1), maze) as char)
    {
        (curr_pos.0, curr_pos.1 - 1)
    } else {
        (curr_pos.0, curr_pos.1 + 1)
    }
}

fn get_char(pos: (i32, i32), maze: &[Vec<u8>]) -> u8 {
    maze[pos.0 as usize][pos.1 as usize]
}

fn get_next_pos(
    curr_char: &u8,
    curr_pos: &(i32, i32),
    seen_pos: &HashSet<(i32, i32)>,
    maze: &[Vec<u8>],
) -> Option<(i32, i32)> {
    let candidates = candidate_pos(curr_char, *curr_pos);
    let next_pos = candidates
        .iter()
        .find(|pos| can_move(**pos, maze) && !seen_pos.contains(*pos));
    if let Some(pos) = next_pos {
        return Some(*pos);
    }
    None
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
