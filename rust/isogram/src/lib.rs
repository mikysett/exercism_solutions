use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut letters = HashSet::new();

    for c in candidate
        .chars()
        .filter_map(|c| c.is_alphabetic().then(|| c.to_ascii_uppercase()))
    {
        if letters.contains(&c) {
            return false;
        }
        letters.insert(c);
    }
    true
}
