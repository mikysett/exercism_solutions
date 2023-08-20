pub fn reply(message: &str) -> &str {
    let message = message.trim();
    let is_question = || message.chars().last().unwrap_or(' ') == '?';
    let is_yelling = || {
        message.chars().any(|c| c.is_alphabetic())
            && message
                .chars()
                .filter(|c| c.is_alphabetic())
                .all(|c| c.is_uppercase())
    };
    let is_silence = || message.is_empty();

    match (is_question(), is_yelling(), is_silence()) {
        (_, _, true) => "Fine. Be that way!",
        (true, true, _) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, _) => "Whoa, chill out!",
        (_, _, _) => "Whatever.",
    }
}
