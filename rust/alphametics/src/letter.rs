use std::collections::HashMap;

#[derive(Debug)]
pub struct Letter {
    pub c: char,
    pub available_vals: Vec<u8>,
    pub val: u8,
    pub weight: u64,
}

impl Clone for Letter {
    fn clone(&self) -> Self {
        Self {
            c: self.c,
            available_vals: self.available_vals.to_vec(),
            val: self.val,
            weight: self.weight,
        }
    }
}

impl Letter {
    pub fn new(c: char) -> Self {
        Self {
            c,
            available_vals: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            val: 0,
            weight: 0,
        }
    }

    pub fn remove_possible(&mut self, nb: u8) {
        self.available_vals.retain(|v| *v != nb);
    }

    pub fn set_possible_val(input: &str, numbers: &[&str], letters: &mut HashMap<char, Letter>) {
        Self::set_weights(numbers, letters);

        input.split(' ').for_each(|word| {
            let first_letter = word.chars().next().unwrap();
            if first_letter.is_alphabetic() {
                letters.get_mut(&first_letter).unwrap().remove_possible(0);
            }
        });
    }

    fn set_weights(numbers: &[&str], letters: &mut HashMap<char, Letter>) {
        numbers.iter().for_each(|nb| {
            nb.chars().rev().enumerate().for_each(|(i, c)| {
                letters
                    .entry(c)
                    .and_modify(|letter| letter.weight += one_exp(i));
            })
        });
    }

    pub fn optimized(
        numbers: &[&str],
        result: &str,
        letters: &mut HashMap<char, Letter>,
    ) -> Vec<Letter> {
        let mut letters_opt = letters
            .iter()
            .map(|(_, letter)| letter.clone())
            .collect::<Vec<Letter>>();
        letters_opt.sort_unstable_by(|a, b| b.weight.cmp(&a.weight));

        let res_len = result.len();
        let num_max_len = numbers.iter().map(|nb| nb.len()).max().unwrap();

        let solution_max = letters_opt
            .iter()
            .zip((0..=9).rev())
            .map(|(letter, val)| {
                let mut new_letter = letter.clone();
                new_letter.val = val as u8;
                new_letter
            })
            .collect::<Vec<Letter>>();

        let total_max: u64 = numbers
            .iter()
            .map(|number| Letter::convert_to_u64(number, &solution_max))
            .sum();

        let mut first_l_result = letters_opt
            .iter_mut()
            .filter(|l| l.c == result.chars().next().unwrap())
            .take(1)
            .next()
            .unwrap();
        if res_len >= num_max_len {
            if nb_digits(&total_max) == result.len() {
                first_l_result.available_vals = (1..=first_digit(&total_max)).collect();
            }
        } else if res_len < num_max_len {
            first_l_result.available_vals = vec![]
        };

        letters_opt.sort_unstable_by(|a, b| a.available_vals.len().cmp(&b.available_vals.len()));
        letters_opt
    }

    pub fn convert_to_u64(s: &str, solution: &[Letter]) -> u64 {
        s.chars()
            .map(|c| {
                solution
                    .iter()
                    .filter(|l| l.c == c)
                    .take(1)
                    .next()
                    .map(|l| l.val)
                    .unwrap() as u64
            })
            .fold(0, |total, digit| total * 10 + digit)
    }
}

fn one_exp(n: usize) -> u64 {
    let mut nb = 1;
    for _ in 0..n {
        nb *= 10;
    }
    nb
}

fn nb_digits(nb: &u64) -> usize {
    let mut nb = *nb;
    let mut nb_digits = 0;

    while nb > 0 {
        nb /= 10;
        nb_digits += 1;
    }
    nb_digits
}

fn first_digit(nb: &u64) -> u8 {
    let mut nb = *nb;

    while nb > 9 {
        nb /= 10;
    }
    nb as u8
}
