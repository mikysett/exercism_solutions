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
    last_word: usize,
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
        let builtin_words = make_builtin_words();

        Self {
            stack: vec![],
            last_word: builtin_words.len(),
            words: builtin_words,
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
                Token::Word => {
                    match Word::get_copy_with_index(&self.words[0..self.last_word], &token) {
                        Some((i, word)) => {
                            let original_last = self.last_word;

                            self.last_word = i;
                            let result = (word.op)(self, &word.tail);
                            self.last_word = original_last;

                            result?;
                        }
                        None => return Err(Error::UnknownWord),
                    }
                }
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
        self.last_word += 1;
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
