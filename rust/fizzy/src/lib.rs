// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

use core::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    sub: String,
}

impl<T> Matcher<T> {
    pub fn new<F, S>(_matcher: F, _subs: S) -> Matcher<T>
    where
        F: Fn(T) -> bool + 'static,
        S: ToString,
    {
        Self {
            matcher: Box::new(_matcher),
            sub: _subs.to_string(),
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T>(Vec<Matcher<T>>);

impl<T> Fizzy<T>
where
    T: ToString + Copy,
{
    pub fn new() -> Self {
        Self(vec![])
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, _matcher: Matcher<T>) -> Self {
        self.0.push(_matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String>
    where
        I: Iterator<Item = T>,
    {
        iter.map(move |el| {
            match self
                .0
                .iter()
                .filter_map(|m| (m.matcher)(el).then_some(m.sub.to_string()))
                .collect::<String>()
            {
                matched if !matched.is_empty() => matched,
                _ => el.to_string(),
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T>() -> Fizzy<T>
where
    T: Rem<Output = T> + ToString + Copy + PartialEq + Default,
    u8: Into<T>,
{
    Fizzy::new()
        .add_matcher(Matcher::new(|nb: T| nb % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|nb: T| nb % 5.into() == 0.into(), "buzz"))
}
