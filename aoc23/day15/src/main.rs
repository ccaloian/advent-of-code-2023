use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let init_seqs = read_data("./data/input.txt");
    let sum_results = part_1(&init_seqs);
    println!("Day 15, Part 1: {:?}", sum_results);

    let total_focus_power = part_2(&init_seqs);
    println!("Day 15, Part 2: {:?}", total_focus_power);
}

fn part_1(init_seqs: &str) -> u64 {
    init_seqs
        .trim()
        .split(',')
        .map(|s| hash_seq(s) as u64)
        .sum()
}

fn part_2(init_seqs: &str) -> u64 {
    let mut boxes: Vec<Vec<(String, u32)>> = vec![Vec::new(); 256];
    for seq in init_seqs.trim().split(',') {
        let (i, op) = (hash_label(seq), get_op(seq));
        let label = get_label(seq);
        match op {
            b'=' => {
                let fl = get_focal_length(seq).unwrap();
                if let Some(ix) = boxes[i].iter().position(|x| *x.0 == label) {
                    boxes[i][ix] = (label, fl);
                } else {
                    boxes[i].push((label, fl));
                }
            }
            b'-' => {
                if let Some(ix) = boxes[i].iter().position(|x| x.0 == label) {
                    boxes[i].remove(ix);
                }
            }
            _ => panic!("unknown symbol"),
        }
    }
    boxes
        .iter()
        .enumerate()
        .map(|(bi, bx)| {
            bx.iter()
                .enumerate()
                .map(|(si, (_, fl))| (bi + 1) as u64 * (si + 1) as u64 * (*fl) as u64)
                .sum::<u64>()
        })
        .sum()
}

fn get_focal_length(s: &str) -> Option<u32> {
    if s.contains('=') {
        Some(s.split('=').last().unwrap().parse::<u32>().unwrap())
    } else {
        None
    }
}

fn get_op(s: &str) -> u8 {
    if s.contains('=') {
        b'='
    } else {
        b'-'
    }
}

fn hash_label(s: &str) -> usize {
    let mut curr_val = 0;
    for b in s.as_bytes() {
        if b.is_ascii_alphabetic() {
            curr_val += *b as u32;
            curr_val *= 17;
            curr_val %= 256
        } else {
            break;
        }
    }
    curr_val as usize
}

fn get_label(s: &str) -> String {
    String::from_utf8(
        s.as_bytes()
            .iter()
            .take_while(|b| b.is_ascii_alphabetic())
            .map(|b| *b)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}

fn hash_seq(s: &str) -> u32 {
    let mut curr_val = 0;
    for b in s.as_bytes() {
        curr_val += *b as u32;
        curr_val *= 17;
        curr_val %= 256
    }
    curr_val
}

fn read_data(filename: &str) -> String {
    let path = Path::new(filename);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut init_seqs = String::new();
    let _ = reader.read_line(&mut init_seqs).unwrap();

    init_seqs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_algo_works() {
        assert_eq!(hash_seq("rn=1"), 30);
        assert_eq!(hash_seq("cm-"), 253);
        assert_eq!(hash_seq("qp=3"), 97);
        assert_eq!(hash_seq("cm=2"), 47);
        assert_eq!(hash_seq("qp-"), 14);
        assert_eq!(hash_seq("pc=4"), 180);
        assert_eq!(hash_seq("ot=9"), 9);
        assert_eq!(hash_seq("ab=5"), 197);
        assert_eq!(hash_seq("pc-"), 48);
        assert_eq!(hash_seq("pc=6"), 214);
        assert_eq!(hash_seq("ot=7"), 231);
    }

    #[test]
    fn hash_label_works() {
        assert_eq!(hash_label("rn=1"), 0);
        assert_eq!(hash_label("cm-"), 0);
        assert_eq!(hash_label("qp=3"), 1);
        assert_eq!(hash_label("pc=4"), 3);
        assert_eq!(hash_label("ot=9"), 3);
    }

    #[test]
    fn focal_length_works() {
        assert_eq!(get_focal_length("rn=1"), Some(1));
        assert_eq!(get_focal_length("qp=3"), Some(3));
        assert_eq!(get_focal_length("pc=4"), Some(4));
        assert_eq!(get_focal_length("ot=9"), Some(9));
        assert_eq!(get_focal_length("cm-"), None);
    }

    #[test]
    fn get_op_works() {
        assert_eq!(get_op("rn=1"), b'=');
        assert_eq!(get_op("cm-"), b'-');
        assert_eq!(get_op("qp=3"), b'=');
        assert_eq!(get_op("cm=2"), b'=');
        assert_eq!(get_op("qp-"), b'-');
        assert_eq!(get_op("pc=4"), b'=');
        assert_eq!(get_op("ot=9"), b'=');
        assert_eq!(get_op("ab=5"), b'=');
        assert_eq!(get_op("pc-"), b'-');
        assert_eq!(get_op("pc=6"), b'=');
        assert_eq!(get_op("ot=7"), b'=');
    }

    #[test]
    fn get_label_works() {
        assert_eq!(get_label("rn=1"), "rn");
        assert_eq!(get_label("cm-"), "cm");
        assert_eq!(get_label("qp=3"), "qp");
        assert_eq!(get_label("cm=2"), "cm");
        assert_eq!(get_label("qp-"), "qp");
        assert_eq!(get_label("pc=4"), "pc");
        assert_eq!(get_label("ot=9"), "ot");
        assert_eq!(get_label("ab=5"), "ab");
        assert_eq!(get_label("pc-"), "pc");
        assert_eq!(get_label("pc=6"), "pc");
        assert_eq!(get_label("ot=7"), "ot");
    }

    #[test]
    fn hash_seq_works() {
        assert_eq!(
            part_1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"),
            1320
        );
    }
}
