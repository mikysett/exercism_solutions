use std::cmp::Ordering;

pub fn find<T>(array: &[T], key: T) -> Option<usize>
where
    T: Ord,
{
    let half = array.len() / 2;
    
    match array.get(half)?.cmp(&key) {
        Ordering::Equal => Some(half),
        Ordering::Less => find(array.get(half + 1..)?, key).map(|i| i + half + 1),
        Ordering::Greater => find(array.get(..half)?, key),
    }
}
