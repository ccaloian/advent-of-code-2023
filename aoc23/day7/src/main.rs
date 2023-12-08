mod camel;
use camel::{Card, Hand, HandType};

fn main() {}

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
    fn camel_hand_print() {
        let hand = Hand {
            cards: (Card::A, Card::A, Card::K, Card::K, Card::Eight),
            type_: HandType::TwoPair,
            rank: 10,
            bid: 100,
        };
        assert_eq!(format!("{}", hand), "AAKK8");
    }

    #[test]
    fn camel_hand_create() {
        let hand = Hand::from_str("AAAAA").unwrap();
        assert_eq!(hand.type_, HandType::FiveOfAKind);
        let hand = Hand::from_str("AA8AA").unwrap();
        assert_eq!(hand.type_, HandType::FourOfAKind);
        let hand = Hand::from_str("23332").unwrap();
        assert_eq!(hand.type_, HandType::FullHouse);
        let hand = Hand::from_str("TTT98").unwrap();
        assert_eq!(hand.type_, HandType::ThreeOfAKind);
        let hand = Hand::from_str("23432").unwrap();
        assert_eq!(hand.type_, HandType::TwoPair);
        let hand = Hand::from_str("A23A4").unwrap();
        assert_eq!(hand.type_, HandType::OnePair);
        let hand = Hand::from_str("23456").unwrap();
        assert_eq!(hand.type_, HandType::HighCard);
    }

    #[test]
    fn camel_card_compare_diff_type() {
        let hand_1 = Hand::from_str("AAAAA").unwrap();
        let hand_2 = Hand::from_str("AA8AA").unwrap();
        assert!(hand_1 > hand_2);

        let hand_1 = Hand::from_str("23332").unwrap();
        let hand_2 = Hand::from_str("A23A4").unwrap();
        assert!(hand_1 > hand_2);
    }

    #[test]
    fn camel_card_compare_same_type() {
        let hand_1 = Hand::from_str("33332").unwrap();
        let hand_2 = Hand::from_str("2AAAA").unwrap();
        assert!(hand_1 > hand_2);

        let hand_1 = Hand::from_str("77888").unwrap();
        let hand_2 = Hand::from_str("77788").unwrap();
        assert!(hand_1 > hand_2);
    }
}
