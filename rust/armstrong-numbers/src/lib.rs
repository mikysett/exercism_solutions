pub fn is_armstrong_number(num: u32) -> bool {
    let nb_digits = nb_digits(num);
    let mut new_num = num;
    let mut armstrong: u32 = 0;

    while new_num != 0 {
        let digit = new_num % 10;
        match armstrong.checked_add(digit.pow(nb_digits)) {
            Some(nb) => armstrong = nb,
            None => return false,
        }

        new_num /= 10;
    }
    armstrong == num
}

fn nb_digits(mut num: u32) -> u32 {
    let mut nb_digits = 0;

    while num != 0 {
        nb_digits += 1;
        num /= 10;
    }
    nb_digits
}

