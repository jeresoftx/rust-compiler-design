//! Pase local de folding de constantes para la IR educativa.

use std::collections::HashMap;

use crate::{
    ir::{Instruction, IrProgram},
    parser::BinaryOperator,
};

#[must_use]
pub fn fold_constants(program: &IrProgram) -> IrProgram {
    let mut constants = HashMap::new();
    let mut instructions = Vec::new();
    for instruction in &program.instructions {
        match instruction {
            Instruction::Const { destination, value } => {
                constants.insert(*destination, *value);
                instructions.push(instruction.clone());
            }
            Instruction::Binary {
                destination,
                operator,
                left,
                right,
            } => {
                let folded = constants
                    .get(left)
                    .zip(constants.get(right))
                    .and_then(|(left, right)| evaluate(*operator, *left, *right));
                if let Some(value) = folded {
                    constants.insert(*destination, value);
                    instructions.push(Instruction::Const {
                        destination: *destination,
                        value,
                    });
                } else {
                    constants.remove(destination);
                    instructions.push(instruction.clone());
                }
            }
            Instruction::Load { destination, .. } => {
                constants.remove(destination);
                instructions.push(instruction.clone());
            }
            _ => instructions.push(instruction.clone()),
        }
    }
    IrProgram { instructions }
}

fn evaluate(operator: BinaryOperator, left: i64, right: i64) -> Option<i64> {
    match operator {
        BinaryOperator::Add => Some(left + right),
        BinaryOperator::Subtract => Some(left - right),
        BinaryOperator::Multiply => Some(left * right),
        BinaryOperator::Divide => (right != 0).then_some(left / right),
    }
}
