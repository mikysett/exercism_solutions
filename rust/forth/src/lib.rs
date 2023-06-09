use std::{collections::HashMap, vec};

mod builtin;
mod token;
mod word;

use crate::word::WordCallback;
use token::Token;
use word::Word;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

const BUILTIN_OPS: [(&str, WordCallback); 8] = [
    ("+", builtin::do_add),
    ("-", builtin::do_sub),
    ("*", builtin::do_mult),
    ("/", builtin::do_div),
    ("DUP", builtin::do_dup),
    ("DROP", builtin::do_drop),
    ("SWAP", builtin::do_swap),
    ("OVER", builtin::do_over),
];

pub struct Forth {
    stack: Vec<Value>,
    words: Vec<Word>,
    words_table: HashMap<String, usize>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        let mut forth = Self {
            stack: vec![],
            words: vec![],
            words_table: HashMap::new(),
        };

        for (name, callback) in BUILTIN_OPS {
            forth.add_word(name.to_string(), Word::new_std(callback));
        }

        forth
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace().map(|el| el.to_uppercase());

        while let Some(token) = tokens.next() {
            match Token::new(&token, self)? {
                Token::WordStart => {
                    let (name, new_word) = Word::new(self, &mut tokens)?;
                    self.add_word(name, new_word);
                }
                other => Self::exec(&mut self.stack, &self.words, &other)?,
            }
        }
        Ok(())
    }

    fn exec(stack: &mut Vec<Value>, words: &[Word], token: &Token) -> Result {
        match token {
            Token::Number(nb) => stack.push(*nb),
            Token::Call(id) => {
                for token in Forth::get_word(words, *id).unwrap().iter() {
                    Self::exec(stack, words, token)?;
                }
            }
            Token::BuiltinCall(callback) => {
                (callback)(stack)?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }

    fn get_word_id(&self, name: &str) -> Option<usize> {
        self.words_table.get(name).copied()
    }

    fn add_word(&mut self, name: String, new_word: Word) {
        self.words_table.insert(name, self.words.len());
        self.words.push(new_word);
    }

    fn get_word(words: &[Word], id: usize) -> Option<&Word> {
        words.get(id)
    }

    fn stack_pop(stack: &mut Vec<Value>) -> std::result::Result<Value, Error> {
        stack.pop().ok_or(Error::StackUnderflow)
    }

    fn stack_last(stack: &[Value]) -> std::result::Result<Value, Error> {
        stack.last().ok_or(Error::StackUnderflow).copied()
    }
}
