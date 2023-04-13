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
        match value.parse::<u32>() {
            Ok(rank) => match rank {
                2..=10 => Ok(CardRanks(rank)),
                _ => Err(format!("Invalid CardRanks: {}", value)),
            },
            Err(_) => match "JQKA".find(value) {
                Some(rank) => Ok(CardRanks(rank as u32 + 11)),
                None => Err(format!("Invalid CardRanks: {}", value)),
            },
        }
    }
}
