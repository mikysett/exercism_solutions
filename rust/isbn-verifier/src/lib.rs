/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let digits: Vec<u32> = isbn
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10).or((c == 'X').then_some(10)))
        .collect();

    if digits.len() != 10 || digits.iter().skip(1).any(|x| *x == 10) {
        return false;
    }

    digits
        .iter()
        .enumerate()
        .map(|(i, d)| d * (i as u32 + 1))
        .sum::<u32>()
        % 11
        == 0
}
