use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use std::fmt;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let weight_matrix = read_data("./data/test_part1.txt");
    let min_heat_loss = dijkstra(Position { row: 0, col: 0 }, &weight_matrix);
    println!("Day 17, Part 1: {:?}", min_heat_loss);
}

type WeightMatrix = Vec<Vec<u32>>;
type DistMatrix = Vec<Vec<Node>>;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq, Hash)]
struct Position {
    row: i32,
    col: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:>3},{:>3})", self.row, self.col)
    }
}

impl Position {
    fn left(&self, dists: &DistMatrix) -> Option<Self> {
        let new_pos = Position {
            row: self.row,
            col: self.col - 1,
        };
        if in_bounds(new_pos, dists) {
            Some(new_pos)
        } else {
            None
        }
    }

    fn right(&self, dists: &DistMatrix) -> Option<Self> {
        let new_pos = Position {
            row: self.row,
            col: self.col + 1,
        };
        if in_bounds(new_pos, dists) {
            Some(new_pos)
        } else {
            None
        }
    }

    fn up(&self, dists: &DistMatrix) -> Option<Self> {
        let new_pos = Position {
            row: self.row - 1,
            col: self.col,
        };
        if in_bounds(new_pos, dists) {
            Some(new_pos)
        } else {
            None
        }
    }

    fn down(&self, dists: &DistMatrix) -> Option<Self> {
        let new_pos = Position {
            row: self.row + 1,
            col: self.col,
        };
        if in_bounds(new_pos, dists) {
            Some(new_pos)
        } else {
            None
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Node {
    pos: Position,
    weight: u32,
    dist: u32,
    parent: Option<Position>,
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Node {}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // write!(f, "[pos: {}, weight: {}, dist: {}, parent: {:?}]", self.pos, self.weight, self.dist, self.parent)
        write!(f, "{:?}", self)
    }
}

impl Node {
    fn new(pos: Position, weight: u32) -> Self {
        Node {
            pos,
            weight,
            dist: u32::MAX,
            parent: None,
        }
    }

    fn neighbors(&self, dists: &DistMatrix) -> Vec<Position> {
        let mut neighbors: Vec<Position> = Vec::new();
        if let Some(p) = self.pos.left(dists) {
            if !self.is_third_left(dists) {
                neighbors.push(p);
            }
        }
        if let Some(p) = self.pos.right(dists) {
            if !self.is_third_right(dists) {
                neighbors.push(p);
            }
        }
        if let Some(p) = self.pos.up(dists) {
            if !self.is_third_up(dists) {
                neighbors.push(p);
            }
        }
        if let Some(p) = self.pos.down(dists) {
            if !self.is_third_down(dists) {
                neighbors.push(p);
            }
        }
        neighbors
    }

    fn is_third_up(&self, dists: &DistMatrix) -> bool {
        if let Some(pp) = self.parent {
            if pp.row == self.pos.row + 1 {
                let parent = dists[pp.row as usize][pp.col as usize];
                if let Some(gpp) = parent.parent {
                    if gpp.row == self.pos.row + 2 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_third_down(&self, dists: &DistMatrix) -> bool {
        if let Some(pp) = self.parent {
            if pp.row == self.pos.row - 1 {
                let parent = dists[pp.row as usize][pp.col as usize];
                if let Some(gpp) = parent.parent {
                    if gpp.row == self.pos.row - 2 {
                        return true;
                    }
                }
            }
        }
        false
    }
    fn is_third_left(&self, dists: &DistMatrix) -> bool {
        if let Some(pp) = self.parent {
            if pp.col == self.pos.col + 1 {
                let parent = dists[pp.row as usize][pp.col as usize];
                if let Some(gpp) = parent.parent {
                    if gpp.col == self.pos.col + 2 {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn is_third_right(&self, dists: &DistMatrix) -> bool {
        if let Some(pp) = self.parent {
            if pp.col == self.pos.col - 1 {
                let parent = dists[pp.row as usize][pp.col as usize];
                if let Some(gpp) = parent.parent {
                    if gpp.col == self.pos.col - 2 {
                        return true;
                    }
                }
            }
        }
        false
    }
}

fn in_bounds(position: Position, dists: &DistMatrix) -> bool {
    let (nrows, ncols) = (dists.len() as i32, dists[0].len() as i32);
    position.row >= 0 && position.row < nrows && position.col >= 0 && position.col < ncols
}

fn dijkstra(start: Position, weights: &WeightMatrix) -> u32 {
    let (nrows, ncols) = (weights.len() as i32, weights[0].len() as i32);
    let mut pq: PriorityQueue<Position, Reverse<u32>> = PriorityQueue::new();
    let mut dists: DistMatrix = weights
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, v)| {
                    let pos = Position {
                        row: i as i32,
                        col: j as i32,
                    };
                    let node = Node {
                        pos,
                        weight: *v,
                        dist: u32::MAX,
                        parent: None,
                    };
                    pq.push(pos, Reverse(node.dist));
                    node
                })
                .collect()
        })
        .collect();
    dists[start.row as usize][start.col as usize].dist = 0;
    while !pq.is_empty() {
        let (curr_pos, prio) = pq.pop().unwrap();
        // println!("curr_pos: {}, priority: {:?}", curr_pos, prio);
        let curr_node = dists[curr_pos.row as usize][curr_pos.col as usize];
        for neighbor_pos in curr_node.neighbors(&dists) {
            let mut neighbor = dists[neighbor_pos.row as usize][neighbor_pos.col as usize];
            // println!("\tneighbor: {:?}", neighbor);
            let new_dist = curr_node.dist + neighbor.weight;
            if new_dist < neighbor.dist {
                neighbor.dist = new_dist;
                neighbor.parent = Some(curr_pos);
                dists[neighbor_pos.row as usize][neighbor_pos.col as usize] = neighbor;
                pq.change_priority(&neighbor_pos, Reverse(new_dist));
            }
        }
    }

    let mut path: Vec<&Node> = Vec::new();
    let mut curr_node = dists.last().unwrap().last().unwrap();
    path.push(curr_node);
    while let Some(p) = curr_node.parent {
        println!("{}", p);
        curr_node = &dists[p.row as usize][p.col as usize];
        path.push(curr_node);
    }

    let total_path_weights: u32 = path.iter().map(|n| n.weight).sum();
    println!(
        "total path weights: {}",
        total_path_weights - dists[start.row as usize][start.col as usize].weight
    );

    for row in &dists {
        println!();
        for col in row {
            print!("{:>4}", col.dist);
        }
        println!();
    }

    dists.last().unwrap().last().unwrap().dist
}

fn read_data(filepath: &str) -> WeightMatrix {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect()
}
