use std::cmp::Ordering;

use crate::card::*;

pub mod card;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum Score {
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

#[derive(Debug, PartialEq)]
pub struct PokerHand<'a> {
    pub cards: Vec<Card>,
    pub str_ref: &'a str,
    pub score: Score,
}

impl<'a> TryFrom<&'a str> for PokerHand<'a> {
    type Error = String;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let cards: Result<Vec<Card>, <Card as TryFrom<&str>>::Error> = value
            .split_whitespace()
            .map(|card| Ok(Card::try_from(card)?))
            .collect();

        match cards {
            Ok(mut hand) => {
                if hand.len() != 5 {
                    Err(format!("Invalid PokerHand: {}", value))
                } else {
                    hand.sort();
                    let mut hand = Self {
                        cards: hand,
                        str_ref: value,
                        score: Score::HighCard,
                    };
                    hand.set_score();
                    Ok(hand)
                }
            }
            Err(msg) => Err(msg),
        }
    }
}

impl PartialOrd for PokerHand<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.score == other.score {
            match self.score {
                Score::HighCard => {
                    for i in (0..5).rev() {
                        if self.cards[i].rank > other.cards[i].rank {
                            return Some(Ordering::Greater);
                        } else if self.cards[i].rank < other.cards[i].rank {
                            return Some(Ordering::Less);
                        }
                    }
                    Some(Ordering::Equal)
                },
                Score::OnePair => {
                    Some(Ordering::Equal)
                },
                Score::TwoPair => {
                    Some(Ordering::Equal)
                },
                Score::ThreeOfAKind => {
                    Some(Ordering::Equal)
                },
                Score::Straight => {
                    Some(Ordering::Equal)
                },
                Score::Flush => {
                    Some(Ordering::Equal)
                },
                Score::FullHouse => {
                    Some(Ordering::Equal)
                },
                Score::FourOfAKind => {
                    Some(Ordering::Equal)
                },
                Score::StraightFlush => {
                    if self.cards[4].rank > other.cards[4].rank {
                        Some(Ordering::Greater)
                    } else if self.cards[4].rank < other.cards[4].rank {
                        Some(Ordering::Less)
                    } else {
                        Some(Ordering::Equal)
                    }
                },
            }
        }
        else if self.score < other.score {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PokerHand<'_> {
    fn set_score(&mut self) {
        let pairs_info = Self::get_pairs_info(&self.cards);

        if self.is_straight_flush() {
            self.score = Score::StraightFlush;
        } else if self.is_four_of_a_kind(&pairs_info) {
            self.score = Score::FourOfAKind;
        } else if self.is_full_house(&pairs_info) {
            self.score = Score::FullHouse;
        } else if self.is_flush() {
            self.score = Score::Flush;
        } else if self.is_straight() {
            self.score = Score::Straight;
        } else if self.is_three_of_a_kind(&pairs_info) {
            self.score = Score::ThreeOfAKind;
        } else if self.is_two_pair(&pairs_info) {
            self.score = Score::TwoPair;
        } else if self.is_one_pair(&pairs_info) {
            self.score = Score::OnePair;
        }
    }

    fn get_pairs_info(cards: &[Card]) -> (u8, u8) {
        let mut different_ranks = 1;
        let mut larger_rank_count = 1;
        let mut current_rank_count = 1;

        let mut last_card = &cards[0];
        for i in 1..5 {
            if last_card.rank != cards[i].rank {
                if current_rank_count > larger_rank_count {
                    larger_rank_count = current_rank_count;
                }
                current_rank_count = 1;
                different_ranks += 1;
            } else {
                current_rank_count += 1;
            }
            last_card = &cards[i];
        }

        if current_rank_count > larger_rank_count {
            larger_rank_count = current_rank_count;
        }
        
        (larger_rank_count, different_ranks)
    }

    fn is_straight_flush(&self) -> bool {
        let mut last_card = &self.cards[0];
        
        for i in 1..5 {
            if last_card.suit != self.cards[i].suit
                || last_card.rank != (self.cards[i].rank as u8  - 1).into() {
                return false;
            }
            last_card = &self.cards[i];
        }
        true
    }

    fn is_four_of_a_kind(&self, pairs_info: &(u8, u8)) -> bool {
        if pairs_info.0 == 4 {
            true
        } else {
            false
        }
    }

    fn is_full_house(&self, pairs_info: &(u8, u8)) -> bool {
        if pairs_info.1 == 2 {
            true
        } else {
            false
        }
    }

    fn is_flush(&self) -> bool {
        let mut last_card = &self.cards[0];
        
        for i in 1..5 {
            if last_card.suit != self.cards[i].suit {
                return false;
            }
            last_card = &self.cards[i];
        }
        true
    }

    fn is_straight(&self) -> bool {
        let mut last_card = &self.cards[0];
        
        for i in 1..5 {
            if last_card.rank != (self.cards[i].rank as u8  - 1).into() {
                return false;
            }
            last_card = &self.cards[i];
        }
        true
    }

    fn is_three_of_a_kind(&self, pairs_info: &(u8, u8)) -> bool {
        if pairs_info.0 == 3 {
            true
        } else {
            false
        }
    }

    fn is_two_pair(&self, pairs_info: &(u8, u8)) -> bool {
        if pairs_info.0 == 2 && pairs_info.1 == 3 {
            true
        } else {
            false
        }
    }

    fn is_one_pair(&self, pairs_info: &(u8, u8)) -> bool {
        if pairs_info.0 == 2 && pairs_info.1 == 4 {
            true
        } else {
            false
        }
    }

    fn is_high_card(&self) -> bool {
        true
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let poker_hands: Result<Vec<PokerHand>, <Card as TryFrom<&str>>::Error> = hands
        .iter()
        .map(|hand| PokerHand::try_from(*hand))
        .collect();

    let mut result = vec![];
    match poker_hands {
        Ok(mut my_hands) => {
            my_hands.sort_by(|a, b| b.partial_cmp(a).unwrap());
    
            for hand in &my_hands {
                if *hand == my_hands[0] {
                    result.push(hand.str_ref);
                } else {
                    break;
                }
            }
        },
        Err(_) => {
            panic!("Bad input");
        }
    }
    return result;
}
