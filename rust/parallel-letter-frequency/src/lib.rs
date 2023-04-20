use std::sync::mpsc;
use std::thread;
use std::collections::HashMap;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut freq = HashMap::new();
    input
        .iter()
        .flat_map(|line| {
            line.chars()
                .filter(|c| c.is_alphabetic())
                .flat_map(|c| c.to_lowercase())
        })
        .for_each(|c| {
            freq.entry(c)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        });
    freq
}
