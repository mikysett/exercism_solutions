/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    sentence
        .chars()
        .filter_map(|c| c.is_ascii_alphabetic().then(|| c.to_ascii_lowercase()))
        .fold(0, |letters_found: i32, c| {
            letters_found | 1 << (c as u8 % 26)
        })
        .count_ones()
        == 26
}
