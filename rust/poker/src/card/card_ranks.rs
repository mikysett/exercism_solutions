use int_enum::IntEnum;
use strum_macros::EnumString;

#[repr(i8)]
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, Ord, Eq, IntEnum, EnumString)]
pub enum CardRanks {
    #[strum(disabled)]
    LowA = 1,
    #[strum(serialize = "2")]
    N2 = 2,
    #[strum(serialize = "3")]
    N3 = 3,
    #[strum(serialize = "4")]
    N4 = 4,
    #[strum(serialize = "5")]
    N5 = 5,
    #[strum(serialize = "6")]
    N6 = 6,
    #[strum(serialize = "7")]
    N7 = 7,
    #[strum(serialize = "8")]
    N8 = 8,
    #[strum(serialize = "9")]
    N9 = 9,
    #[strum(serialize = "10")]
    N10 = 10,
    J = 11,
    Q = 12,
    K = 13,
    A = 14,
}
