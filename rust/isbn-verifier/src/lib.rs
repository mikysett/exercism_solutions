/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    match isbn
        .chars()
        .rev()
        .filter_map(|c| c.to_digit(10).or((c == 'X').then_some(10)))
        .enumerate()
        .filter(|(i, c)| *c != 10 || *i == 0)
        .fold((0, 0), |a, (i, c)| (a.0 + 1, a.1 + c * (i as u32 + 1)))
    {
        (10, sum) => sum % 11 == 0,
        _ => false,
    }
}
