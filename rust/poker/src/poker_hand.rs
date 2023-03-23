use crate::card::{card_ranks::CardRanks, *};
use int_enum::IntEnum;
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
        let cards: Result<Vec<Card>, <Card as TryFrom<&str>>::Error> =
            value.split_whitespace().map(Card::try_from).collect();

        match cards {
            Ok(mut general_hand) => {
                if general_hand.len() != 5 {
                    Err(format!("Invalid PokerHand: {}", value))
                } else {
                    general_hand.sort();
                    if PokerHand::is_low_ace(&general_hand) {
                        general_hand[4].rank = CardRanks::LowA;
                        general_hand.swap(0, 4);
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
        match self.score.cmp(&other.score) {
            Ordering::Equal => {
                for i in (0..self.cards.len()).rev() {
                    match self.cards[i].1.rank.cmp(&other.cards[i].1.rank) {
                        Ordering::Greater => return Some(Ordering::Greater),
                        Ordering::Less => return Some(Ordering::Less),
                        Ordering::Equal => continue,
                    }
                }
                Some(Ordering::Equal)
            }
            Ordering::Less => Some(Ordering::Less),
            Ordering::Greater => Some(Ordering::Greater),
        }
    }
}

impl PokerHand<'_> {
    fn prepare_for_poker(gen_hand: &mut [Card]) -> Vec<(usize, Card)> {
        let mut poker_hand: Vec<(usize, Card)> = vec![];

        gen_hand.sort();
        for curr_gen_hand in gen_hand {
            let mut is_duplicate = false;
            for curr_poker_hand in &mut poker_hand {
                match curr_gen_hand.rank.cmp(&curr_poker_hand.1.rank) {
                    Ordering::Equal => {
                        curr_poker_hand.0 += 1;
                        is_duplicate = true;
                        break;
                    }
                    Ordering::Less => break,
                    Ordering::Greater => continue,
                }
            }
            if !is_duplicate {
                poker_hand.push((1, *curr_gen_hand));
            }
        }
        poker_hand.sort_unstable_by(|a, b| a.0.cmp(&b.0));

        poker_hand
    }

    fn is_low_ace(gen_hand: &[Card]) -> bool {
        if gen_hand[4].rank != CardRanks::A || gen_hand[0].rank != CardRanks::N2 {
            false
        } else {
            let mut last_card = &gen_hand[0];
            for curr_gen_hand in gen_hand.iter().take(4).skip(1) {
                if last_card.rank.int_value() != curr_gen_hand.rank.int_value() - 1 {
                    return false;
                }
                last_card = curr_gen_hand;
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
            if last_card.rank.int_value() != self.cards[i].1.rank.int_value() - 1 {
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
