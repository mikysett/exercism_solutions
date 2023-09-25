/// `Palindrome` is a newtype which only exists when the contained value is a palindrome number in base ten.
///
/// A struct with a single field which is used to constrain behavior like this is called a "newtype", and its use is
/// often referred to as the "newtype pattern". This is a fairly common pattern in Rust.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Palindrome(u64);

impl Palindrome {
    /// Create a `Palindrome` only if `value` is in fact a palindrome when represented in base ten. Otherwise, `None`.
    pub fn new(value: u64) -> Option<Palindrome> {
        if value % 2 == 0 && value % 11 != 0 {
            return None;
        }
        (value.to_string() == value.to_string().chars().rev().collect::<String>())
            .then_some(Self(value))
    }

    /// Get the value of this palindrome.
    pub fn into_inner(self) -> u64 {
        self.0
    }
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    let mut numbers: Vec<Palindrome> = (min..=max)
        .into_iter()
        .flat_map(|nb| (nb..=max).into_iter().filter_map(move |nb2| Palindrome::new(nb * nb2)))
        .collect();
    numbers.sort_unstable();

    match numbers.is_empty() {
        true => None,
        _ => Some((*numbers.first().unwrap(), *numbers.last().unwrap()))
    }
}
