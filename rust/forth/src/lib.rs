use std::{collections::HashMap, vec};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, Word>,
}

type WordOp = fn(&mut Forth, &str) -> Result;

struct Word {
    name: String,
    op: WordOp,
    // expand: String,
}

impl Word {
    fn new(name: &str, op: WordOp) -> Self {
        Self {
            name: name.to_owned(),
            op,
        }
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
        let arithmetic_word =
            |op: String| Word::new(&op, |forth: &mut Self, op: &str| forth.do_arithmetic_op(op));
        let builtin_word =
            |op: String| Word::new(&op, |forth: &mut Self, op: &str| forth.do_stack_op(op));

        let mut std_words = HashMap::new();

        ARITHMETIC_OPS.iter().for_each(|op| {
            std_words.insert(op.to_string(), arithmetic_word(op.to_string()));
        });
        STACK_OPS.iter().for_each(|op| {
            std_words.insert(op.to_string(), builtin_word(op.to_string()));
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
        input.split_whitespace().try_for_each(|s| {
            let s = s.to_uppercase();

            match Token::new(&s, &self.words)? {
                Token::Number(nb) => self.stack.push(nb),
                Token::Word(name) => {
                    let (word_op, expanded) = self
                        .words
                        .get(&name)
                        .map(|word| (word.op, word.name.to_string()))
                        .unwrap();
                    (word_op)(self, &expanded)?;
                }
                _ => unreachable!(),
            }
            Ok(())
        })
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
}

impl Token {
    fn new(s: &str, words: &HashMap<String, Word>) -> std::result::Result<Self, Error> {
        if let Ok(nb) = s.parse::<Value>() {
            Ok(Token::Number(nb))
        } else if words.contains_key(s) {
            Ok(Token::Word(s.to_owned()))
        } else if s == ":" {
            Ok(Token::WordStart)
        } else if s == ";" {
            Ok(Token::WordEnd)
        } else {
            Err(Error::UnknownWord)
        }
    }
}
