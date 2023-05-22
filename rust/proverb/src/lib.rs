use std::iter::once;

pub fn build_proverb(list: &[&str]) -> String {
    match list.is_empty() {
        true => String::new(),
        false => list
            .windows(2)
            .map(|words| format!("For want of a {} the {} was lost.\n", words[0], words[1]))
            .chain(once(format!(
                "And all for the want of a {}.",
                list.first().unwrap()
            )))
            .collect(),
    }
}
