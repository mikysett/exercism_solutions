use std::vec;

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: Vec<Word>,
}

type WordOp = dyn Fn(&mut Forth, &str) -> Result;

struct Word {
    name: String,
    op: Box<WordOp>,
    // expand: String,
}

impl Word {
    fn new(name: &str, op: Box<WordOp>) -> Self {
        Self { name: name.to_owned(), op }
    }
}

const ARITHMETIC_OPS: [&str; 4] = ["+", "-", "*", "/"];
const STACK_OPS: [&str; 4] = ["DUP", "DROP", "SWAP", "OVER"];

enum Token {
    Number(Value),
    Arithmetic(String),
    StackOp(&'static str),
    WordStart,
    WordName(String),
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
            Word::new(&op, Box::new(|forth: &mut Self, op: &str| Ok(forth.do_arithmetic_op(op)?)))
        };
        Self {
            stack: vec![],
            words: vec![
                arithmetic_word("+".to_string()),
                arithmetic_word("-".to_string()),
                arithmetic_word("*".to_string()),
                arithmetic_word("/".to_string()),
            ],
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> Result {
        input
            .split_whitespace()
            .map(|s| {
                match Token::new(s)? {
                    Token::Number(nb) => self.stack.push(nb),
                    Token::Arithmetic(op) => {
                        self.do_arithmetic_op(&op)?;
                    }
                    Token::StackOp(op) => self.do_stack_op(op)?,
                    _ => unreachable!(),
                }
                Ok(())
            })
            .collect()
    }

    fn do_arithmetic_op(&mut self, op: &str) -> Result {
        let rhs = self.stack_pop()?;
        let lhs = self.stack_pop()?;

        self.stack.push(match op {
            "+" => lhs + rhs,
            "-" => lhs - rhs,
            "*" => lhs * rhs,
            "/" => lhs.checked_div(rhs).ok_or_else(|| Error::DivisionByZero)?,
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
                let before_last = self.stack[self.stack.len() - 2];
                self.stack.push(before_last);
            },
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
    fn new(s: &str) -> std::result::Result<Self, Error> {
        if let Ok(nb) = s.parse::<Value>() {
            Ok(Token::Number(nb))
        } else if ARITHMETIC_OPS.contains(&s) {
            Ok(Token::Arithmetic(s.to_owned()))
        } else if let Some(op) = STACK_OPS
            .iter()
            .find(|&&op| op == s.to_uppercase().as_str())
        {
            Ok(Token::StackOp(op))
        } else {
            Err(Error::InvalidWord)
        }
    }
}
