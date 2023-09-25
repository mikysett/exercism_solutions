use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();

    candidate
        .chars()
        .filter_map(|c| c.is_alphabetic().then(|| c.to_ascii_lowercase()))
        .all(|c| letters.insert(c))
}
