use std::collections::HashMap;

use crate::Error;
use crate::Value;
use crate::Word;

#[derive(Debug, Clone)]
pub enum Token {
    Number(Value),
    WordStart,
    Word(usize),
    WordEnd,
}

impl Token {
    pub fn new(s: &str, words_table: &HashMap<String, usize>) -> std::result::Result<Self, Error> {
        if let Ok(nb) = s.parse::<Value>() {
            Ok(Token::Number(nb))
        } else if s == ":" {
            Ok(Token::WordStart)
        } else if s == ";" {
            Ok(Token::WordEnd)
        } else {
            match Word::get_id(words_table, s) {
                Some(id) => Ok(Token::Word(*id)),
                None => Err(Error::UnknownWord),
            }
        }
    }
}
