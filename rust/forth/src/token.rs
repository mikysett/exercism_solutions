use crate::Error;
use crate::Value;
use crate::Word;

pub enum Token {
    Number(Value),
    WordStart,
    Word,
    WordEnd,
}

impl Token {
    pub fn new(s: &str, words: &[Word]) -> std::result::Result<Self, Error> {
        if let Ok(nb) = s.parse::<Value>() {
            Ok(Token::Number(nb))
        } else if Word::get(words, s).is_some() {
            Ok(Token::Word)
        } else if s == ":" {
            Ok(Token::WordStart)
        } else if s == ";" {
            Ok(Token::WordEnd)
        } else {
            Err(Error::UnknownWord)
        }
    }
}
