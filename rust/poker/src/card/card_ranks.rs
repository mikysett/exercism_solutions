use int_enum::IntEnum;

#[repr(i8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Ord, Eq, IntEnum)]
pub enum CardRanks {
    LowA = 1,
    N2 = 2,
    N3 = 3,
    N4 = 4,
    N5 = 5,
    N6 = 6,
    N7 = 7,
    N8 = 8,
    N9 = 9,
    N10 = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
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
