pub fn reply(message: &str) -> &str {
    let m = message.trim();
    let is_yelling = || {
        m.chars().any(|c| c.is_alphabetic())
            && m.chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    };

    match (m.ends_with('?'), is_yelling(), m.is_empty()) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (_, _, _) => "Whatever.",
    }
}
