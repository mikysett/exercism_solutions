use std::collections::{hash_map::Entry, HashMap};

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !"ACGT".contains(nucleotide) {
        return Err(nucleotide);
    }
    nucleotide_counts(dna).map(|counts| *counts.get(&nucleotide).unwrap())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    let mut counts = HashMap::new();

    "ACGT".chars().for_each(|c| {
        counts.insert(c, 0);
    });

    dna.chars()
        .try_for_each(|c| match counts.entry(c).and_modify(|e| *e += 1) {
            Entry::Occupied(_) => Ok(()),
            Entry::Vacant(_) => Err(c),
        })
        .map(|_| counts)
}
