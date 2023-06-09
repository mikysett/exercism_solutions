use std::collections::HashMap;

use crate::token::Token;
use crate::word::Word;
use crate::Error;
use crate::Forth;
use crate::Result;

const BUILTIN_OPS: [&str; 8] = ["+", "-", "*", "/", "DUP", "DROP", "SWAP", "OVER"];

pub fn make_builtin_words() -> (Vec<Word>, HashMap<String, usize>) {
    let mut words = vec![];
    let mut words_table = HashMap::new();

    let mut add_word = |name: &str, call| {
        let id = words.len();
        let new_word = Word::new_std(id, call);

        words.push(new_word);
        words_table.insert(name.to_owned(), id);
    };

    let calls = [
        do_add, do_sub, do_mult, do_div, do_dup, do_drop, do_swap, do_over,
    ];

    for (name, call) in BUILTIN_OPS.iter().zip(calls.iter()) {
        add_word(name, *call);
    }

    (words, words_table)
}

fn do_add(forth: &mut Forth, _: Token) -> Result {
    let rhs = forth.stack_pop()?;
    let lhs = forth.stack_pop()?;

    forth.stack.push(lhs + rhs);
    Ok(())
}

fn do_sub(forth: &mut Forth, _: Token) -> Result {
    let rhs = forth.stack_pop()?;
    let lhs = forth.stack_pop()?;

    forth.stack.push(lhs - rhs);
    Ok(())
}

fn do_mult(forth: &mut Forth, _: Token) -> Result {
    let rhs = forth.stack_pop()?;
    let lhs = forth.stack_pop()?;

    forth.stack.push(lhs * rhs);
    Ok(())
}

fn do_div(forth: &mut Forth, _: Token) -> Result {
    let rhs = forth.stack_pop()?;
    let lhs = forth.stack_pop()?;

    forth
        .stack
        .push(lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?);
    Ok(())
}

fn do_dup(forth: &mut Forth, _: Token) -> Result {
    let last = forth.stack_last()?;
    forth.stack.push(last);
    Ok(())
}

fn do_drop(forth: &mut Forth, _: Token) -> Result {
    forth.stack_pop()?;
    Ok(())
}

fn do_swap(forth: &mut Forth, _: Token) -> Result {
    let rhs = forth.stack_pop()?;
    let lhs = forth.stack_pop()?;

    forth.stack.push(rhs);
    forth.stack.push(lhs);
    Ok(())
}

fn do_over(forth: &mut Forth, _: Token) -> Result {
    if forth.stack.len() < 2 {
        return Err(Error::StackUnderflow);
    }

    let before_last = forth.stack[forth.stack.len() - 2];
    forth.stack.push(before_last);
    Ok(())
}
