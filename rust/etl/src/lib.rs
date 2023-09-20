use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut score_per_letter = BTreeMap::new();

    for (k, v) in h {
        for letter in v {
            score_per_letter.insert((*letter).to_ascii_lowercase(), *k);
        }
    }
    score_per_letter
}
