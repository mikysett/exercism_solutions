use std::ops::Deref;

use crate::Error;
use crate::Forth;
use crate::Result;
use crate::Token;
use crate::Value;

pub type WordCallback = fn(&'_ mut Vec<Value>) -> Result;

pub struct Word(pub Vec<Token>);

impl Deref for Word {
    type Target = Vec<Token>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Word {
    pub fn new_std(op: WordCallback) -> Self {
        Self(vec![Token::BuiltinCall(op)])
    }

    pub fn new(
        forth: &Forth,
        tokens: &mut impl Iterator<Item = String>,
    ) -> std::result::Result<(String, Self), Error> {
        let head = tokens.next().ok_or(Error::InvalidWord)?;

        if !Word::is_valid_name(&head) {
            return Err(Error::InvalidWord);
        }

        let mut tail = vec![];
        for t in tokens.by_ref() {
            match Token::new(&t, forth)? {
                Token::WordStart => return Err(Error::InvalidWord),
                Token::WordEnd => {
                    return Ok((head, Self(tail)));
                }
                other => {
                    tail.push(other);
                }
            }
        }
        Err(Error::InvalidWord)
    }

    fn is_valid_name(s: &str) -> bool {
        !(s.chars().all(|c| c.is_numeric()) || s == ";" || s == ":")
    }
}
