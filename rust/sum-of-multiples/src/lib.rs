use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .iter()
        .filter(|&base| *base > 0)
        .flat_map(|&base| (base..limit).filter(move |&nb| nb % base == 0))
        .collect::<HashSet<u32>>()
        .iter()
        .sum()
}
