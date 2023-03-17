/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum CardValues {
    A,
    K,
    Q,
    J,
    N10,
    N9,
    N8,
    N7,
    N6,
    N5,
    N4,
    N3,
    N2,
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
    score: u32,
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
                        score: 0,
                    };
                    hand.set_score();
                    Ok(hand)
                }
            }
            Err(msg) => Err(msg),
        }
    }
}

impl PokerHand<'_> {
    fn set_score(&mut self) {
        self.score = 10;
    }

    fn is_straight_flush(&self) -> bool {
        unimplemented!()
    }

    fn is_full_house(&self) -> bool {
        unimplemented!()
    }

    fn is_flush(&self) -> bool {
        unimplemented!()
    }

    fn is_straight(&self) -> bool {
        unimplemented!()
    }

    fn is_three_of_a_kind(&self) -> bool {
        unimplemented!()
    }

    fn is_two_pair(&self) -> bool {
        unimplemented!()
    }

    fn is_high_card(&self) -> bool {
        unimplemented!()
    }
}

pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    unimplemented!("Out of {hands:?}, which hand wins?")
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
                Card {
                    value: CardValues::J,
                    suit: CardSuits::Hearts,
                },
                Card {
                    value: CardValues::N6,
                    suit: CardSuits::Hearts,
                },
                Card {
                    value: CardValues::N5,
                    suit: CardSuits::Diamonds,
                },
                Card {
                    value: CardValues::N4,
                    suit: CardSuits::Spades,
                },
                Card {
                    value: CardValues::N3,
                    suit: CardSuits::Spades,
                },
            ],
            str_ref: &"3S 4S 5D 6H JH",
            score: 10,
        })
    )
}
