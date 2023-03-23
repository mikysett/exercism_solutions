use strum_macros::EnumString;

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy, EnumString)]
pub enum CardSuits {
    #[strum(serialize = "C")]
    Clubs,
    #[strum(serialize = "D")]
    Diamonds,
    #[strum(serialize = "H")]
    Hearts,
    #[strum(serialize = "S")]
    Spades,
}
