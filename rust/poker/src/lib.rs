use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Ord, Eq)]
enum CardValues {
    N2 = 2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    N10,
    J,
    Q,
    K,
    A,
}

impl TryFrom<&str> for CardValues {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(CardValues::A),
            "K" => Ok(CardValues::K),
            "Q" => Ok(CardValues::Q),
            "J" => Ok(CardValues::J),
            "10" => Ok(CardValues::N10),
            "9" => Ok(CardValues::N9),
            "8" => Ok(CardValues::N8),
            "7" => Ok(CardValues::N7),
            "6" => Ok(CardValues::N6),
            "5" => Ok(CardValues::N5),
            "4" => Ok(CardValues::N4),
            "3" => Ok(CardValues::N3),
            "2" => Ok(CardValues::N2),
            _ => Err(format!("Invalid CardValue: {}", value)),
        }
    }
}

impl From<u8> for CardValues {
    fn from(value: u8) -> Self {
        match value {
            14 => CardValues::A,
            13 => CardValues::K,
            12 => CardValues::Q,
            11 => CardValues::J,
            10 => CardValues::N10,
            9 => CardValues::N9,
            8 => CardValues::N8,
            7 => CardValues::N7,
            6 => CardValues::N6,
            5 => CardValues::N5,
            4 => CardValues::N4,
            3 => CardValues::N3,
            2 => CardValues::N2,
            15..=u8::MAX => CardValues::N2,
            u8::MIN..=1 => CardValues::A,
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum CardSuits {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

impl TryFrom<char> for CardSuits {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'C' => Ok(CardSuits::Clubs),
            'D' => Ok(CardSuits::Diamonds),
            'H' => Ok(CardSuits::Hearts),
            'S' => Ok(CardSuits::Spades),
            _ => Err(format!("Invalid CardSuit: {}", value)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Score {
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

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
struct Card {
    value: CardValues,
    suit: CardSuits,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(val_str) = value.rsplit_once(&['C', 'D', 'H', 'S']) {
            Ok(Self {
                value: CardValues::try_from(val_str.0)?,
                suit: CardSuits::try_from(value.chars().last().unwrap())?,
            })
        } else {
            Err(format!("Invalid Card: {}", value))
        }
    }
}

#[derive(Debug, PartialEq)]
struct PokerHand<'a> {
    cards: [Card; 5],
    str_ref: &'a str,
    score: Score,
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
                        cards: hand.try_into().unwrap(),
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
                        if self.cards[i].value > other.cards[i].value {
                            return Some(Ordering::Greater);
                        } else if self.cards[i].value < other.cards[i].value {
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
                    if self.cards[4].value > other.cards[4].value {
                        Some(Ordering::Greater)
                    } else if self.cards[4].value < other.cards[4].value {
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
            if last_card.value != cards[i].value {
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
                || last_card.value != (self.cards[i].value as u8  - 1).into() {
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
            if last_card.value != (self.cards[i].value as u8  - 1).into() {
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

#[test]
fn test_card() {
    assert_eq!(
        Card::try_from("AS"),
        Ok(Card {
            value: CardValues::A,
            suit: CardSuits::Spades
        })
    );
    assert_eq!(
        Card::try_from("10C"),
        Ok(Card {
            value: CardValues::N10,
            suit: CardSuits::Clubs
        })
    );
    assert_eq!(
        Card::try_from("QD"),
        Ok(Card {
            value: CardValues::Q,
            suit: CardSuits::Diamonds
        })
    );
    assert_eq!(Card::try_from(""), Err("Invalid Card: ".to_string()));
    assert_eq!(
        Card::try_from("ASS"),
        Err("Invalid CardValue: AS".to_string())
    );
    assert_eq!(Card::try_from("A"), Err("Invalid Card: A".to_string()));
    assert_eq!(Card::try_from("10"), Err("Invalid Card: 10".to_string()));
    assert_eq!(
        Card::try_from("1C"),
        Err("Invalid CardValue: 1".to_string())
    );
}

#[test]
fn test_poker_hand() {
    assert_eq!(
        PokerHand::try_from("3S 4S 5D 6H JH"),
        Ok(PokerHand {
            cards: [
                Card {value: CardValues::N3,suit: CardSuits::Spades,},
                Card {value: CardValues::N4,suit: CardSuits::Spades,},
                Card {value: CardValues::N5,suit: CardSuits::Diamonds,},
                Card {value: CardValues::N6,suit: CardSuits::Hearts,},
                Card {value: CardValues::J,suit: CardSuits::Hearts,},
            ],
            str_ref: &"3S 4S 5D 6H JH",
            score: Score::HighCard,
        })
    )
}

#[test]
fn test_hand_scores() {
    let straight_flush = PokerHand::try_from("JD 10D 9D 8D 7D").unwrap();
    assert_eq!(straight_flush.score, Score::StraightFlush);

    let four_of_a_kind = PokerHand::try_from("7D AS AD AC AH").unwrap();
    assert_eq!(four_of_a_kind.score, Score::FourOfAKind);

    let full_house = PokerHand::try_from("7D 7H AD AC AH").unwrap();
    assert_eq!(full_house.score, Score::FullHouse);

    let flush = PokerHand::try_from("AH JH 9H 8H 7H").unwrap();
    assert_eq!(flush.score, Score::Flush);

    let straight = PokerHand::try_from("10H 9S 8D 7D 6H").unwrap();
    assert_eq!(straight.score, Score::Straight);

    let three_of_a_kind = PokerHand::try_from("AH AD AS 2H 3D").unwrap();
    assert_eq!(three_of_a_kind.score, Score::ThreeOfAKind);

    let two_pairs = PokerHand::try_from("7H 7D 6H 5D 5S").unwrap();
    assert_eq!(two_pairs.score, Score::TwoPair);

    let one_pair = PokerHand::try_from("KH QH 10D 2D 2S").unwrap();
    assert_eq!(one_pair.score, Score::OnePair);

    let one_pair = PokerHand::try_from("KH QH 10D AC 2S").unwrap();
    assert_eq!(one_pair.score, Score::HighCard);
}
