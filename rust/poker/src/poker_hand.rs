use crate::card::{card_ranks::CardRanks, *};
use std::cmp::Ordering;

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
    pub cards: Vec<(usize, Card)>,
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
            Ok(mut general_hand) => {
                if general_hand.len() != 5 {
                    Err(format!("Invalid PokerHand: {}", value))
                } else {
                    general_hand.sort();
                    if PokerHand::is_low_ace(&general_hand) {
                        general_hand[4].rank = CardRanks::LowA;
                        let buf = general_hand[0];
                        general_hand[0] = general_hand[4];
                        general_hand[4] = buf;
                    }
                    let poker_hand = PokerHand::prepare_for_poker(&mut general_hand);
                    let mut hand = Self {
                        cards: poker_hand,
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
            for i in (0..self.cards.len()).rev() {
                if self.cards[i].1.rank > other.cards[i].1.rank {
                    return Some(Ordering::Greater);
                } else if self.cards[i].1.rank < other.cards[i].1.rank {
                    return Some(Ordering::Less);
                }
            }
            Some(Ordering::Equal)
        } else if self.score < other.score {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Greater)
        }
    }
}

impl PokerHand<'_> {
    fn prepare_for_poker(gen_hand: &mut Vec<Card>) -> Vec<(usize, Card)> {
        let mut poker_hand: Vec<(usize, Card)> = vec![];

        gen_hand.sort();
        for i in 0..5 {
            let mut is_duplicate = false;
            for j in 0..poker_hand.len() {
                if gen_hand[i].rank == poker_hand[j].1.rank {
                    poker_hand[j].0 += 1;
                    is_duplicate = true;
                    break;
                } else if gen_hand[i].rank < poker_hand[j].1.rank {
                    break;
                }
            }
            if !is_duplicate {
                poker_hand.push((1, gen_hand[i]));
            }
        }
        poker_hand.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        poker_hand
    }

    fn is_low_ace(gen_hand: &Vec<Card>) -> bool {
        if gen_hand[4].rank != CardRanks::A || gen_hand[0].rank != CardRanks::N2 {
            false
        } else {
            let mut last_card = &gen_hand[0];
            for i in 1..4 {
                if last_card.rank != (gen_hand[i].rank as u8 - 1).into() {
                    return false;
                }
                last_card = &gen_hand[i];
            }
            true
        }
    }

    fn set_score(&mut self) {
        if self.is_straight_flush() {
            self.score = Score::StraightFlush;
        } else if self.is_four_of_a_kind() {
            self.score = Score::FourOfAKind;
        } else if self.is_full_house() {
            self.score = Score::FullHouse;
        } else if self.is_flush() {
            self.score = Score::Flush;
        } else if self.is_straight() {
            self.score = Score::Straight;
        } else if self.is_three_of_a_kind() {
            self.score = Score::ThreeOfAKind;
        } else if self.is_two_pair() {
            self.score = Score::TwoPair;
        } else if self.is_one_pair() {
            self.score = Score::OnePair;
        }
    }

    fn is_straight_flush(&self) -> bool {
        self.is_flush() && self.is_straight()
    }

    fn is_four_of_a_kind(&self) -> bool {
        self.cards.last().unwrap().0 == 4
    }

    fn is_full_house(&self) -> bool {
        self.cards.len() == 2 && self.cards.last().unwrap().0 == 3
    }

    fn is_flush(&self) -> bool {
        let suit = &self.cards[0].1.suit;
        !(self.cards.len() != 5 || self.cards.iter().any(|(_, card)| card.suit != *suit))
    }

    fn is_straight(&self) -> bool {
        if self.cards.len() != 5 {
            return false;
        }

        let mut last_card = &self.cards[0].1;
        for i in 1..5 {
            if last_card.rank != (self.cards[i].1.rank as u8 - 1).into() {
                return false;
            }
            last_card = &self.cards[i].1;
        }
        true
    }

    fn is_three_of_a_kind(&self) -> bool {
        self.cards.last().unwrap().0 == 3
    }

    fn is_two_pair(&self) -> bool {
        self.cards.len() == 3
    }

    fn is_one_pair(&self) -> bool {
        self.cards.last().unwrap().0 == 2
    }
}
