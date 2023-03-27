pub mod card_ranks;
pub mod card_suits;

use crate::card::card_ranks::CardRanks;
use crate::card::card_suits::CardSuits;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub struct Card {
    pub rank: CardRanks,
    pub suit: CardSuits,
}

impl TryFrom<&str> for Card {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(val_str) = value.rsplit_once(&['C', 'D', 'H', 'S']) {
            Ok(Self {
                rank: CardRanks::try_from(val_str.0)?,
                suit: CardSuits::try_from(value.chars().last().unwrap())?,
            })
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
            rank: CardRanks(14),
            suit: CardSuits::Spades
        })
    );
    assert_eq!(
        Card::try_from("10C"),
        Ok(Card {
            rank: CardRanks(10),
            suit: CardSuits::Clubs
        })
    );
    assert_eq!(
        Card::try_from("QD"),
        Ok(Card {
            rank: CardRanks(12),
            suit: CardSuits::Diamonds
        })
    );
    assert_eq!(Card::try_from(""), Err("Invalid Card: ".to_string()));
    assert_eq!(
        Card::try_from("ASS"),
        Err("Invalid CardRanks: AS".to_string())
    );
    assert_eq!(Card::try_from("A"), Err("Invalid Card: A".to_string()));
    assert_eq!(Card::try_from("10"), Err("Invalid Card: 10".to_string()));
    assert_eq!(
        Card::try_from("1C"),
        Err("Invalid CardRanks: 1".to_string())
    );
}
