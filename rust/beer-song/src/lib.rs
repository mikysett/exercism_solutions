pub fn verse(n: u32) -> String {
    format!(
        "{} of beer on the wall, {} of beer.\n\
    {}, {} of beer on the wall.\n",
        nb_bottle(n, true),
        nb_bottle(n, false),
        action_to_do(n),
        nb_bottle(n.checked_sub(1).unwrap_or(99), false)
    )
}

pub fn nb_bottle(n: u32, phrase_start: bool) -> String {
    match n {
        0 if phrase_start => "No more bottles".to_string(),
        0 => "no more bottles".to_string(),
        1 => "1 bottle".to_string(),
        _ => format!("{n} bottles"),
    }
}

pub fn action_to_do(n: u32) -> &'static str {
    match n {
        0 => "Go to the store and buy some more",
        1 => "Take it down and pass it around",
        _ => "Take one down and pass it around",
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(verse)
        .collect::<Vec<_>>()
        .join("\n")
}
