pub fn abbreviate(phrase: &str) -> String {
    let mut last_char = ' ';

    phrase
        .chars()
        .filter_map(|c| {
            let acronym = if c.is_alphabetic() {
                match last_char {
                    _ if [' ', '-', '_'].contains(&last_char)
                        || (last_char.is_lowercase() && c.is_uppercase()) =>
                    {
                        Some(c.to_ascii_uppercase())
                    }
                    _ => None,
                }
            } else {
                None
            };
            last_char = c;

            acronym
        })
        .collect()
}
