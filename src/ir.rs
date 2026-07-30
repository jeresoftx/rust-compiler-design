//! Lowering lineal desde el AST validado.

use crate::{
    ast::{AstExpression, AstProgram, AstStatement},
    parser::BinaryOperator,
};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct ValueId(pub usize);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Const {
        destination: ValueId,
        value: i64,
    },
    Load {
        destination: ValueId,
        name: String,
    },
    Store {
        name: String,
        value: ValueId,
    },
    Binary {
        destination: ValueId,
        operator: BinaryOperator,
        left: ValueId,
        right: ValueId,
    },
    Return(ValueId),
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct IrProgram {
    pub instructions: Vec<Instruction>,
}

#[must_use]
pub fn lower(program: &AstProgram) -> IrProgram {
    let mut lowerer = Lowerer {
        next: 0,
        instructions: Vec::new(),
    };
    for statement in &program.statements {
        match statement {
            AstStatement::Let { name, value, .. } => {
                let value = lowerer.expression(value);
                lowerer.instructions.push(Instruction::Store {
                    name: name.clone(),
                    value,
                });
            }
            AstStatement::Expression { expression, .. } => {
                let value = lowerer.expression(expression);
                lowerer.instructions.push(Instruction::Return(value));
            }
        }
    }
    IrProgram {
        instructions: lowerer.instructions,
    }
}

struct Lowerer {
    next: usize,
    instructions: Vec<Instruction>,
}
impl Lowerer {
    fn value(&mut self) -> ValueId {
        let id = ValueId(self.next);
        self.next += 1;
        id
    }
    fn expression(&mut self, expression: &AstExpression) -> ValueId {
        match expression {
            AstExpression::Integer { value, .. } => {
                let destination = self.value();
                self.instructions.push(Instruction::Const {
                    destination,
                    value: *value,
                });
                destination
            }
            AstExpression::Name { value, .. } => {
                let destination = self.value();
                self.instructions.push(Instruction::Load {
                    destination,
                    name: value.clone(),
                });
                destination
            }
            AstExpression::Prefix { operand, .. } => {
                let zero = self.value();
                self.instructions.push(Instruction::Const {
                    destination: zero,
                    value: 0,
                });
                let right = self.expression(operand);
                let destination = self.value();
                self.instructions.push(Instruction::Binary {
                    destination,
                    operator: BinaryOperator::Subtract,
                    left: zero,
                    right,
                });
                destination
            }
            AstExpression::Binary {
                left,
                operator,
                right,
                ..
            } => {
                let left = self.expression(left);
                let right = self.expression(right);
                let destination = self.value();
                self.instructions.push(Instruction::Binary {
                    destination,
                    operator: *operator,
                    left,
                    right,
                });
                destination
            }
        }
    }
}
