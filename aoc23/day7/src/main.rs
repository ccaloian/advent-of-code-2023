mod camel;
use camel::{Card, Hand, HandType};

fn main() {
    println!("Card A: {:?}{:?}", Card::A, Card::Eight);
    println!("Card T: {:?}", Card::T);
    println!("Card 2: {:?}", Card::Two);
    println!("Hand Five of a Kind: {:?}", HandType::FiveOfAKind);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_cards_order() {
        assert!(Card::A > Card::K);
        assert!(Card::Nine > Card::Six);
        assert!(Card::J > Card::Two);
    }

    #[test]
    fn camel_print_hand() {
        let hand = Hand {
            cards: (Card::A, Card::A, Card::K, Card::K, Card::Eight),
            type_: HandType::TwoPair,
            rank: 10,
            bid: 100,
        };
        assert_eq!(format!("{}", hand), "AAKK8");
    }
}
