use std::collections::HashMap;

use crate::Error;
use crate::Forth;
use crate::Result;
use crate::Token;

pub type WordOp = fn(&'_ mut Forth, Token) -> Result;

#[derive(Clone)]
pub struct Word {
    pub tail: Vec<Token>,
    pub op: WordOp,
    pub expanded: Option<Vec<Token>>,
}

impl Word {
    pub fn new_std(id: usize, op: WordOp) -> Self {
        Self {
            tail: vec![],
            expanded: Some(vec![Token::Word(id)]),
            op,
        }
    }

    pub fn new(
        words_table: &HashMap<String, usize>,
        tokens: &mut impl Iterator<Item = String>,
    ) -> std::result::Result<(String, Self), Error> {
        let head = tokens.next().ok_or(Error::InvalidWord)?;

        if !Word::is_valid_name(&head) {
            return Err(Error::InvalidWord);
        }

        let mut tail = vec![];
        for t in tokens.by_ref() {
            match Token::new(&t, words_table)? {
                Token::WordStart => return Err(Error::InvalidWord),
                Token::WordEnd => {
                    return Ok((
                        head,
                        Self {
                            tail,
                            op: Forth::exec,
                            expanded: None,
                        },
                    ));
                }
                other => {
                    tail.push(other);
                }
            }
        }
        Err(Error::InvalidWord)
    }

    pub fn get_id<'a>(words_table: &'a HashMap<String, usize>, name: &str) -> Option<&'a usize> {
        words_table.get(name)
    }

    fn is_valid_name(s: &str) -> bool {
        !(s.chars().all(|c| c.is_numeric()) || s == ";" || s == ":")
    }
}
