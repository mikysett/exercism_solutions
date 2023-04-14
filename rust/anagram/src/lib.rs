use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let sort_str = |s: &str| {
        let mut sorted = s.chars().collect::<Vec<char>>();
        sorted.sort_unstable();
        sorted
    };
    let word = word.to_lowercase();
    let sorted_word = sort_str(&word);
    possible_anagrams
        .iter()
        .filter(|anagram| {
            let anagram = anagram.to_lowercase();
            word != anagram && sorted_word == sort_str(&anagram)
        })
        .copied()
        .collect()
}
