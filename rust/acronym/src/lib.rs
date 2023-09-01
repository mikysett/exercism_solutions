use std::iter::once;

pub fn abbreviate(phrase: &str) -> String {
    once(' ')
        .chain(phrase.chars())
        .zip(phrase.chars())
        .filter(|&(prev, cur)| {
            " -_".contains(prev) && cur.is_alphabetic()
                || prev.is_lowercase() && cur.is_uppercase()
        })
        .map(|(_, cur)| cur.to_ascii_uppercase())
        .collect()
}
