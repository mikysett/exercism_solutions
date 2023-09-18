use std::cmp::Ordering;

pub fn find<T>(array: &[T], key: T) -> Option<usize>
where
    T: Ord,
{
    binary_find(array, &key, 0)
}

fn binary_find<T>(array: &[T], key: &T, offset: usize) -> Option<usize>
where
    T: Ord,
{
    if array.is_empty() {
        return None;
    }
    let half = array.len() / 2;

    match array.get(half)?.cmp(key) {
        Ordering::Equal => Some(offset + half),
        Ordering::Less => binary_find(array.get(half + 1..)?, key, offset + half + 1),
        Ordering::Greater => binary_find(array.get(..half)?, key, offset),
    }
}
