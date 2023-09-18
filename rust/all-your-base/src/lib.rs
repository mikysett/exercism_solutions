#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

///
/// Convert a number between two bases.
///
/// A number is any slice of digits.
/// A digit is any unsigned integer (e.g. u8, u16, u32, u64, or usize).
/// Bases are specified as unsigned integers.
///
/// Return an `Err(.)` if the conversion is impossible.
/// The tests do not test for specific values inside the `Err(.)`.
///
///
/// You are allowed to change the function signature as long as all test still pass.
///
///
/// Example:
/// Input
///   number: &[4, 2]
///   from_base: 10
///   to_base: 2
/// Result
///   Ok(vec![1, 0, 1, 0, 1, 0])
///
/// The example corresponds to converting the number 42 from decimal
/// which is equivalent to 101010 in binary.
///
///
/// Notes:
///  * The empty slice ( "[]" ) is equal to the number 0.
///  * Never output leading 0 digits, unless the input number is 0, in which the output must be `[0]`.
///    However, your function must be able to process input with leading 0 digits.
///
pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base <= 1 {
        return Err(Error::InvalidInputBase);
    } else if to_base <= 1 {
        return Err(Error::InvalidOutputBase);
    } else if let Some(nb) = number.iter().find(|&&nb| nb >= from_base) {
        return Err(Error::InvalidDigit(*nb));
    }

    let mut nb_base_10: u32 = number
        .iter()
        .rev()
        .enumerate()
        .map(|(i, nb)| nb * from_base.pow(i as u32))
        .sum();
    if nb_base_10 == 0 {
        return Ok(vec![0]);
    }

    let mut res = vec![];
    let mut base_position = 1;
    loop {
        let cur_base = to_base.pow(base_position);
        let digit = nb_base_10 % cur_base;
        
        res.push(digit / to_base.pow(base_position - 1));
        
        base_position += 1;
        if cur_base > nb_base_10 {
            break;
        }
    }

    res.reverse();
    Ok(res)
}
