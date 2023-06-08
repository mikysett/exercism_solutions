use crate::word::Word;
use crate::Error;
use crate::Forth;
use crate::Result;

const ARITHMETIC_OPS: [&str; 4] = ["+", "-", "*", "/"];
const STACK_OPS: [&str; 4] = ["DUP", "DROP", "SWAP", "OVER"];

pub fn make_builtin_words() -> Vec<Word> {
    let arithmetic_word = |op: String| Word::new_std(&op, do_arithmetic_op);
    let builtin_word = |op: String| Word::new_std(&op, do_stack_op);

    let mut std_words = vec![];

    ARITHMETIC_OPS.iter().for_each(|op| {
        std_words.push(arithmetic_word(op.to_string()));
    });
    STACK_OPS.iter().for_each(|op| {
        std_words.push(builtin_word(op.to_string()));
    });

    std_words
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
