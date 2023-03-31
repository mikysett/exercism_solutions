pub mod letter;
pub mod parser;

use crate::letter::Letter;
use crate::parser::*;
use std::collections::HashMap;

const POSSIBLE_DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (mut letters, numbers, result) = parse(input);
    Letter::set_possible_val(input, &numbers, &mut letters);
    let letters = Letter::optimized(&numbers, result, &mut letters);

    solve_sgl_char(&letters, 0, &POSSIBLE_DIGITS, &numbers, result)
}

fn solve_sgl_char(
    letters: &Vec<Letter>,
    letter_index: usize,
    poss_digits: &[u8],
    numbers: &[&str],
    result: &str,
) -> Option<HashMap<char, u8>> {
    if letter_index == letters.len() {
        return match is_solution_valid(numbers, result, letters) {
            true => Some(
                letters
                    .iter()
                    .map(|letter| (letter.c, letter.val))
                    .collect(),
            ),
            false => None,
        };
    }

    let mut new_letters = letters.clone();
    for try_nb in poss_digits.iter() {
        let curr_letter = new_letters.get_mut(letter_index).unwrap();
        if !curr_letter.available_vals.contains(try_nb) {
            continue;
        }

        curr_letter.val = *try_nb;
        let new_poss_digits = poss_digits
            .iter()
            .filter(|nb| *nb != try_nb)
            .copied()
            .collect::<Vec<u8>>();

        if let Some(solution) = solve_sgl_char(
            &new_letters,
            letter_index + 1,
            &new_poss_digits,
            numbers,
            result,
        ) {
            return Some(solution);
        }
    }
    None
}

fn is_solution_valid(numbers: &[&str], result: &str, solution: &[Letter]) -> bool {
    let total: u64 = numbers
        .iter()
        .map(|number| Letter::convert_to_u64(number, solution))
        .sum();

    total == Letter::convert_to_u64(result, solution)
}
