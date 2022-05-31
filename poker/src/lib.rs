use std::cmp::{Ordering, PartialOrd};
use std::collections::HashMap;

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(PartialEq, Eq)]
struct Card {
    number: u8,
    suit: Suit,
}

impl Card {
    pub fn parse_card(card: &str) -> Option<Card> {
        if let (Some(number), Some(suit)) = match card.len() {
            2 => (Card::parse_rank(&card[0..1]), Card::parse_suit(&card[1..2])),
            3 => (Card::parse_rank(&card[0..2]), Card::parse_suit(&card[2..3])),
            _ => (None, None),
        } {
            Some(Card { number, suit })
        } else {
            None
        }
    }

    fn parse_rank(rank: &str) -> Option<u8> {
        match rank {
            "2" => Some(2),
            "3" => Some(3),
            "4" => Some(4),
            "5" => Some(5),
            "6" => Some(6),
            "7" => Some(7),
            "8" => Some(8),
            "9" => Some(9),
            "10" => Some(10),
            "J" => Some(11),
            "Q" => Some(12),
            "K" => Some(13),
            "A" => Some(14),
            _ => None,
        }
    }

    fn parse_suit(suit: &str) -> Option<Suit> {
        match suit {
            "C" => Some(Suit::Clubs),
            "D" => Some(Suit::Diamonds),
            "H" => Some(Suit::Hearts),
            "S" => Some(Suit::Spades),
            _ => None,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Card) -> Option<Ordering> {
        Some(self.number.cmp(&other.number))
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

struct PokerHand<'a> {
    cards: Vec<Card>,
    cards_string: &'a str,
}

impl<'a> PartialEq for PokerHand<'a> {
    fn eq(&self, other: &PokerHand) -> bool {
        self.rank() == other.rank()
    }
}

impl<'a> PokerHand<'a> {
    fn new(_cards_string: &'a str) -> PokerHand {
        let cards_string = _cards_string;
        let cards = cards_string
            .split(" ")
            .map(|card| {
                if let Some(card) = Card::parse_card(card) {
                    card
                } else {
                    panic!("Invalid card: {}", card);
                }
            })
            .collect();
        PokerHand {
            cards,
            cards_string,
        }
    }
    /// Returns the rank of the hand as a quartet of (HandRank, high card in the rank, high card in secondary rank, kicker)
    fn rank(&self) -> (HandRank, u8, u8, u8) {
        let mut sorted_ranks = self
            .cards
            .iter()
            .map(|card| card.number)
            .collect::<Vec<u8>>();
        sorted_ranks.sort();
        let mut ranks = sorted_ranks.clone();
        ranks.dedup();

        let mut dups = HashMap::new();

        for rank in sorted_ranks.iter() {
            *dups.entry(rank).or_insert(0) += 1;
        }

        match ranks.len() {
            2 => {
                if sorted_ranks[0] == sorted_ranks[3] {
                    (HandRank::FourOfAKind, sorted_ranks[3], sorted_ranks[4], 0)
                } else if sorted_ranks[1] == sorted_ranks[4] {
                    (HandRank::FourOfAKind, sorted_ranks[1], sorted_ranks[0], 0)
                } else {
                    (HandRank::FullHouse, sorted_ranks[0], sorted_ranks[4], 0)
                }
            }
            3 => {
                let max_pair = dups.iter().max_by(|a, b| a.1.cmp(&b.1));
                let cards_in_pair = *max_pair.unwrap().1;

                if cards_in_pair == 3 {
                    let pair_card = **max_pair.unwrap().0;
                    let kicker = ranks
                        .iter()
                        .filter(|rank| **rank != pair_card)
                        .max()
                        .unwrap();
                    (HandRank::ThreeOfAKind, pair_card, *kicker, 0)
                } else {
                    let pair_cards = dups
                        .iter()
                        .filter(|(_, count)| **count == 2)
                        .map(|(rank, _)| **rank)
                        .collect::<Vec<u8>>();
                    let high_pair_card = pair_cards.iter().max().unwrap();
                    let low_pair_card = pair_cards.iter().min().unwrap();
                    let kicker = ranks
                        .iter()
                        .filter(|rank| !pair_cards.contains(*rank))
                        .max()
                        .unwrap();
                    (HandRank::TwoPair, *high_pair_card, *low_pair_card, *kicker)
                }
            }
            4 => {
                let pair_cards = dups
                    .iter()
                    .filter(|(_, count)| **count == 2)
                    .map(|(rank, _)| **rank)
                    .collect::<Vec<u8>>();
                let high_pair_card = pair_cards.iter().max().unwrap();
                let kicker = ranks
                    .iter()
                    .filter(|rank| !pair_cards.contains(*rank))
                    .max()
                    .unwrap();
                (HandRank::OnePair, *high_pair_card, *kicker, 0)
            }
            5 => {
                let ace_straight = ranks[4] - ranks[0] == 12;
                let straight = ranks[4] - ranks[0] == 4 || ace_straight;
                let flush = self
                    .cards
                    .iter()
                    .all(|card| card.suit == self.cards[0].suit);
                match (straight, ace_straight, flush) {
                    (true, true, true) => (HandRank::StraightFlush, ranks[3], 0, 0),
                    (true, false, true) => (HandRank::StraightFlush, ranks[4], 0, 0),
                    (true, true, false) => (HandRank::Straight, ranks[3], 0, 0),
                    (true, false, false) => (HandRank::Straight, ranks[4], 0, 0),
                    (false, false, true) => (HandRank::Flush, ranks[4], 0, 0),
                    _ => (HandRank::HighCard, ranks[4], ranks.iter().sum(), 0),
                }
            }
            _ => panic!("Invalid number of cards: {}", ranks.len()),
        }
    }
}

impl<'a> PartialOrd for PokerHand<'a> {
    fn partial_cmp(&self, other: &PokerHand) -> Option<Ordering> {
        let order = self.rank().cmp(&other.rank());
        Some(order)
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut poker_hands = hands
        .iter()
        .map(|hand| PokerHand::new(hand))
        .collect::<Vec<_>>();

    poker_hands.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let winner = poker_hands.last().unwrap();
    let ties = poker_hands
        .iter()
        .filter(|hand| (**hand) == (*winner))
        .map(|hand| hand.cards_string)
        .collect::<Vec<_>>();
    ties
}
