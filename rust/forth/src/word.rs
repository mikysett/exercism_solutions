use crate::Error;
use crate::Forth;
use crate::Result;
use crate::Token;

pub type WordOp = fn(&'_ mut Forth, &str) -> Result;

#[derive(Clone)]
pub struct Word {
    pub head: String,
    pub tail: String,
    pub op: WordOp,
    pub expanded: Option<String>,
}

impl Word {
    pub fn new_std(name: &str, op: WordOp) -> Self {
        Self {
            head: name.to_owned(),
            tail: name.to_owned(),
            op,
            expanded: Some(name.to_owned()),
        }
    }

    pub fn new(
        words: &[Word],
        tokens: &mut impl Iterator<Item = String>,
    ) -> std::result::Result<Self, Error> {
        let head = tokens.next().ok_or(Error::InvalidWord)?;

        if !Word::is_valid_name(&head) {
            return Err(Error::InvalidWord);
        }

        let mut tail = String::new();
        for t in tokens.by_ref() {
            match Token::new(&t, words)? {
                Token::WordStart => return Err(Error::InvalidWord),
                Token::WordEnd => {
                    return Ok(Self {
                        head,
                        tail,
                        op: Forth::eval,
                        expanded: None,
                    });
                }
                _ => {
                    tail.push(' ');
                    tail.push_str(&t);
                }
            }
        }
        Err(Error::InvalidWord)
    }

    pub fn get<'a>(words: &'a [Word], name: &'a str) -> Option<&'a Word> {
        words.iter().rev().find(|word| word.head == name)
    }

    pub fn get_copy_with_index(words: &[Word], name: &str) -> Option<(usize, Word)> {
        words
            .iter()
            .enumerate()
            .rfind(|(_, word)| word.head == name)
            .map(|(i, word)| (i, word.clone()))
    }

    fn is_valid_name(s: &str) -> bool {
        !(s.chars().all(|c| c.is_numeric()) || s == ";" || s == ":")
    }
}
