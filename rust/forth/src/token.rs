use crate::word::WordCallback;
use crate::Error;
use crate::Forth;
use crate::Value;

pub enum Token {
    Number(Value),
    WordStart,
    Call(usize),
    BuiltinCall(WordCallback),
    WordEnd,
}

impl Token {
    pub fn new(s: &str, forth: &Forth) -> std::result::Result<Self, Error> {
        if let Ok(nb) = s.parse::<Value>() {
            Ok(Token::Number(nb))
        } else if s == ":" {
            Ok(Token::WordStart)
        } else if s == ";" {
            Ok(Token::WordEnd)
        } else {
            forth
                .get_word_id(s)
                .map(Token::Call)
                .ok_or(Error::UnknownWord)
        }
    }
}
