//! Generación de instrucciones de pila desde la IR lineal.
use crate::{
    ir::{Instruction, IrProgram},
    parser::BinaryOperator,
};
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum OpCode {
    PushConstant(i64),
    Load(String),
    Store(String),
    Add,
    Subtract,
    Multiply,
    Divide,
    Return,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Bytecode {
    pub instructions: Vec<OpCode>,
}
#[must_use]
pub fn generate(program: &IrProgram) -> Bytecode {
    let mut instructions = Vec::new();
    for instruction in &program.instructions {
        match instruction {
            Instruction::Const { value, .. } => instructions.push(OpCode::PushConstant(*value)),
            Instruction::Load { name, .. } => instructions.push(OpCode::Load(name.clone())),
            Instruction::Store { name, .. } => instructions.push(OpCode::Store(name.clone())),
            Instruction::Binary { operator, .. } => instructions.push(match operator {
                BinaryOperator::Add => OpCode::Add,
                BinaryOperator::Subtract => OpCode::Subtract,
                BinaryOperator::Multiply => OpCode::Multiply,
                BinaryOperator::Divide => OpCode::Divide,
            }),
            Instruction::Return(_) => instructions.push(OpCode::Return),
        }
    }
    Bytecode { instructions }
}
