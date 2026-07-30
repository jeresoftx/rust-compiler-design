//! Máquina virtual de pila para el bytecode educativo.
use crate::bytecode::{Bytecode, OpCode};
use std::collections::HashMap;
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VmError {
    StackUnderflow,
    UnknownName(String),
    DivisionByZero,
    MissingReturn,
}
pub fn execute(bytecode: &Bytecode) -> Result<i64, VmError> {
    let mut stack = Vec::new();
    let mut names = HashMap::new();
    for instruction in &bytecode.instructions {
        match instruction {
            OpCode::PushConstant(value) => stack.push(*value),
            OpCode::Load(name) => stack.push(
                *names
                    .get(name)
                    .ok_or_else(|| VmError::UnknownName(name.clone()))?,
            ),
            OpCode::Store(name) => {
                let value = stack.pop().ok_or(VmError::StackUnderflow)?;
                names.insert(name.clone(), value);
            }
            OpCode::Add => binary(&mut stack, |a, b| Ok(a + b))?,
            OpCode::Subtract => binary(&mut stack, |a, b| Ok(a - b))?,
            OpCode::Multiply => binary(&mut stack, |a, b| Ok(a * b))?,
            OpCode::Divide => binary(&mut stack, |a, b| {
                if b == 0 {
                    Err(VmError::DivisionByZero)
                } else {
                    Ok(a / b)
                }
            })?,
            OpCode::Return => return stack.pop().ok_or(VmError::StackUnderflow),
        }
    }
    Err(VmError::MissingReturn)
}
fn binary(
    stack: &mut Vec<i64>,
    op: impl FnOnce(i64, i64) -> Result<i64, VmError>,
) -> Result<(), VmError> {
    let right = stack.pop().ok_or(VmError::StackUnderflow)?;
    let left = stack.pop().ok_or(VmError::StackUnderflow)?;
    stack.push(op(left, right)?);
    Ok(())
}
