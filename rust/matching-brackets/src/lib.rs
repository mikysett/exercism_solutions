pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];

    for c in string.chars() {
        match c {
            _ if c == '{' => stack.push('}'),
            _ if c == '[' => stack.push(']'),
            _ if c == '(' => stack.push(')'),
            '}' | ']' | ')' => {
                if stack.pop() != Some(c) {
                    return false;
                }
            }
            _ => (),
        }
    }
    stack.is_empty()
}
