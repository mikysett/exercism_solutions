use std::{collections::HashMap, vec};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: Vec<Word>,
}

type WordOp = fn(&mut Forth, &str) -> Result;

struct Word {
    head: String,
    tail: String,
    op: WordOp,
}

impl Word {
    fn new_std(name: &str, op: WordOp) -> Self {
        Self {
            head: name.to_owned(),
            tail: name.to_owned(),
            op,
        }
    }

    fn new(
        words: &[Word],
        tokens: &mut impl Iterator<Item = String>,
    ) -> std::result::Result<Self, Error> {
        let name = tokens.next().ok_or(Error::InvalidWord)?;

        if !Word::is_valid_name(&name) {
            return Err(Error::InvalidWord);
        }

        let mut tail = String::new();
        for t in tokens.by_ref() {
            match Token::new(&t, words)? {
                Token::WordEnd => {
                    return {
                        Ok(
                            Self {
                                head: name,
                                tail,
                                op: Forth::eval,
                            },
                        )
                    }
                }
                Token::WordStart => return Err(Error::InvalidWord),
                Token::Word(found_expand) => {
                    tail.push(' ');
                    tail.push_str(&found_expand);
                }
                Token::Number(_) => {
                    tail.push(' ');
                    tail.push_str(&t);
                }
            }
        }
        Err(Error::InvalidWord)
    }

    fn get<'a>(words: &'a [Word], name: &'a str) -> Option<&'a Word> {
        words.iter().rev().find(|word| word.head == name)
    }

    fn is_valid_name(s: &str) -> bool {
        !(s.chars().all(|c| c.is_numeric()) || s == ";" || s == ":")
    }
}

const ARITHMETIC_OPS: [&str; 4] = ["+", "-", "*", "/"];
const STACK_OPS: [&str; 4] = ["DUP", "DROP", "SWAP", "OVER"];

enum Token {
    Number(Value),
    WordStart,
    Word(String),
    WordEnd,
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
                Token::Word(_) => {
                    match Word::get(&self.words, &token)
                        .map(|word| (word.op, word.tail.to_string()))
                    {
                        Some((op, expand)) => (op)(self, &expand)?,
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

impl Token {
    fn new(s: &str, words: &[Word]) -> std::result::Result<Self, Error> {
        if let Ok(nb) = s.parse::<Value>() {
            Ok(Token::Number(nb))
        } else if let Some(word) = Word::get(words, s) {
            Ok(Token::Word(word.tail.to_owned()))
        } else if s == ":" {
            Ok(Token::WordStart)
        } else if s == ";" {
            Ok(Token::WordEnd)
        } else {
            Err(Error::UnknownWord)
        }
    }
}

fn print_words(words: &[Word]) {
    for word in words {
        println!("{}: {}", word.head, word.tail);
    }
}
