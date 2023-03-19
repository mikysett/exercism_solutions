#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
pub enum CardSuits {
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
