use std::vec;

mod token;
mod word;
use token::Token;
use word::Word;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: Vec<Word>,
}

const ARITHMETIC_OPS: [&str; 4] = ["+", "-", "*", "/"];
const STACK_OPS: [&str; 4] = ["DUP", "DROP", "SWAP", "OVER"];

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        let arithmetic_word = |op: String| {
            Word::new_std(&op, |forth: &mut Self, op: &str| forth.do_arithmetic_op(op))
        };
        let builtin_word =
            |op: String| Word::new_std(&op, |forth: &mut Self, op: &str| forth.do_stack_op(op));

        let mut std_words = vec![];

        ARITHMETIC_OPS.iter().for_each(|op| {
            std_words.push(arithmetic_word(op.to_string()));
        });
        STACK_OPS.iter().for_each(|op| {
            std_words.push(builtin_word(op.to_string()));
        });

        Self {
            stack: vec![],
            words: std_words,
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

    fn do_arithmetic_op(&mut self, op: &str) -> Result {
        let rhs = self.stack_pop()?;
        let lhs = self.stack_pop()?;

        self.stack.push(match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?,
            _ => unreachable!(),
        });
        Ok(())
    }

    fn do_stack_op(&mut self, op: &str) -> Result {
        match op {
            "DUP" => {
                let last = self.stack_last()?;
                self.stack.push(last);
            }
            "DROP" => {
                self.stack_pop()?;
            }
            "SWAP" => {
                let rhs = self.stack_pop()?;
                let lhs = self.stack_pop()?;

                self.stack.push(rhs);
                self.stack.push(lhs);
            }
            "OVER" => {
                if self.stack.len() < 2 {
                    return Err(Error::StackUnderflow);
                }

                let before_last = self.stack[self.stack.len() - 2];
                self.stack.push(before_last);
            }
            _ => unreachable!(),
        }
        Ok(())
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

    fn get_words(&self) -> &[Word] {
        &self.words
    }

    fn add_word(&mut self, new_word: Word) {
        self.words.push(new_word);
    }
}
