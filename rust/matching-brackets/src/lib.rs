const OPEN_BRACKETS: [char; 3] = ['{', '[', '('];
const CLOSE_BRACKETS: [char; 3] = ['}', ']', ')'];

pub fn brackets_are_balanced(string: &str) -> bool {
    match string.chars().position(|c| {
        OPEN_BRACKETS
            .iter()
            .zip(CLOSE_BRACKETS.iter())
            .any(|(&open, &close)| open == c || close == c)
    }) {
        Some(i)
            if CLOSE_BRACKETS
                .iter()
                .any(|&b| b == string.chars().nth(i).unwrap()) =>
        {
            false
        }
        Some(i) => check_bracket(i, string)
            .and_then(|last_i| brackets_are_balanced(&string[last_i..]).then_some(()))
            .is_some(),
        None => true,
    }
}

fn check_bracket(mut i: usize, s: &str) -> Option<usize> {
    let open = s.chars().nth(i).unwrap();
    let close = CLOSE_BRACKETS[OPEN_BRACKETS.iter().position(|&b| b == open).unwrap()];

    i += 1;
    while i < s.len() {
        let cur_char = s.chars().nth(i).unwrap();
        if cur_char == close {
            return Some(i + 1);
        } else if CLOSE_BRACKETS.iter().any(|&b| b == cur_char) {
            return None;
        } else if OPEN_BRACKETS.iter().any(|&b| b == cur_char) {
            match check_bracket(i, s) {
                Some(new_i) => i = new_i,
                None => return None,
            }
        } else {
            i += 1;
        }
    }
    None
}
