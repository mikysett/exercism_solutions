#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Ord, Eq)]
pub enum CardRanks {
    LowA = 1,
    N2,
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

impl TryFrom<&str> for CardRanks {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" => Ok(CardRanks::A),
            "K" => Ok(CardRanks::K),
            "Q" => Ok(CardRanks::Q),
            "J" => Ok(CardRanks::J),
            "10" => Ok(CardRanks::N10),
            "9" => Ok(CardRanks::N9),
            "8" => Ok(CardRanks::N8),
            "7" => Ok(CardRanks::N7),
            "6" => Ok(CardRanks::N6),
            "5" => Ok(CardRanks::N5),
            "4" => Ok(CardRanks::N4),
            "3" => Ok(CardRanks::N3),
            "2" => Ok(CardRanks::N2),
            _ => Err(format!("Invalid CardValue: {}", value)),
        }
    }
}

impl From<u8> for CardRanks {
    fn from(value: u8) -> Self {
        match value {
            14 => CardRanks::A,
            13 => CardRanks::K,
            12 => CardRanks::Q,
            11 => CardRanks::J,
            10 => CardRanks::N10,
            9 => CardRanks::N9,
            8 => CardRanks::N8,
            7 => CardRanks::N7,
            6 => CardRanks::N6,
            5 => CardRanks::N5,
            4 => CardRanks::N4,
            3 => CardRanks::N3,
            2 => CardRanks::N2,
            1 => CardRanks::LowA,
            _ => CardRanks::A,
        }
    }
}
