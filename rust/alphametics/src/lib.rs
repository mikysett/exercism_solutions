use std::collections::HashMap;

const POSSIBLE_DIGITS: [u8; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let (numbers, result) = input.split_once(" == ").unwrap();
    let numbers = numbers.split(" + ").collect::<Vec<&str>>();

    let mut letters = input
        .chars()
        .filter(|c| c.is_alphabetic())
        .map(|c| (c, (None, true)))
        .collect::<HashMap<char, (Option<u8>, bool)>>();

    input.split(' ').for_each(|word| {
        let first_letter = word.chars().next().unwrap();
        if first_letter.is_alphabetic() {
            letters.get_mut(&first_letter).unwrap().1 = false;
        }
    });

    solve_sgl_char(&letters, &POSSIBLE_DIGITS, &numbers, result)
}

fn solve_sgl_char(
    letters: &HashMap<char, (Option<u8>, bool)>,
    poss_digits: &[u8],
    numbers: &[&str],
    result: &str,
) -> Option<HashMap<char, u8>> {
    let curr_letter = match letters
        .iter()
        .filter(|(_, (nb, _))| nb.is_none())
        .take(1)
        .next()
    {
        Some((key, _)) => key,
        None => {
            if is_solution_valid(numbers, result, letters) {
                return Some(
                    letters
                        .iter()
                        .map(|(c, (nb, _))| (*c, nb.unwrap()))
                        .collect(),
                );
            } else {
                return None;
            }
        }
    };

    let mut new_unique_c = letters.clone();
    let letter_val = new_unique_c.get_mut(curr_letter).unwrap();

    let skip_invalid_zero = if poss_digits.contains(&0) && !letter_val.1 {
        1
    } else {
        0
    };

    for try_nb in poss_digits.iter().skip(skip_invalid_zero) {
        new_unique_c.get_mut(curr_letter).unwrap().0 = Some(*try_nb);

        let new_poss_digits = poss_digits
            .iter()
            .filter(|nb| *nb != try_nb)
            .copied()
            .collect::<Vec<u8>>();

        if let Some(solution) = solve_sgl_char(&new_unique_c, &new_poss_digits, numbers, result) {
            return Some(solution);
        }
    }
    None
}

fn is_solution_valid(
    numbers: &[&str],
    result: &str,
    solution: &HashMap<char, (Option<u8>, bool)>,
) -> bool {
    let total: u64 = numbers
        .iter()
        .map(|number| convert_to_u64(number, solution))
        .sum();

    total == convert_to_u64(result, solution)
}

fn convert_to_u64(s: &str, solution: &HashMap<char, (Option<u8>, bool)>) -> u64 {
    s.chars()
        .map(|c| solution.get(&c).unwrap().0.unwrap() as u64)
        .fold(0, |total, digit| total * 10 + digit)
}
