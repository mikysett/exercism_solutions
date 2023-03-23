pub mod card_ranks;
pub mod card_suits;

use crate::card::card_ranks::CardRanks;
use crate::card::card_suits::CardSuits;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct Card {
    pub rank: CardRanks,
    pub suit: CardSuits,
}

impl Card {
    fn new(rank: CardRanks, suit: CardSuits) -> Self {
        Self { rank, suit }
    }
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.len() > 1 {
            let rank_str = &value[0..value.len() - 1];
            let suit_str = &value[value.len() - 1..value.len()];

            Ok(Self::new(
                match CardRanks::try_from(rank_str) {
                    Ok(rank) => rank,
                    _ => return Err(format!("Invalid CardRanks: {}", rank_str)),
                },
                match CardSuits::try_from(suit_str) {
                    Ok(suit) => suit,
                    _ => return Err(format!("Invalid CardSuits: {}", suit_str)),
                },
            ))
        } else {
            Err(format!("Invalid Card: {}", value))
        }
    }
}

#[test]
fn test_card() {
    assert_eq!(
        Card::try_from("AS"),
        Ok(Card {
            rank: CardRanks::A,
            suit: CardSuits::Spades
        })
    );
    assert_eq!(
        Card::try_from("10C"),
        Ok(Card {
            rank: CardRanks::N10,
            suit: CardSuits::Clubs
        })
    );
    assert_eq!(
        Card::try_from("QD"),
        Ok(Card {
            rank: CardRanks::Q,
            suit: CardSuits::Diamonds
        })
    );
    assert_eq!(Card::try_from(""), Err("Invalid Card: ".to_string()));
    assert_eq!(
        Card::try_from("ASS"),
        Err("Invalid CardRanks: AS".to_string())
    );
    assert_eq!(Card::try_from("A"), Err("Invalid Card: A".to_string()));
    assert_eq!(Card::try_from("10"), Err("Invalid CardRanks: 1".to_string()));
    assert_eq!(
        Card::try_from("1C"),
        Err("Invalid CardRanks: 1".to_string())
    );
}
