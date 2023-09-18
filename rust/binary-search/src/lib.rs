use std::cmp::Ordering;

pub fn find<T: Ord>(array: &[T], key: T) -> Option<usize> {
    let half = array.len() / 2;

    match array.get(half)?.cmp(&key) {
        Ordering::Equal => Some(half),
        Ordering::Less => find(array.get(half + 1..)?, key).map(|i| i + half + 1),
        Ordering::Greater => find(array.get(..half)?, key),
    }
}
