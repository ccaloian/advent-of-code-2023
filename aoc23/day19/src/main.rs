use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};
use std::ops;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let (wf_map, ratings) = read_data("./data/input.txt");
    let part_1_res = part_1(&wf_map, &ratings);
    println!("Day 19, Part 1: {:?}", part_1_res);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Workflow(String);

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Accept,
    Reject,
    Workflow(Workflow),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseStateError;

impl FromStr for State {
    type Err = ParseStateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(State::Accept),
            "R" => Ok(State::Reject),
            _ => Ok(State::Workflow(Workflow(s.to_string()))),
        }
    }
}

impl State {
    fn workflow(&self) -> &Workflow {
        if let State::Workflow(w) = self {
            w
        } else {
            panic!("state {:?} is not a workflow", self)
        }
    }
}

#[derive(Debug, PartialEq)]
struct Condition {
    rating: Option<u8>,
    op: Option<u8>,
    value: Option<u64>,
    state: State,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Rating {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRatingError;

impl FromStr for Rating {
    type Err = ParseRatingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ratings = s
            .strip_prefix('{')
            .unwrap()
            .strip_suffix('}')
            .unwrap()
            .split(',')
            .collect::<Vec<&str>>();

        let scores = ratings
            .into_iter()
            .map(|s| s.split_once('=').unwrap().1.parse::<u64>())
            .collect::<Result<Vec<_>, std::num::ParseIntError>>();
        match scores {
            Ok(v) => Ok(Rating {
                x: v[0],
                m: v[1],
                a: v[2],
                s: v[3],
            }),
            Err(_e) => Err(ParseRatingError),
        }
    }
}

impl ops::Add<Rating> for Rating {
    type Output = Rating;

    fn add(self, _rhs: Rating) -> Rating {
        Rating {
            x: self.x + _rhs.x,
            m: self.m + _rhs.m,
            a: self.a + _rhs.a,
            s: self.s + _rhs.s,
        }
    }
}

impl Rating {
    fn sum(&self) -> u64 {
        self.x + self.m + self.a + self.s
    }
}

fn part_1(wf_map: &HashMap<Workflow, Vec<Condition>>, ratings: &Vec<Rating>) -> u64 {
    ratings
        .iter()
        .filter(|r| sort_rating(wf_map, r) == State::Accept)
        .fold(
            Rating {
                x: 0,
                m: 0,
                a: 0,
                s: 0,
            },
            |acc, x| acc + *x,
        )
        .sum()
}

fn sort_rating(wf_map: &HashMap<Workflow, Vec<Condition>>, rating: &Rating) -> State {
    let mut curr_state = State::Workflow(Workflow("in".to_string()));
    while curr_state != State::Accept && curr_state != State::Reject {
        let wf = curr_state.workflow();
        let curr_workflow = wf_map.get(wf).unwrap();
        for cond in curr_workflow {
            if let Some(r) = cond.rating {
                match r {
                    b'x' => match cond.op.unwrap() {
                        b'>' => {
                            if rating.x > cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        b'<' => {
                            if rating.x < cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        _ => panic!("unknown cond.op"),
                    },
                    b'm' => match cond.op.unwrap() {
                        b'>' => {
                            if rating.m > cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        b'<' => {
                            if rating.m < cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        _ => panic!("unknown cond.op"),
                    },
                    b'a' => match cond.op.unwrap() {
                        b'>' => {
                            if rating.a > cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        b'<' => {
                            if rating.a < cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        _ => panic!("unknown cond.op"),
                    },
                    b's' => match cond.op.unwrap() {
                        b'>' => {
                            if rating.s > cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        b'<' => {
                            if rating.s < cond.value.unwrap() {
                                curr_state = cond.state.clone();
                                break;
                            }
                        }
                        _ => panic!("unknown cond.op"),
                    },
                    _ => panic!("unknown rating"),
                }
            } else {
                curr_state = cond.state.clone();
            }
        }
    }
    curr_state
}

fn read_data(filepath: &str) -> (HashMap<Workflow, Vec<Condition>>, Vec<Rating>) {
    let path = Path::new(filepath);
    let file = File::open(path).unwrap();
    let mut reader = BufReader::new(file);

    let mut input = String::new();
    let _ = reader.read_to_string(&mut input);

    let (workflows_str, ratings_str) = input.split_once("\n\n").unwrap();

    let mut workflows = HashMap::new();
    for wf in workflows_str.split('\n') {
        let (k, v) = parse_workflow(wf);
        workflows.insert(Workflow(k), v);
    }

    let ratings = ratings_str
        .split('\n')
        .map(|r| Rating::from_str(r).unwrap())
        .collect::<Vec<Rating>>();

    (workflows, ratings)
}

fn parse_workflow(w: &str) -> (String, Vec<Condition>) {
    let (key, val) = w.split_once('{').unwrap();
    let conditions = val
        .strip_suffix('}')
        .unwrap()
        .split(',')
        .map(|wf| {
            if wf.contains(':') {
                let (cond, state) = wf.split_once(':').unwrap();
                Condition {
                    rating: Some(cond.as_bytes()[0]),
                    op: Some(cond.as_bytes()[1]),
                    value: Some(cond[2..].parse::<u64>().unwrap()),
                    state: State::from_str(state).unwrap(),
                }
            } else {
                Condition {
                    rating: None,
                    op: None,
                    value: None,
                    state: State::from_str(wf).unwrap(),
                }
            }
        })
        .collect::<Vec<Condition>>();
    (key.to_string(), conditions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn state_from_str() {
        assert_eq!(State::from_str("A").unwrap(), State::Accept);
        assert_eq!(State::from_str("R").unwrap(), State::Reject);
        assert_eq!(
            State::from_str("rfg").unwrap(),
            State::Workflow(Workflow("rfg".to_string()))
        );
    }

    #[test]
    fn parse_workflow_ok() {
        let w = "px{a<2006:qkq,m>2090:A,rfg}";
        let expected_cond = vec![
            Condition {
                rating: Some(b'a'),
                op: Some(b'<'),
                value: Some(2006),
                state: State::Workflow(Workflow("qkq".to_string())),
            },
            Condition {
                rating: Some(b'm'),
                op: Some(b'>'),
                value: Some(2090),
                state: State::Accept,
            },
            Condition {
                rating: None,
                op: None,
                value: None,
                state: State::Workflow(Workflow("rfg".to_string())),
            },
        ];
        assert_eq!(parse_workflow(w), ("px".to_string(), expected_cond));
    }

    #[test]
    fn rating_from_str() {
        let r = "{x=787,m=2655,a=1222,s=2876}";
        assert_eq!(
            Rating::from_str(r),
            Ok(Rating {
                x: 787,
                m: 2655,
                a: 1222,
                s: 2876
            })
        );
    }

    #[test]
    fn add_ratings() {
        assert_eq!(
            Rating::from_str("{x=787,m=2655,a=1222,s=2876}").unwrap()
                + Rating::from_str("{x=2036,m=264,a=79,s=2244}").unwrap()
                + Rating::from_str("{x=2127,m=1623,a=2188,s=1013}").unwrap(),
            Rating {
                x: 787 + 2036 + 2127,
                m: 2655 + 264 + 1623,
                a: 1222 + 79 + 2188,
                s: 2876 + 2244 + 1013,
            }
        );
    }

    #[test]
    fn sum_rating_scores() {
        let rating = Rating {
            x: 787 + 2036 + 2127,
            m: 2655 + 264 + 1623,
            a: 1222 + 79 + 2188,
            s: 2876 + 2244 + 1013,
        };
        let expected = 19114;
        assert_eq!(rating.sum(), expected);
    }
}
