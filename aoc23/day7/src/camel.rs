use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Ord, Eq, PartialOrd, PartialEq, Hash, Clone, Copy)]
pub enum Card {
    A = 12,
    K = 11,
    Q = 10,
    T = 9,
    Nine = 8,
    Eight = 7,
    Seven = 6,
    Six = 5,
    Five = 4,
    Four = 3,
    Three = 2,
    Two = 1,
    J = 0,
}

impl Card {
    pub fn from_byte(s: u8) -> Option<Self> {
        match s {
            b'A' => Some(Card::A),
            b'K' => Some(Card::K),
            b'Q' => Some(Card::Q),
            b'J' => Some(Card::J),
            b'T' => Some(Card::T),
            b'9' => Some(Card::Nine),
            b'8' => Some(Card::Eight),
            b'7' => Some(Card::Seven),
            b'6' => Some(Card::Six),
            b'5' => Some(Card::Five),
            b'4' => Some(Card::Four),
            b'3' => Some(Card::Three),
            b'2' => Some(Card::Two),
            _ => None,
        }
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", &self)
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let s = match &self {
            Card::A => "A",
            Card::K => "K",
            Card::Q => "Q",
            Card::J => "J",
            Card::T => "T",
            Card::Nine => "9",
            Card::Eight => "8",
            Card::Seven => "7",
            Card::Six => "6",
            Card::Five => "5",
            Card::Four => "4",
            Card::Three => "3",
            Card::Two => "2",
        };
        write!(f, "{}", s)
    }
}

pub struct Hand {
    pub cards: (Card, Card, Card, Card, Card),
    pub type_: HandType,
    pub rank: u64,
    pub bid: u64,
}

impl Hand {
    pub fn from_str(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();
        let char_set: HashSet<&u8> = HashSet::from_iter("AKQJT98765432".as_bytes());
        let chars_got: HashSet<&u8> = HashSet::from_iter(bytes);
        if !chars_got.is_subset(&char_set) && bytes.len() != 5 {
            return None;
        }
        Some(Hand {
            cards: (
                Card::from_byte(bytes[0]).unwrap(),
                Card::from_byte(bytes[1]).unwrap(),
                Card::from_byte(bytes[2]).unwrap(),
                Card::from_byte(bytes[3]).unwrap(),
                Card::from_byte(bytes[4]).unwrap(),
            ),
            type_: Self::get_type(s),
            rank: 0,
            bid: 0,
        })
    }

    // pub fn set_rank(&mut self, r: u64) {
    //     self.rank = r;
    // }

    pub fn set_bid(&mut self, b: u64) {
        self.bid = b;
    }

    fn get_type(s: &str) -> HandType {
        let mut freq: HashMap<Card, u32> = HashMap::new();
        for b in s.as_bytes() {
            *freq.entry(Card::from_byte(*b).unwrap()).or_insert(0) += 1;
        }

        // if the length of the frequencies is 1, the hand is highest type
        if freq.len() == 1 {
            HandType::FiveOfAKind
        } else {
            // get the count of J and add it to the card with highest count
            // break ties by strongest card
            let j_count = freq.remove(&Card::J).unwrap_or(0);
            let highest_card = freq
                .iter()
                .max_by(|a, b| {
                    // if the counts are equal
                    if a.1 == b.1 {
                        // compare the cards
                        a.0.cmp(&b.0)
                    } else {
                        a.1.cmp(&b.1)
                    }
                })
                .map(|(k, _v)| k)
                .unwrap();
            let mut new_freq: HashMap<&Card, u32> = HashMap::new();
            for (card, count) in freq.iter() {
                if card == highest_card {
                    new_freq.insert(card, *count + j_count);
                } else {
                    new_freq.insert(card, *count);
                }
            }

            match new_freq.len() {
                1 => HandType::FiveOfAKind,
                2 => {
                    if new_freq.values().any(|v| v == &4_u32) {
                        HandType::FourOfAKind
                    } else {
                        HandType::FullHouse
                    }
                }
                3 => {
                    if new_freq.values().any(|v| v == &3_u32) {
                        HandType::ThreeOfAKind
                    } else {
                        HandType::TwoPair
                    }
                }
                4 => HandType::OnePair,
                _ => HandType::HighCard,
            }
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.type_ == other.type_ {
            return self.cards.cmp(&other.cards);
        }
        self.type_.cmp(&other.type_)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.type_ == other.type_
    }
}

impl Eq for Hand {}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4
        )
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.cards.0, self.cards.1, self.cards.2, self.cards.3, self.cards.4
        )
    }
}

#[derive(Debug, Ord, Eq, PartialOrd, PartialEq)]
pub enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}
