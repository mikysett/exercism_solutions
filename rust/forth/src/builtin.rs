use std::collections::HashMap;

use crate::word::Word;
use crate::Error;
use crate::Forth;
use crate::Result;

const ARITHMETIC_OPS: [&str; 4] = ["+", "-", "*", "/"];
const STACK_OPS: [&str; 4] = ["DUP", "DROP", "SWAP", "OVER"];

pub fn make_builtin_words() -> (HashMap<usize, Word>, HashMap<String, usize>) {
    let mut words = HashMap::new();
    let mut words_table = HashMap::new();

    let mut add_word = |name: &str, call| {
        let id = words.len();
        let new_word = Word::new_std(name, call);

        words.insert(id, new_word);
        words_table.insert(name.to_owned(), id);
    };

    ARITHMETIC_OPS.iter().for_each(|name| {
        add_word(name, do_arithmetic_op);
    });
    STACK_OPS.iter().for_each(|name| {
        add_word(name, do_stack_op);
    });

    (words, words_table)
}

fn do_arithmetic_op(forth: &mut Forth, op: &str) -> Result {
    let rhs = forth.stack_pop()?;
    let lhs = forth.stack_pop()?;

    forth.stack.push(match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?,
        _ => unreachable!(),
    });
    Ok(())
}

fn do_stack_op(forth: &mut Forth, op: &str) -> Result {
    match op {
        "DUP" => {
            let last = forth.stack_last()?;
            forth.stack.push(last);
        }
        "DROP" => {
            forth.stack_pop()?;
        }
        "SWAP" => {
            let rhs = forth.stack_pop()?;
            let lhs = forth.stack_pop()?;

            forth.stack.push(rhs);
            forth.stack.push(lhs);
        }
        "OVER" => {
            if forth.stack.len() < 2 {
                return Err(Error::StackUnderflow);
            }

            let before_last = forth.stack[forth.stack.len() - 2];
            forth.stack.push(before_last);
        }
        _ => unreachable!(),
    }
    Ok(())
}
