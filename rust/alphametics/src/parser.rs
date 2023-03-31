use std::collections::HashMap;

use crate::letter::Letter;

pub fn parse(input: &str) -> (HashMap<char, Letter>, Vec<&str>, &str) {
    let (numbers, result) = input.split_once(" == ").unwrap();
    let numbers = numbers.split(" + ").collect::<Vec<&str>>();

    let letters = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c, Letter::new(c)))
        .collect::<HashMap<char, Letter>>();

    (letters, numbers, result)
}
