use std::vec;

mod builtin;
mod token;
mod word;

use builtin::make_builtin_words;
use token::Token;
use word::Word;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: Vec<Word>,
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
        Self {
            stack: vec![],
            words: make_builtin_words(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace().map(|el| el.to_uppercase());

        while let Some(token) = tokens.next() {
            match Token::new(&token, &self.words)? {
                Token::Number(nb) => self.stack.push(nb),
                Token::Word => match Word::get_copy_with_index(&self.words, &token) {
                    Some((i, word)) => {
                        // if word.expanded.is_none() {
                        //     word.expand()?;
                        // }

                        let original_words = self.words.clone();
                        self.words = self.words[0..i].to_owned();

                        let result = (word.op)(self, &word.tail);

                        self.words = original_words;
                        result?;
                    }
                    None => return Err(Error::UnknownWord),
                },
                Token::WordStart => {
                    let new_word = Word::new(self.get_words(), &mut tokens)?;
                    self.add_word(new_word);
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn expand_word(&mut self, index: usize) -> String {
        unimplemented!()
    }

    fn get_words(&self) -> &[Word] {
        &self.words
    }

    fn add_word(&mut self, new_word: Word) {
        self.words.push(new_word);
    }

    fn stack_pop(&mut self) -> std::result::Result<Value, Error> {
        match self.stack.pop() {
            Some(el) => Ok(el),
            None => Err(Error::StackUnderflow),
        }
    }

    fn stack_last(&mut self) -> std::result::Result<Value, Error> {
        match self.stack.last() {
            Some(el) => Ok(*el),
            None => Err(Error::StackUnderflow),
        }
    }
}
