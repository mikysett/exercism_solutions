use std::{collections::HashMap, vec};

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
    words: HashMap<usize, Word>,
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
        let (words, words_table) = make_builtin_words();

        Self {
            stack: vec![],
            words,
            words_table,
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        let mut tokens = input.split_whitespace().map(|el| el.to_uppercase());

        while let Some(token) = tokens.next() {
            match Token::new(&token, &self.words_table)? {
                Token::Number(nb) => self.stack.push(nb),
                Token::Word(id) => {
                    let word = self.expand_word(id);

                    let exp = word.expanded.unwrap();
                    (word.op)(self, &exp)?;
                }
                Token::WordStart => {
                    let (name, new_word) = Word::new(self.get_words_table(), &mut tokens)?;
                    self.add_word(&name, new_word);
                }
                _ => unreachable!(),
            }
        }
        Ok(())
    }

    fn expand_word(&mut self, id: usize) -> Word {
        let word = self.get_word(id).unwrap().clone();

        match word.expanded.is_some() {
            true => word,
            false => {
                let expand = self.make_expand(&word.tail);

                let word = self.get_mut_word(id).unwrap();
                word.expanded = Some(expand);

                word.clone()
            }
        }
    }

    fn make_expand(&mut self, tail: &[Token]) -> String {
        let mut result = String::new();

        for token in tail {
            match token {
                Token::Number(nb) => {
                    result.push(' ');
                    result.push_str(&nb.to_string());
                }
                Token::Word(id) => {
                    let word = self.get_word(*id).unwrap().clone();

                    let expanded = word
                        .expanded
                        .unwrap_or(self.expand_word(*id).expanded.unwrap());

                    result.push(' ');
                    result.push_str(&expanded);
                }
                _ => unreachable!(),
            }
        }

        result
    }

    fn get_words_table(&self) -> &HashMap<String, usize> {
        &self.words_table
    }

    fn add_word(&mut self, name: &str, new_word: Word) {
        let id = self.words.len();

        self.words_table.insert(name.to_owned(), id);
        self.words.insert(id, new_word);
    }

    pub fn get_word(&mut self, id: usize) -> Option<&Word> {
        self.words.get(&id)
    }

    pub fn get_mut_word(&mut self, id: usize) -> Option<&mut Word> {
        self.words.get_mut(&id)
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
