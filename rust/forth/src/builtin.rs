use crate::Error;
use crate::Forth;
use crate::Result;
use crate::Value;

pub fn do_add(stack: &mut Vec<Value>) -> Result {
    let rhs = Forth::stack_pop(stack)?;
    let lhs = Forth::stack_pop(stack)?;

    stack.push(lhs + rhs);
    Ok(())
}

pub fn do_sub(stack: &mut Vec<Value>) -> Result {
    let rhs = Forth::stack_pop(stack)?;
    let lhs = Forth::stack_pop(stack)?;

    stack.push(lhs - rhs);
    Ok(())
}

pub fn do_mult(stack: &mut Vec<Value>) -> Result {
    let rhs = Forth::stack_pop(stack)?;
    let lhs = Forth::stack_pop(stack)?;

    stack.push(lhs * rhs);
    Ok(())
}

pub fn do_div(stack: &mut Vec<Value>) -> Result {
    let rhs = Forth::stack_pop(stack)?;
    let lhs = Forth::stack_pop(stack)?;

    stack.push(lhs.checked_div(rhs).ok_or(Error::DivisionByZero)?);
    Ok(())
}

pub fn do_dup(stack: &mut Vec<Value>) -> Result {
    let last = Forth::stack_last(stack)?;
    stack.push(last);
    Ok(())
}

pub fn do_drop(stack: &mut Vec<Value>) -> Result {
    Forth::stack_pop(stack)?;
    Ok(())
}

pub fn do_swap(stack: &mut Vec<Value>) -> Result {
    let rhs = Forth::stack_pop(stack)?;
    let lhs = Forth::stack_pop(stack)?;

    stack.push(rhs);
    stack.push(lhs);
    Ok(())
}

pub fn do_over(stack: &mut Vec<Value>) -> Result {
    if stack.len() < 2 {
        return Err(Error::StackUnderflow);
    }

    let before_last = stack[stack.len() - 2];
    stack.push(before_last);
    Ok(())
}
