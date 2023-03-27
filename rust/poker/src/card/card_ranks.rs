#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Ord, Eq)]
pub struct CardRanks(u32);

impl std::ops::Deref for CardRanks {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CardRanks {
    pub fn new(rank: u32) -> Self {
        Self(rank)
    }
}

impl TryFrom<&str> for CardRanks {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let rank = match value.parse::<u32>() {
            Ok(rank) => rank,
            Err(_) => "JQKA".find(value).unwrap_or(15) as u32 + 11,
        };
        match rank {
            2..=14 => Ok(CardRanks(rank)),
            _ => Err(format!("Invalid CardRanks: {}", value)),
        }
    }
}
