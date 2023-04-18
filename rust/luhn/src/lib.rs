/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    if code.trim().len() <= 1 {
        return false;
    }
    code.as_bytes()
        .iter()
        .rev()
        .filter(|&c| *c != b' ')
        .enumerate()
        .try_fold(0, |acc, (i, &c)| {
            if c.is_ascii_digit() {
                Some(if i % 2 == 1 {
                    match c - b'0' + c - b'0' {
                        sum if sum > 9 => acc + sum - 9,
                        other => acc + other,
                    }
                } else {
                    acc + c - b'0'
                })
            } else {
                None
            }
        })
        .map_or(false, |checksum| checksum % 10 == 0)
}
