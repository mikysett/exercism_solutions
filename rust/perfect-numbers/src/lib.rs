#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(mut num: u64) -> Option<Classification> {
    match (1..=num / 2)
        .filter(|fact| num % fact == 0)
        .sum::<u64>() {
        _ if num == 0 => None,
        sum if sum == num => Some(Classification::Perfect),
        sum if sum < num => Some(Classification::Deficient),
        _ => Some(Classification::Abundant),
    }
}
